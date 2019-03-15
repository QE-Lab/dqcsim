.SUFFIXES:

.PHONY: all
all: build

.PHONY: cargo-build
cargo-build:
	cargo build

.PHONY: cargo-release
cargo-release:
	cargo build --release

gen/py/dqcshost.c: dqcshost.i gen/c/dqcshost.h
	mkdir -p gen/py
	swig -python -py3 -outdir gen/py -o gen/py/dqcshost.c $<
	rm -f gen/py/dqcshost.py

.PHONY: py-build
py-build: gen/py/dqcshost.c gen/c/dqcshost.h setup.py
	python3 setup.py build -b gen/py/debug/build install --prefix gen/py/debug/install

.PHONY: py-release
py-release: gen/py/dqcshost.c gen/c/dqcshost.h setup.py
	DQCSIM_RUST_RELEASE=1 python3 setup.py build -b gen/py/release/build install --prefix gen/py/release/install

.PHONY: build
build: cargo-build py-build

.PHONY: release
release: cargo-release py-release

.PHONY: clean
clean:
	rm -rf gen
	cargo clean

