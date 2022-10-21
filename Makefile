CC=cc
APPLE_TARGETS=aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-darwin x86_64-apple-ios
HEADER_FILE=./crates/abi/include/yxy.h

.PHONY: core abi cli all apple clean-xcf xcf clean cbindgen ctest

core:
	@echo "Building core lib..."
	@cargo build -p yxy

abi:
	@echo "Building cdylib & staticlib..."
	@cargo build -p yxy-abi

cli:
	@echo "Building CLI..."
	@cargo build -p yxy-cli

all:
	@echo "Building all..."
	@cargo build

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
		cargo build -p yxy-abi --target $$target --release; \
	done

cbindgen:
	@echo "Generating C bindings..."
	@rustup run nightly cbindgen --config ./crates/abi/cbindgen.toml --crate yxy-abi --output $(HEADER_FILE)
	@echo "Generate successfully out to: $(HEADER_FILE)"

xcf: apple cbindgen clean-xcf
	@echo "Building XCFramework..."
	
	@mkdir -p target/universal/release
	@lipo -create \
			target/x86_64-apple-darwin/release/libyxy_abi.a \
			target/aarch64-apple-darwin/release/libyxy_abi.a \
		-output target/universal/release/libyxy_macos.a

	@lipo -create \
			target/aarch64-apple-ios-sim/release/libyxy_abi.a \
			target/x86_64-apple-ios/release/libyxy_abi.a \
		-output target/universal/release/libyxy_iossim.a
	

	@xcodebuild -create-xcframework \
		-library ./target/universal/release/libyxy_macos.a -headers ./crates/abi/include/ \
		-library ./target/universal/release/libyxy_iossim.a -headers ./crates/abi/include/ \
		-library ./target/aarch64-apple-ios/release/libyxy_abi.a -headers ./crates/abi/include/ \
		-output target/universal/yxy-static.xcframework

ctest: abi
	@echo "Building C test..."
	@mkdir -p ./target/tests
	@rustup run nightly cbindgen --crate yxy-abi -c ./crates/abi/cbindgen.toml --output ./tests/yxy.h 
	@$(CC) -l yxy_abi -L ./target/debug -o ./target/debug/main ./tests/main.c
	@echo "Runing tests..."
	@./target/debug/main
