.SUFFIXES:

.PHONY: help
help:
	@echo "Run:"
	@echo " - 'make install' to install DQCsim to '~/.dqcsim' and Python modules to"
	@echo "   wherever the --user dir is (you DON'T need root)"
	@echo " - 'make release' to do a release build"
	@echo " - 'make build' to do a debug build"
	@echo " - 'make clean' to clean build artifacts"
	@echo " - 'make format' to reformat/pretty-print the rust sources"
	@echo " - 'make clippy' to run the clippy linter"
	@echo " - 'make doc' to build documentation"
	@echo " - 'make test' to run tests"
	@echo " - 'make commit' to run format, clippy, build, doc, and test; these should"
	@echo "   work before you commit!"
	@echo ""
	@echo "If cargo is not installed (\$$CARGO_HOME not set), the user is told how"
	@echo "to install rust, which requires relogging. Otherwise, everything should"
	@echo "happen automagically without requiring root privileges. Fingers crossed!"

.PHONY: %
%:
ifdef CARGO_HOME
ifeq (,$(wildcard $(CARGO_HOME)/bin/cargo-make))
	cargo install cargo-make
endif
	cargo make $@
else
	@echo "\$$CARGO_HOME is not set. This probably means that you don't have rust"
	@echo "installed. Run the following:"
	@echo ""
	@echo "    curl https://sh.rustup.rs -sSf | sh"
	@echo ""
	@echo "and follow the instructions. Specifically, after installing, either log"
	@echo "out and back in to reload your profile, or run the source script specified"
	@echo "by rustup. Then try again!"
endif
