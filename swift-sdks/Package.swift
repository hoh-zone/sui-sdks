// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "SuiSwiftSDK",
    platforms: [
        .macOS(.v13),
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "SuiSwiftSDK",
            targets: ["SuiSwiftSDK"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-crypto.git", from: "2.0.0"),
        .package(url: "https://github.com/21-DOT-DEV/swift-secp256k1", exact: "0.21.1")
    ],
    targets: [
        .target(
            name: "SuiSwiftSDK",
            dependencies: [
                .product(name: "Crypto", package: "swift-crypto"),
                .product(name: "P256K", package: "swift-secp256k1")
            ]
        ),
        .testTarget(
            name: "SuiSwiftSDKTests",
            dependencies: ["SuiSwiftSDK"]
        )
    ]
)
