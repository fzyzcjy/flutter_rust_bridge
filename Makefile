

# install local flutter_rust_bridge_codegen
install.frb:
	cargo install flutter_rust_bridge_codegen --path frb_codegen/ --all-features

# rebuild all and run
farsh: 
	@make codegen
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

	cd frb_example/pure_dart/dart \
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
	&& dart run ./lib/main.dart "/media/human/DE2466F72466D1D7/Work/Github/Test/flutter_rust_bridge/target/debug/libflutter_rust_bridge_example.so"

# build rust example lib
rust.build: 
	cd frb_example/pure_dart/rust \
	&& cargo build 

so.fn:
	cd frb_example/pure_dart/rust/target/debug \
	&& nm -D libflutter_rust_bridge_example.so