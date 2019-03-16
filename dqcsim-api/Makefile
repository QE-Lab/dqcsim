
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
	cp -fp ../target/debug/libdqcsim_api.so ../target/debug/libdqcsim.so

.PHONY: cargo-release
cargo-release:
	cargo build --release
	cp -fp ../target/release/libdqcsim_api.so ../target/release/libdqcsim.so

.PHONY: cargo-release
cargo-install:
	mkdir -p $(DQCSIM_HOME)/lib
	cp -fp ../target/release/libdqcsim.so $(DQCSIM_HOME)/lib/libdqcsim.so
	mkdir -p $(DQCSIM_HOME)/include
	cp -fp c/gen/dqcsim.h $(DQCSIM_HOME)/include/dqcsim.h
	cp -fp cpp/gen/dqcsim.hpp $(DQCSIM_HOME)/include/dqcsim.hpp

py/gen/dqcsim.i: py/add_swig_directives.py py/gen/dqcsim.h
	python3 $^ $@

py/gen/dqcsim.c: py/gen/dqcsim.i py/gen/dqcsim.h
	mkdir -p py/gen
	swig -v -python -py3 -outdir py/gen -o py/gen/dqcsim.c $<
	rm -f py/gen/dqcsim.py

.PHONY: py-build
py-build: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && python3 setup.py build -b gen/debug/build

.PHONY: py-release
py-release: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 python3 setup.py build -b gen/release/build

.PHONY: py-install
py-install: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 python3 setup.py build -b gen/release/build install --user

.PHONY: py-clean
py-clean:
	rm -rf py/gen/debug
	rm -rf py/gen/release
	rm -rf py/gen/*.c
	rm -rf py/gen/*.i
	rm -rf py/dist
	rm -rf py/dqcsim.egg-info

.PHONY: build
build: cargo-build py-build

.PHONY: release
release: cargo-release py-release

.PHONY: install
install: cargo-release cargo-install py-install

.PHONY: clean
clean: py-clean
	rm -rf c/gen
	rm -rf cpp/gen
	rm -rf py/gen
	cargo clean

