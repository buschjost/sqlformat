.PHONY : build release

build:
	cargo build --release  --target=aarch64-apple-darwin
	cargo build --release  --target=x86_64-apple-darwin
	mkdir -p target/universal
	lipo -create -output target/universal/sqlformat target/aarch64-apple-darwin/release/sqlformat target/x86_64-apple-darwin/release/sqlformat

release: build
	VERSION=$(shell cat Cargo.toml | grep version | cut -d'"' -f2)
	mkdir -p release/${VERSION}
	cp target/x86_64-apple-darwin/release/sqlformat release/${VERSION}/sqlformat-darwin-x86_64
	cp target/aarch64-apple-darwin/release/sqlformat release/${VERSION}/sqlformat-darwin-arm64
	cp target/universal/sqlformat release/${VERSION}/sqlformat-darwin-universal
	cd release/${VERSION}; shasum -a 256 sqlformat-* > SHA256SUMS; cd -
