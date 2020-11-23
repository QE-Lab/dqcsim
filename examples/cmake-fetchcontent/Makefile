
# NOTE: this file just drives the CMake configure/build process!

.PHONY: all
all: run clean

.PHONY: run
run: build/main
	cd build && ./main

build/main:
	mkdir -p build
	cd build && cmake ../project
	$(MAKE) -C build --no-print-directory

.PHONY: clean
clean:
	rm -rf build
