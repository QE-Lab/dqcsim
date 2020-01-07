
SUBDIRS := $(dir $(wildcard */Makefile))

.PHONY: test
test: $(SUBDIRS)

.PHONY: $(SUBDIRS)
$(SUBDIRS):
	$(MAKE) -C $@
