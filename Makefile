# install local flutter_rust_bridge_codegen
install.frb:
	cargo install flutter_rust_bridge_codegen --path frb_codegen/ --all-features

# rebuild all and run
farsh: 
# @make codegen
	@make rust.build
	@make dart.run

rebuild: 
	@make codegen
	@make rust.build

# codegen
codegen:
	flutter_rust_bridge_codegen \
		--rust-input=frb_example/pure_dart/rust/src/api.rs \
		--dart-output=frb_example/pure_dart/dart/lib/bridge_generated.dart \
		--dart-format-line-length 120 

	cd frb_example/pure_dart/dart \
	&& flutter pub get \
	&& flutter pub run build_runner build --delete-conflicting-outputs

flutter.codegen:
	flutter_rust_bridge_codegen \
		--rust-input=frb_example/with_flutter/rust/src/api.rs \
		--dart-output=frb_example/with_flutter/lib/bridge_generated.dart \
		--dart-format-line-length 120 

	cd frb_example/with_flutter/ \
	&& flutter pub get \
	&& flutter pub run build_runner build --delete-conflicting-outputs


dart.get: 
	cd frb_example/pure_dart/dart \
	&& flutter pub get
	cd frb_dart/ \
	&& flutter pub get \

# run dart test
dart.run: 
	cd frb_example/pure_dart/dart \
	&& dart run ./lib/main.dart "/media/human/DE2466F72466D1D73/Work/Github/Test/flutter_rust_bridge/target/debug/libflutter_rust_bridge_example.so"

# build dart web test
dart.build.web: 
	cd frb_example/pure_dart/dart \
	&& flutter build web

flutter.deps: 
	cd frb_example/pure_dart/dart \
	&& dart run flutter_rust_bridge:serve ./lib/main.dart
# && flutter pub get

# build rust example lib
rust.build: 
	cd frb_example/pure_dart/rust \
	&& cargo build 

so.fn:
	cd frb_example/pure_dart/rust/target/debug \
	&& nm -D libflutter_rust_bridge_example.so

web.farsh:
	@make wasm.build 
	@mkdir -p frb_example/pure_dart/dart/build/web/pkg/
	cp -f frb_example/pure_dart/rust/lib/flutter_rust_bridge_example_bg.wasm \
		frb_example/pure_dart/dart/build/web/pkg/flutter_rust_bridge_example_bg.wasm
	cp -f frb_example/pure_dart/rust/lib/flutter_rust_bridge_example.js \
		frb_example/pure_dart/dart/build/web/pkg/flutter_rust_bridge_example.js
	@make dart.build.web

wasm.build:
	export RUSTUP_TOOLCHAIN=nightly && \
	export RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals" && \
	wasm-pack build \
		frb_example/pure_dart/rust/ \
		-t no-modules \
		-d lib/ \
		--no-typescript -- \
		--target wasm32-unknown-unknown -Z build-std=std,panic_abort

dart.fmt:
	dart format --line-length 80 .

fmt: 
	cargo fmt --all
	dart format --line-length 80 ./frb_dart
	dart format --line-length 120 ./frb_example