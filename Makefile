CC=cc
APPLE_TARGETS=aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-darwin x86_64-apple-ios
HEADER_FILE=./crates/ffi/include/yxy.h

.PHONY: core ffi cli all apple clean-xcf xcf clean cbindgen ctest httpd server

core:
	@echo "Building core lib..."
	@cargo build -p yxy

ffi:
	@echo "Building cdylib & staticlib..."
	@cargo build -p yxy-ffi --all-features

cli:
	@echo "Building CLI..."
	@cargo build -p yxy-cli

httpd:
	@echo "Building HTTPd..."
	@cargo build -p yxy-httpd

all:
	@echo "Building all..."
	@cargo build

server:
	@echo "Starting HTTPd..."
	@cargo run --bin yxy-httpd

test:
	@echo "Testing..."
	@cargo test
	@cargo clippy

clean:
	@echo "Cleaning releases..."
	@rm -rf ./target/aarch64-apple-ios
	@rm -rf ./target/aarch64-apple-ios-sim
	@rm -rf ./target/aarch64-apple-darwin
	@rm -rf ./target/x86_64-apple-ios
	@rm -rf ./target/x86_64-apple-darwin
	@rm -rf ./target/package
	@rm -rf ./target/universal
	@rm -rf ./target/release

clean-all:
	@echo "Cleaning all..."
	@cargo clean

clean-xcf:
	@echo "Cleaning XCFramework..."
	@rm -rf target/universal/release
	@rm -rf ./target/universal/yxy-static.xcframework

apple:
	@echo "Building Apple targets..."
	@for target in $(APPLE_TARGETS); do \
		cargo build -p yxy-ffi --all-features --target $$target --release; \
	done

cbindgen:
	@echo "Generating C bindings..."
	@rustup run nightly cbindgen --config ./crates/ffi/cbindgen.toml --crate yxy-ffi --output $(HEADER_FILE)
	@echo "Generate successfully out to: $(HEADER_FILE)"

xcf: apple cbindgen clean-xcf
	@echo "Building XCFramework..."
	
	@mkdir -p target/universal/release
	@lipo -create \
			target/x86_64-apple-darwin/release/libyxy_ffi.a \
			target/aarch64-apple-darwin/release/libyxy_ffi.a \
		-output target/universal/release/libyxy_macos.a

	@lipo -create \
			target/aarch64-apple-ios-sim/release/libyxy_ffi.a \
			target/x86_64-apple-ios/release/libyxy_ffi.a \
		-output target/universal/release/libyxy_iossim.a
	

	@xcodebuild -create-xcframework \
		-library ./target/universal/release/libyxy_macos.a -headers ./crates/ffi/include/ \
		-library ./target/universal/release/libyxy_iossim.a -headers ./crates/ffi/include/ \
		-library ./target/aarch64-apple-ios/release/libyxy_ffi.a -headers ./crates/ffi/include/ \
		-output target/universal/yxy-static.xcframework

ctest: ffi
	@echo "Building C test..."
	@mkdir -p ./target/tests
	@rustup run nightly cbindgen --crate yxy-ffi -c ./crates/ffi/cbindgen.toml --output ./tests/yxy.h 
	@$(CC) -l yxy_ffi -L ./target/debug -o ./target/debug/main ./tests/main.c
	@echo "Runing tests..."
	@./target/debug/main
