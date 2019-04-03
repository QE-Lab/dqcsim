
.PHONY: test
test: build/cmake-stamp
	$(MAKE) -C build
	$(MAKE) -C build test+verbose

# Make sure cmake is called again when any CMakeLists.txt files outside of the
# build tree have been modified.
CMAKEFILES = $(shell find . -name CMakeLists.txt -not -path './build/*')
build/cmake-stamp: $(CMAKEFILES)
	rm -f $@
	mkdir -p build
	cd build && cmake ..
	touch $@

# If we're configured, clean chains to CMake-generated target. Otherwise, throw
# out the build directory.
.PHONY: clean
clean:
ifeq (,$(wildcard build/Makefile))
	rm -rf build
else
	$(MAKE) -C build clean
endif

# prune always throws out the build directory.
.PHONY: prune
prune:
	rm -rf build
