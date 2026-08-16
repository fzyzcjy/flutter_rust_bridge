// swift-tools-version: 5.9
// Before building with SwiftPM, run from rust_builder:
// sh cargokit/build_spm.sh ../REPLACE_ME_RUST_CRATE_DIR ios/REPLACE_ME_RUST_CRATE_NAME release
import PackageDescription

let package = Package(
    name: "REPLACE_ME_RUST_CRATE_NAME",
    platforms: [.iOS(.v13)],
    products: [
        .library(
            name: "REPLACE_ME_RUST_CRATE_HYPHENATED_NAME",
            targets: ["REPLACE_ME_RUST_CRATE_NAME"]
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
            name: "REPLACE_ME_RUST_CRATE_NAME",
            dependencies: [
                "RustLibrary",
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
