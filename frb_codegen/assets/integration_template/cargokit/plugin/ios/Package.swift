// swift-tools-version: 5.9
// Before building with SwiftPM, run from the package root:
// sh cargokit/build_spm.sh rust ios release
import PackageDescription

let package = Package(
    name: "REPLACE_ME_DART_PACKAGE_NAME",
    platforms: [.iOS(.v13)],
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
