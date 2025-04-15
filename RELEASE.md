# Release steps

* Bump versions
    * Android in `android/lib/build.gradle.kts`
    * JVM in `jvm/lib/build.gradle.kts`
    * C# in `csharp/Nostr.Sdk/Nostr.Sdk.csproj`
    * Python in `python/setup.py`
    * Swift Package DOESN'T require version update

* Update the changelog

* Commit, tag and push

* Build and publish bindings
    * Python and Swift are built and released with GitHub Actions, so is enough to run the workflow
    * The other bindings are compiled and published manually from a local computer:
        * Run `just build` or `bash scripts/all.sh` to build all binaries
        * Run `just publish-aar` to publish the Android library
        * Run `just publish-jar` to publish the JVM library   
        * Run `just csharp` to pack the C# library
            * The C# packaged library will be located in `csharp/Nostr.Sdk/bin/Release/Nostr.Sdk.<version>.nupkg`
            * Go to <nuget.org> and upload the `.nupkg` file
