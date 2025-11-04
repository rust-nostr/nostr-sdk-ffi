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

[linux]
python:
	rm -rf python/dist
	pip install -r python/requirements.txt
	cargo build --lib
	cargo run --bin uniffi-bindgen generate --library target/debug/libnostr_sdk_ffi.so --language python --no-format -o python/src/nostr-sdk/
	cargo build --lib --release
	cp target/release/libnostr_sdk_ffi.so python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[macos]
python:
	rm -rf python/dist
	pip install -r python/requirements.txt
	cargo build --lib
	cargo run --bin uniffi-bindgen generate --library target/debug/libnostr_sdk_ffi.dylib --language python --no-format -o python/src/nostr-sdk/
	cargo build --lib --release
	cp target/release/libnostr_sdk_ffi.dylib python/src/nostr-sdk/
	cd python && python setup.py bdist_wheel
	pip install ./python/dist/nostr_sdk*.whl --force-reinstall

[windows]
python:
	pip install -r python\requirements.txt
	cargo build --lib
	cargo run --bin uniffi-bindgen generate --library target\debug\nostr_sdk_ffi.dll --language python --no-format -o python\src\nostr-sdk\
	cargo build --lib --release
	copy target\release\nostr_sdk_ffi.dll python\src\nostr-sdk
	del /F /Q python\dist\* 2>nul || exit /b 0
	cd python && python setup.py bdist_wheel
	FOR %%i in (.\python\dist\*.whl) DO pip install %i --force-reinstall
