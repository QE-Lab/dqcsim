
ifndef DQCSIM_HOME
$(error $$DQCSIM_HOME is not set)
endif

PY_SOURCES = $(shell find py/src -name '*.py')

.SUFFIXES:
.NOTPARALLEL:

.PHONY: all
all: build

.PHONY: cargo-build
cargo-build:
	cargo build

.PHONY: cargo-release
cargo-release:
	cargo build --release

.PHONY: cargo-release
cargo-install:
	mkdir -p $(DQCSIM_HOME)/lib
	cp -t $(DQCSIM_HOME)/lib ../../../target/release/libdqcshost.so
	mkdir -p $(DQCSIM_HOME)/bin
	ln -fs ../lib/libdqcshost.so $(DQCSIM_HOME)/bin/libdqcshost.so

py/gen/dqcshost.c: dqcshost.i c/gen/dqcshost.h
	mkdir -p py/gen
	swig -python -py3 -outdir py/gen -o py/gen/dqcshost.c $<
	rm -f py/gen/dqcshost.py

.PHONY: py-build
py-build: py/gen/dqcshost.c c/gen/dqcshost.h py/setup.py $(PY_SOURCES)
	cd py && python3 setup.py build -b gen/debug/build

.PHONY: py-release
py-release: py/gen/dqcshost.c c/gen/dqcshost.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 python3 setup.py build -b gen/release/build

.PHONY: py-install
py-install: py/gen/dqcshost.c c/gen/dqcshost.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 python3 setup.py build -b gen/release/build install --user

.PHONY: build
build: cargo-build py-build

.PHONY: release
release: cargo-release py-release

.PHONY: install
install: cargo-release cargo-install py-install

.PHONY: clean
clean:
	rm -rf gen
	cargo clean

