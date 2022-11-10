install:
	cargo build --release
	mv target/release/vampire /usr/local/bin
