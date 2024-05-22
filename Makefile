release:
	cargo build -r
	mkdir -p bin
	cp target/release/buy_low bin

.phony: release
