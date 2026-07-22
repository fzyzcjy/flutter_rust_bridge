// swift-tools-version: 5.9

import PackageDescription

let productName = "rust_lib_flutter_via_integrate"
    .split(separator: "_")
    .joined(separator: "-")

let package = Package(
    name: "rust_lib_flutter_via_integrate",
    platforms: [
        .iOS("13.0"),
    ],
    products: [
        .library(
            name: productName,
            targets: ["rust_lib_flutter_via_integrate"]
        ),
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework"),
    ],
    targets: [
        .target(
            name: "rust_lib_flutter_via_integrate",
            dependencies: [
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
