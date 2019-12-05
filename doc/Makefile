APISRC_FILES = $(shell find src -name '*.apisrc.md')
APIGEN_FILES = $(APISRC_FILES:.apisrc.md=.apigen.md)

.PHONY: all
all: book python rust

.PHONY: open
open: all
	open ../target/book/index.html || xdg-open ../target/book/index.html

.PHONY: clean
clean:
	rm -rf ../target/book $(APIGEN_FILES)

.PHONY: book
book: $(APIGEN_FILES)
	mdbook build

$(APIGEN_FILES): tools/api_gen.py ../target/include/dqcsim.h ../rust/src/bindings/mod.rs $(APISRC_FILES)
	python3 $^

../target/include/dqcsim.h:
	cargo build --all-features

.PHONY: python
python:
	@(cd .. && python3 -m pdoc --html --output-dir target/book/py_ dqcsim --force) \
		|| echo "WARNING: pdoc3 failed, Python doc not generated"

.PHONY: rust
rust:
	cargo doc --no-deps --all-features
	cp -R ../target/doc/ ../target/book/rust_

