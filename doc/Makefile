APISRC_FILES = $(shell find src -name '*.apisrc.md')
APIGEN_FILES = $(APISRC_FILES:.apisrc.md=.apigen.md)

all: $(APIGEN_FILES)
	mdbook build

$(APIGEN_FILES): tools/api_gen.py ../target/include/dqcsim.h ../rust/src/bindings/mod.rs $(APISRC_FILES)
	python3 $^

../target/include/dqcsim.h:
	cargo build --all-features
