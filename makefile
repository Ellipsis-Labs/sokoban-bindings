.PHONY: default clean mock

default: build

build:
	@echo "\n--- Building library... ---\n"
	RUSTFLAGS="-Ctarget-cpu=native" cargo +nightly build --release
	@echo "\n--- Library built at target/release ---\n"

	@echo "\n--- Building example usage ---\n"
	gcc -o examples/c/sokoban.out examples/c/sokoban.c -L./target/release/ -lsokoban_bindgen_example -O3 -march=native
	@echo "\n--- Executable built at examples/c/sokoban.out ---\n"

clean:
	@echo "\n--- Cleaning project... ---\n"
	cargo clean
	rm examples/c/sokoban.out
	@echo "\n--- Project cleaned ---\n"
