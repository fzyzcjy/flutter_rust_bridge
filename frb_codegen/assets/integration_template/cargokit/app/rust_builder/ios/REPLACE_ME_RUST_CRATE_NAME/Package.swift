// swift-tools-version: 5.9

import PackageDescription

let productName = "REPLACE_ME_RUST_CRATE_NAME"
    .split(separator: "_")
    .joined(separator: "-")

let package = Package(
    name: "REPLACE_ME_RUST_CRATE_NAME",
    platforms: [
        .iOS("13.0"),
    ],
    products: [
        .library(
            name: productName,
            targets: ["REPLACE_ME_RUST_CRATE_NAME"]
        ),
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework"),
    ],
    targets: [
        .target(
            name: "REPLACE_ME_RUST_CRATE_NAME",
            dependencies: [
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
