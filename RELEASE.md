# Release steps

## Bump the versions

- Android in `android/lib/build.gradle.kts`
- JVM in `jvm/lib/build.gradle.kts`
- KMP in `kmp/nostr-sdk-kmp/build.gradle.kts`
- C# in `csharp/Nostr.Sdk/Nostr.Sdk.csproj`
- Python in `python/pyproject.toml`
- Swift Package **DOESN'T** require version update

## Update the changelog

Go to the CHANGELOG.md, update the header and other stuff if needed

## Commit, tag and push

Commit the changelog changes with the bump of the libraries versions, tag the commit and push to the repo

## Build and publish bindings

Run CI actions to build and publish the bindings.

## Bump supported platforms

In the book, if needed, bump the supported platforms.
