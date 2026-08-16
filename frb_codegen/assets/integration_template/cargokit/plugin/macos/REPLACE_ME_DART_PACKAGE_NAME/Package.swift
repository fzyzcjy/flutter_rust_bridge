// swift-tools-version: 5.9
import Foundation
import PackageDescription

let packageDirectory = URL(fileURLWithPath: #filePath)
    .deletingLastPathComponent()
    .resolvingSymlinksInPath()
let dartPackageDirectory = packageDirectory
    .deletingLastPathComponent()
    .deletingLastPathComponent()
let environment = ProcessInfo.processInfo.environment
let requestedConfiguration = (
    environment["CARGOKIT_CONFIGURATION"] ??
        environment["CONFIGURATION"] ??
        "release"
).lowercased()
let cargokitConfiguration = requestedConfiguration.contains("debug") ? "debug" : "release"

func buildRustLibrary() {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/bin/sh")
    process.arguments = [
        dartPackageDirectory.appendingPathComponent("cargokit/build_spm.sh").path,
        dartPackageDirectory.appendingPathComponent("REPLACE_ME_RUST_CRATE_DIR").path,
        packageDirectory.path,
        cargokitConfiguration,
    ]
    process.currentDirectoryURL = dartPackageDirectory

    do {
        try process.run()
    } catch {
        fatalError("Failed to start Cargokit SwiftPM build: \(error)")
    }
    process.waitUntilExit()
    guard process.terminationStatus == 0 else {
        fatalError("Cargokit SwiftPM build failed with exit code \(process.terminationStatus)")
    }
}

buildRustLibrary()

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
        .binaryTarget(
            name: "RustLibrary",
            path: "REPLACE_ME_RUST_CRATE_NAME.xcframework"
        ),
        .target(
            name: "REPLACE_ME_DART_PACKAGE_NAME",
            dependencies: [
                "RustLibrary",
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
