// swift-tools-version: 5.9

import PackageDescription

let productName = "flutter_package"
    .split(separator: "_")
    .joined(separator: "-")

let package = Package(
    name: "flutter_package",
    platforms: [
        .iOS("13.0"),
    ],
    products: [
        .library(
            name: productName,
            targets: ["flutter_package"]
        ),
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework"),
    ],
    targets: [
        .target(
            name: "flutter_package",
            dependencies: [
                .product(name: "FlutterFramework", package: "FlutterFramework"),
            ]
        ),
    ]
)
