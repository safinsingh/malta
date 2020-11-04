DEFAULT_GOAL := dev

dev:
	cd helios && \
	cargo build && \
	cp target/debug/helios ../dist/dev/helios && \
	cd ../ares && \
	cargo build && \
	cp target/debug/ares ../dist/dev/ares

release:
	cd helios && \
	cargo build --release && \
	cp target/release/helios ../dist/helios && \
	cd ../ares && \
	cargo build --release && \
	cp target/release/ares ../dist/ares
