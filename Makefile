# Variables [cite: 28]
BINARY_NAME = ricecooker
TARGETS = aarch64-linux-android aarch64-pc-windows-msvc aarch64-unknown-linux-gnu armv7-linux-androideabi i686-linux-android wasm32-unknown-unknown x86_64-linux-android x86_64-pc-windows-msvc x86_64-unknown-linux-gnu x86_64-unknown-linux-musl
BUILD_DIR = target/release

# Default action: build for the host machine [cite: 28]
all: $(TARGETS)

## Pattern rule for targets [cite: 28]
$(TARGETS):
	@echo "Building for $@..."
	@if echo "$@" | grep -q "msvc"; then \
		cargo xwin build --release --target $@; \
	else \
		cargo build --release --target $@; \
	fi

## Build for a specific os and/or group
desktop: $(filter %-linux-gnu %-linux-musl %-windows-msvc, $(TARGETS))
linux: $(filter %-linux-gnu %-linux-musl, $(TARGETS))
android: $(filter %-android %-androideabi, $(TARGETS))
windows: $(filter %-windows-msvc, $(TARGETS))

## Clean build artifacts [cite: 28]
clean:
	cargo clean

## Help command to see available options [cite: 28]
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build-all    Build for all platforms ($(TARGETS))"
	@echo "  clean        Remove build artifacts"
	@echo "  help         Show this help message"

.PHONY: all build-all clean help $(TARGETS)