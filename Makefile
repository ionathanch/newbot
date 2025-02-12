lambda.zip: target/x86_64-unknown-linux-musl/release/newbot
	cp $< bootstrap
	strip bootstrap
	ls -l bootstrap
	rm -f lambda.zip
	zip lambda.zip bootstrap
	rm -f bootstrap

newbot:
	cargo build --bin newbot --no-default-features --release

new:
	cargo run --release newbot

.PHONY: target/x86_64-unknown-linux-musl/release/newbot
target/x86_64-unknown-linux-musl/release/newbot:
	cargo build --bin newbot --no-default-features --release --target x86_64-unknown-linux-musl

.PHONY: clean
clean:
	rm -f lambda.zip
	cargo clean
