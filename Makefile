.PHONY: build clean

build:
	cd rust && cargo build
	mkdir -p godot/addons/kitn/bin/macos
	mkdir -p godot/addons/kitn/bin/linux
	mkdir -p godot/addons/kitn/bin/windows
	# Copy built binaries to Godot project (example for local dev on execution platform)
	# User will need to manually invoke for their platform or we can detect
	cp rust/target/debug/libkitn.dylib godot/addons/kitn/bin/macos/ || true
	# cp rust/target/debug/libkitn.so godot/addons/kitn/bin/linux/ || true
	# cp rust/target/debug/kitn.dll godot/addons/kitn/bin/windows/ || true

clean:
	cd rust && cargo clean
	rm -rf godot/addons/kitn/bin/*
