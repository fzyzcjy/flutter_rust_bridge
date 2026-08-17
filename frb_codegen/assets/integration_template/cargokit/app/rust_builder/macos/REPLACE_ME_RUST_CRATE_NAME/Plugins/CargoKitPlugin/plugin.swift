import Foundation
import PackagePlugin

@main
struct CargoKitPlugin: BuildToolPlugin {
    func createBuildCommands(
        context: PluginContext,
        target: Target
    ) throws -> [Command] {
        let packageDirectory = URL(
            fileURLWithPath: context.package.directory.string
        ).resolvingSymlinksInPath()
        let dartPackageDirectory = packageDirectory
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        let manifestDirectory = dartPackageDirectory
            .appendingPathComponent("REPLACE_ME_RUST_CRATE_DIR")
        let cargokitDirectory = packageDirectory.appendingPathComponent("cargokit")
        let script = cargokitDirectory.appendingPathComponent("build_spm.sh")
        let outerBuildToolDirectory = dartPackageDirectory
            .appendingPathComponent("rust_builder/cargokit/build_tool")
        let dartPackageConfig = outerBuildToolDirectory
            .appendingPathComponent(".dart_tool/package_config.json")
        let flutterRoot = try flutterRoot(in: dartPackageDirectory)
        let archiveName = "lib" + "REPLACE_ME_RUST_CRATE_NAME"
            .replacingOccurrences(of: "-", with: "_") + ".a"
        let archivePath = "/tmp/cargokit-spm/" +
            stablePathHash(packageDirectory.path) + "/" + archiveName
        let generatedSource = context.pluginWorkDirectory
            .appending("CargoKitGenerated.c")
        let buildWorkDirectory = context.pluginWorkDirectory
            .appending("cargokit")

        let inputs = inputFiles(in: manifestDirectory) +
            inputFiles(in: cargokitDirectory) +
            inputFiles(in: outerBuildToolDirectory) +
            [Path(dartPackageConfig.path)]

        return [
            .buildCommand(
                displayName: "Building REPLACE_ME_RUST_CRATE_NAME with CargoKit",
                executable: Path("/bin/bash"),
                arguments: [
                    "-c",
                    #"""
                    CARGOKIT_DART_PACKAGE_CONFIG="$7" FLUTTER_ROOT="$6" \
                    "$1" "$2" "$3" "$4" &&
                    printf 'static void cargokit_generated_anchor(void) {}\n' > "$5"
                    """#,
                    "cargokit",
                    script.path,
                    manifestDirectory.path,
                    archivePath,
                    buildWorkDirectory.string,
                    generatedSource.string,
                    flutterRoot,
                    dartPackageConfig.path,
                ],
                inputFiles: inputs,
                outputFiles: [generatedSource]
            ),
        ]
    }

    private func stablePathHash(_ value: String) -> String {
        var hash: UInt64 = 14_695_981_039_346_656_037
        for byte in value.utf8 {
            hash ^= UInt64(byte)
            hash &*= 1_099_511_628_211
        }
        return String(format: "%016llx", hash)
    }

    private func flutterRoot(in dartPackageDirectory: URL) throws -> String {
        let packageConfig = dartPackageDirectory
            .appendingPathComponent(".dart_tool/package_config.json")
        let data = try Data(contentsOf: packageConfig)
        let json = try JSONSerialization.jsonObject(with: data)
        guard
            let object = json as? [String: Any],
            let packages = object["packages"] as? [[String: Any]],
            let flutter = packages.first(where: { $0["name"] as? String == "flutter" }),
            let rootUri = flutter["rootUri"] as? String,
            let flutterPackage = URL(string: rootUri, relativeTo: packageConfig)
        else {
            throw PluginError.flutterSdkNotFound(packageConfig.path)
        }
        return flutterPackage.standardizedFileURL
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .path
    }

    private func inputFiles(in root: URL) -> [Path] {
        let keys: [URLResourceKey] = [.isDirectoryKey, .isRegularFileKey]
        guard let enumerator = FileManager.default.enumerator(
            at: root,
            includingPropertiesForKeys: keys,
            options: [.skipsHiddenFiles]
        ) else {
            return []
        }

        var result: [Path] = []
        for case let fileURL as URL in enumerator {
            let values = try? fileURL.resourceValues(forKeys: Set(keys))
            if values?.isDirectory == true {
                if ["target", ".dart_tool"].contains(fileURL.lastPathComponent) {
                    enumerator.skipDescendants()
                }
            } else if values?.isRegularFile == true {
                result.append(Path(fileURL.path))
            }
        }
        return result
    }
}

private enum PluginError: Error, CustomStringConvertible {
    case flutterSdkNotFound(String)

    var description: String {
        switch self {
        case .flutterSdkNotFound(let packageConfig):
            return "Could not find Flutter SDK in \(packageConfig)"
        }
    }
}
