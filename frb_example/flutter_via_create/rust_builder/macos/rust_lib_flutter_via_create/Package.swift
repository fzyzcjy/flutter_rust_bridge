// swift-tools-version: 5.9

import PackageDescription

let productName = "rust_lib_flutter_via_create"
    .split(separator: "_")
    .joined(separator: "-")

let package = Package(
    name: "rust_lib_flutter_via_create",
    platforms: [
        .macOS("10.15"),
    ],
    products: [
        .library(
            name: productName,
            targets: ["rust_lib_flutter_via_create"]
        ),
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework"),
    ],
    targets: [
        .target(
            name: "rust_lib_flutter_via_create",
            dependencies: [
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
