build:
	cargo build --release  --target=aarch64-apple-darwin
	cargo build --release  --target=x86_64-apple-darwin
	mkdir -p target/universal
	lipo -create -output target/universal/sqlformat target/aarch64-apple-darwin/release/sqlformat target/x86_64-apple-darwin/release/sqlformat

