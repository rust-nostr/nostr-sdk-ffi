// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "nostr-sdk-swift",
    platforms: [
        .macOS(.v12),
        .iOS(.v14),
    ],
    swiftLanguageModes: [.v5], // UniFFI doesn't fully support Swift 6 yet
    products: [
        .library(name: "NostrSDK", targets: ["nostr_sdkFFI", "NostrSDK"]),
    ],
    dependencies: [],
    targets: [
        .binaryTarget(name: "nostr_sdkFFI", path: "./nostr_sdkFFI.xcframework"),
        .target(name: "NostrSDK", dependencies: ["nostr_sdkFFI"]),
        .testTarget(name: "NostrSDKTests", dependencies: ["NostrSDK"]),
    ]
)
