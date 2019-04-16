
ifndef DQCSIM_HOME
$(error $$DQCSIM_HOME is not set)
endif

PYTHON = `which python3`

PY_SOURCES = $(shell find py/dqcsim -name '*.py')
PY_TESTS = $(shell find py/tests -name '*.py')
UNAME = $(shell uname)

ifeq ($(UNAME),Linux)
	DYLIB_EXT = "so"
endif
ifeq ($(UNAME),Darwin)
	DYLIB_EXT = "dylib"
endif

.SUFFIXES:

# This is needed because a bunch of the rules here don't have proper
# dependencies. This largely has to do with stuff conflicting with
# Cargo/build.rs - all three of cargo-build, cargo-release, and cargo-install
# *probably* run build.rs, which generates the header files that the rest of
# the targets here depend on. But there is no good way to *just* run build.rs
# as a dependency; Cargo makes you do one of those three. So let's say you pick
# cargo-release and there's a cargo-build dependency on the header files,
# you'd end up with a build that's twice as slow because now cargo runs twice.
# To avoid all this nonsense we take make's parallelism out of the mix and just
# make sure we just list whatever cargo command the user wants first. NOTE:
# this also means that building *just* the language-specific targets may fail.
.NOTPARALLEL:

.PHONY: all
all: build

#------------------------------------------------------------------------------
# Rust/Cargo targets
#------------------------------------------------------------------------------
.PHONY: cargo-build
cargo-build:
	cargo build
	cp -fp ../target/debug/libdqcsim_api.$(DYLIB_EXT) ../target/debug/libdqcsim.$(DYLIB_EXT)

.PHONY: cargo-release
cargo-release:
	cargo build --release
	cp -fp ../target/release/libdqcsim_api.$(DYLIB_EXT) ../target/release/libdqcsim.$(DYLIB_EXT)

.PHONY: cargo-install
cargo-install: cargo-release
	mkdir -p $(DQCSIM_HOME)/lib
	cp -fp ../target/release/libdqcsim.$(DYLIB_EXT) $(DQCSIM_HOME)/lib/libdqcsim.$(DYLIB_EXT)

.PHONY: cargo-test
cargo-test:
	cargo test

.PHONY: cargo-clean
cargo-clean:
	rm -rf c/gen
	rm -rf cpp/gen
	rm -rf py/gen
	cargo clean

#------------------------------------------------------------------------------
# C API targets
#------------------------------------------------------------------------------
.PHONY: c-install
c-install: | cargo-build cargo-release cargo-install
	mkdir -p $(DQCSIM_HOME)/include
	cp -fp c/gen/include/* $(DQCSIM_HOME)/include

# NOTE: there is no test target here. To keep things simpler, the C/raw C++
# APIs are tested by the same build system that tests the C++ API.

#------------------------------------------------------------------------------
# C++ API targets
#------------------------------------------------------------------------------
.PHONY: cpp-install
cpp-install: | cargo-build cargo-release cargo-install
	mkdir -p $(DQCSIM_HOME)/include
	cp -fp cpp/gen/include/* $(DQCSIM_HOME)/include
	cp -fp cpp/include/* $(DQCSIM_HOME)/include

.PHONY: cpp-test
cpp-test:
	$(MAKE) -C cpp test

.PHONY: cpp-clean
cpp-clean:
	$(MAKE) -C cpp clean

.PHONY: cpp-prune
cpp-prune:
	$(MAKE) -C cpp prune

#------------------------------------------------------------------------------
# Python API targets
#------------------------------------------------------------------------------
py/gen/dqcsim.i: py/add_swig_directives.py py/gen/dqcsim.h
	$(PYTHON) $^ $@

py/gen/dqcsim.c: py/gen/dqcsim.i py/gen/dqcsim.h
	mkdir -p py/gen
	swig -v -python -py3 -outdir py/gen -o py/gen/dqcsim.c $<
	rm -f py/gen/dqcsim.py

.PHONY: py-build
py-build: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && $(PYTHON) setup.py build

.PHONY: py-release
py-release: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 $(PYTHON) setup.py build

.PHONY: py-install
py-install: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES)
	cd py && DQCSIM_RUST_RELEASE=1 $(PYTHON) setup.py install --user --prefix=
	mkdir -p $(DQCSIM_HOME)/bin
	ln -fs $(PYTHON) $(DQCSIM_HOME)/bin/dqcsfepy
	ln -fs $(PYTHON) $(DQCSIM_HOME)/bin/dqcsoppy
	ln -fs $(PYTHON) $(DQCSIM_HOME)/bin/dqcsbepy

.PHONY: py-test
py-test: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES) $(PY_TESTS)
	cd py && LD_LIBRARY_PATH=$(shell pwd)/../target/debug $(PYTHON) setup.py test

# NOTE: py-doc depends on py-test to make sure the shared objects are correctly
# spammed around by setup.py, and to make sure that pdoc3 is installed.
.PHONY: py-doc
py-doc: py-test
	rm -rf py/build/doc
	cd py && LD_LIBRARY_PATH=$(shell pwd)/../target/debug PYTHONPATH=. pdoc3 --html --html-dir build/doc dqcsim

# NOTE: this target is meant for manual invocation! It depends on kcov being
# installed.
.PHONY: py-cov
py-cov: py/gen/dqcsim.c py/gen/dqcsim.h py/setup.py $(PY_SOURCES) $(PY_TESTS)
	rm -rf py/build/cov
	cd py && LD_LIBRARY_PATH=$(shell pwd)/../target/debug kcov --python-parser=$(PYTHON) --include-pattern=/py/dqcsim/ --exclude-line=no_kcoverage build/cov setup.py test
	@echo "Reports are here: file://$(shell pwd)/py/build/cov/index.html"

# NOTE: this target is meant for manual invocation! It depends on kcov being
# installed and there being a graphical environment with a browser to open the
# reports in.
.PHONY: py-cov-open
py-cov-open: py-cov
	xdg-open py/build/cov/index.html

.PHONY: py-clean
py-clean:
	rm -rf py/build
	rm -rf py/gen/*.c
	rm -rf py/gen/*.i
	rm -rf py/dist
	rm -rf py/dqcsim.egg-info
	rm -rf $(shell find py -name __pycache__)
	rm -f $(shell find py/dqcsim -type f -and -not -name '*.py')

#------------------------------------------------------------------------------
# Combo targets
#------------------------------------------------------------------------------
# These targets should always work from a clean build tree.

# Perform a debug build, called for "cargo make build".
.PHONY: build
build: cargo-build py-build

# Perform a release build, called for "cargo make release".
.PHONY: release
release: cargo-release py-release

# Perform a release build and install, called for "cargo make install".
.PHONY: install
install: cargo-install c-install cpp-install py-install

# Perform tests (using the debug build), called for "cargo make test".
.PHONY: test
test: cargo-build cargo-test cpp-test py-test

# Clean build objects but not generated code (i.e. header files generated by
# cbindgen through build.rs and makefiles generated by CMake). This is called
# for "cargo make clean".
.PHONY: clean
clean: cargo-clean py-clean cpp-clean

# Throw away all generated files.
.PHONY: prune
prune: cargo-clean py-clean cpp-prune
