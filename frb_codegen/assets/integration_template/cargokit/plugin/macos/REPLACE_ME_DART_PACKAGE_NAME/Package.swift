// swift-tools-version: 5.9
import Foundation
import PackageDescription

let packageDirectory = URL(fileURLWithPath: #filePath)
    .deletingLastPathComponent()
    .resolvingSymlinksInPath()

func stablePathHash(_ value: String) -> String {
    var hash: UInt64 = 14_695_981_039_346_656_037
    for byte in value.utf8 {
        hash ^= UInt64(byte)
        hash &*= 1_099_511_628_211
    }
    return String(format: "%016llx", hash)
}

let archiveName = "lib" + "REPLACE_ME_RUST_CRATE_NAME"
    .replacingOccurrences(of: "-", with: "_") + ".a"
let archivePath = "/tmp/cargokit-spm/" +
    stablePathHash(packageDirectory.path) + "/" + archiveName
let archiveDirectory = URL(fileURLWithPath: archivePath)
    .deletingLastPathComponent().path
let archiveLibraryName = "REPLACE_ME_RUST_CRATE_NAME"
    .replacingOccurrences(of: "-", with: "_")

let package = Package(
    name: "REPLACE_ME_DART_PACKAGE_NAME",
    platforms: [.macOS(.v10_15)],
    products: [
        .library(
            name: "REPLACE_ME_DART_PACKAGE_HYPHENATED_NAME",
            targets: ["REPLACE_ME_DART_PACKAGE_NAME"]
        ),
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework"),
    ],
    targets: [
        .target(
            name: "REPLACE_ME_DART_PACKAGE_NAME",
            dependencies: [
                "REPLACE_ME_DART_PACKAGE_NAME_CargoKitLinker",
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
        .target(
            name: "REPLACE_ME_DART_PACKAGE_NAME_CargoKitLinker",
            path: "Sources/CargoKitLinker",
            linkerSettings: [
                .unsafeFlags([
                    "-L", archiveDirectory,
                    "-Xlinker", "-u",
                    "-Xlinker", "_frbgen_REPLACE_ME_DART_PACKAGE_NAME_link_anchor",
                    "-l" + archiveLibraryName,
                ]),
            ],
            plugins: ["REPLACE_ME_DART_PACKAGE_NAME_CargoKitPlugin"]
        ),
        .plugin(
            name: "REPLACE_ME_DART_PACKAGE_NAME_CargoKitPlugin",
            capability: .buildTool(),
            path: "Plugins/CargoKitPlugin"
        ),
    ]
)
