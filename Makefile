NAME=$(notdir $(CURDIR))

build:
	cargo fmt
	cargo build --release

VERSION=$(shell cargo run --quiet --bin getversion)
dist:
	zip -j findrun-$(VERSION)-windows-amd64.zip target/release/findrun.exe

manifest:
	make-scoop-manifest *-windows-*.zip > $(NAME).json
