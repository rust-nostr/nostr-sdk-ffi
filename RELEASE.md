# Release steps

## Prerequisites

- Rust
- Python and `twine` (can be installed with `pip install twine`)
- Docker

## Bump the versions

- Android in `android/lib/build.gradle.kts`
- JVM in `jvm/lib/build.gradle.kts`
- C# in `csharp/Nostr.Sdk/Nostr.Sdk.csproj`
- Python in `python/pyproject.toml`
- Swift Package **DOESN'T** require version update

## Update the changelog

Go to the CHANGELOG.md, update the header and other stuff if needed

## Commit, tag and push

Commit the changelog changes with the bump of the libraries versions, tag the commit and push to the repo

## Build and publish bindings

- Swift package is built and released with GitHub Actions, so is enough to run the workflow
- The other bindings are compiled and published manually from a local computer:
    - Run `just build` or `bash scripts/all.sh` to build all binaries
    - Run `just publish-aar` to publish the Android library
    - Run `just publish-jar` to publish the JVM library   
    - Run `just publish-py` to publish the Python Wheels library
    - Run `just csharp` to pack the C# library
        - The C# packaged library will be located in `csharp/Nostr.Sdk/bin/Release/Nostr.Sdk.<version>.nupkg`
        - Go to <nuget.org> and upload the `.nupkg` file

## Bump supported platforms

In the book, if needed, bump the supported platforms.
