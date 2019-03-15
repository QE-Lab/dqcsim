
ifndef DQCSIM_HOME
$(error $$DQCSIM_HOME is not set)
endif

.PHONY: install
install:
	mkdir -p $(DQCSIM_HOME)/bin
	cp -f ../target/release/dqcsim-cli $(DQCSIM_HOME)/bin/dqcsim
