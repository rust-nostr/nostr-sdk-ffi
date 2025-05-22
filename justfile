set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

[private]
default:
	@just --list

[private]
bloat:
    cargo bloat --release -n 1000

# Check to perform before push a commit
precommit:
    @bash contrib/scripts/precommit.sh

# Execute a full checks
check:
    @bash contrib/scripts/check.sh

# Build all binaries (android, linux, macos and windows)
build:
    @cd scripts && bash all.sh

# Build the binaries for android
android:
    cd scripts && bash android.sh

# Build the binaries for linux
linux:
    cd scripts && bash linux.sh

# Build the binaries for macos
macos:
    cd scripts && bash macos.sh

# Build the binaries for windows
win:
    cd scripts && bash windows.sh

# Build desktop binaries (linux, macos and windows)
desktop: linux macos win

# Assemble the Android Archive (AAR)
aar:
    @cd android && bash assemble.sh

# Assemble the Java Archive (JAR)
jar:
    @cd jvm && bash assemble.sh

# Assemble the python wheels
py:
    @cd python && bash assemble.sh

# Assemble the C# package
csharp:
    @cd csharp && bash assemble.sh

# Publish AAR
[confirm]
publish-aar: aar
	cd android && ./gradlew publishAndReleaseToMavenCentral --no-configuration-cache

# Publish JAR
[confirm]
publish-jar: jar
	cd jvm && ./gradlew publishAndReleaseToMavenCentral --no-configuration-cache

# Publish Wheels
[confirm]
publish-py: py
    cd python && bash publish.sh

# Compile and build Swift Package
[macos]
swift:
    @cd swift && bash build-xcframework.sh

[linux]
python:
	rm -rf python/dist
	pip install -r python/requirements.txt
	cargo build --lib --release
	cargo run --features uniffi-cli --bin uniffi-bindgen generate --library target/release/libnostr_sdk_ffi.so --language python --no-format -o python/src/nostr-sdk/
	cp target/release/libnostr_sdk_ffi.so python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[macos]
python:
	rm -rf python/dist
	pip install -r python/requirements.txt
	cargo build --lib --release
	cargo run --features uniffi-cli --bin uniffi-bindgen generate --library target/release/libnostr_sdk_ffi.dylib --language python --no-format -o python/src/nostr-sdk/
	cp target/release/libnostr_sdk_ffi.dylib python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[windows]
python:
	pip install -r python\requirements.txt
	cargo build --lib --release
	cargo run --features uniffi-cli --bin uniffi-bindgen generate --library target\release\nostr_sdk_ffi.dll --language python --no-format -o python\src\nostr-sdk\
	copy target\release\nostr_sdk_ffi.dll python\src\nostr-sdk
	del /F /Q python\dist\* 2>nul || exit /b 0
	cd python && python setup.py bdist_wheel
	FOR %%i in (.\python\dist\*.whl) DO pip install %i --force-reinstall
