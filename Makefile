NAME=$(notdir $(CURDIR))

build:
	cargo fmt
	cargo build --release

VERSION=$(shell cargo run --quiet --bin getversion)
dist:
	zip -j findrun-$(VERSION)-windows-amd64.zip target/release/findrun.exe

release:
	pwsh -Command "latest-notes.ps1" | gh release create -d --notes-file - -t $(VERSION) $(VERSION) $(wildcard $(NAME)-$(VERSION)-*.zip)

manifest:
	make-scoop-manifest *-windows-*.zip > $(NAME).json

.PHONY: dist release build
