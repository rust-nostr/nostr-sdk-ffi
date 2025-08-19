set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

[private]
default:
	@just --list

[private]
bloat:
    cargo bloat --release -n 1000

# Format the codebase using nightly cargo
fmt:
    cargo +nightly fmt --all -- --config format_code_in_doc_comments=true

# Check the codebase for errors
check:
    cargo check --all

# Check the codebase using clippy
clippy:
    cargo clippy --all

# Run the tests for the codebase
test:
    cargo test --all

# Run all pre-commit hooks
precommit: fmt check clippy test

# Build all binaries (android, freebsd, linux, macos and windows)
build:
    @cd scripts && bash all.sh

# Build the binaries for android
android:
    cd scripts && bash android.sh

# Build the binaries for linux
linux:
    cd scripts && bash linux.sh

# Build the binaries for FreeBSD
freebsd:
    cd scripts && bash freebsd.sh

# Build the binaries for macos
macos:
    cd scripts && bash macos.sh

# Build desktop binaries (linux and macos)
desktop: linux macos

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
	cd android && ./gradlew publishToMavenCentral --no-configuration-cache

# Publish JAR
[confirm]
publish-jar: jar
	cd jvm && ./gradlew publishToMavenCentral --no-configuration-cache

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
	cargo run --bin uniffi-bindgen generate --library target/release/libnostr_sdk_ffi.so --language python --no-format -o python/src/nostr-sdk/
	cp target/release/libnostr_sdk_ffi.so python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[macos]
python:
	rm -rf python/dist
	pip install -r python/requirements.txt
	cargo build --lib --release
	cargo run --bin uniffi-bindgen generate --library target/release/libnostr_sdk_ffi.dylib --language python --no-format -o python/src/nostr-sdk/
	cp target/release/libnostr_sdk_ffi.dylib python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[windows]
python:
	pip install -r python\requirements.txt
	cargo build --lib --release
	cargo run --bin uniffi-bindgen generate --library target\release\nostr_sdk_ffi.dll --language python --no-format -o python\src\nostr-sdk\
	copy target\release\nostr_sdk_ffi.dll python\src\nostr-sdk
	del /F /Q python\dist\* 2>nul || exit /b 0
	cd python && python setup.py bdist_wheel
	FOR %%i in (.\python\dist\*.whl) DO pip install %i --force-reinstall
