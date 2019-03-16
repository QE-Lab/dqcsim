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
	@echo "If cargo is not installed, the user is told how to install rust, which"
	@echo "requires relogging. Otherwise, everything should happen automagically"
	@echo "without requiring root privileges. Fingers crossed!"

.PHONY: %
%:
ifndef DQCSIM_HOME
 ifeq (,$(shell grep DQCSIM_HOME ~/.profile))
	@echo "\$$DQCSIM_HOME is not set. Trying to add the default value to ~/.profile now..."
	@echo "export DQCSIM_HOME=\"\$$HOME/.dqcsim\"" >> $(HOME)/.profile
  ifeq (,$(shell which cargo))
	@echo ""
	@echo "Also, cargo was not found. This probably means that you don't have rust"
	@echo "installed. Run the following:"
	@echo ""
	@echo "    curl https://sh.rustup.rs -sSf | sh"
	@echo ""
	@echo "and follow the instructions. Specifically, after installing, either log"
	@echo "out and back in to reload your profile, or run the source script specified"
	@echo "by rustup. Then try again!"
  else
	@echo ""
	@echo "Please log out and then log back in again so the changes take effect!"
  endif
 else
	@echo "\$$DQCSIM_HOME is not set, but does seem to be defined in ~/.profile. You"
	@echo "may need to log out and then log back in again if it was recently changed."
	@echo "If this message persists, modify the file manually. The default location"
	@echo "DQCsim is '\$$HOME/.dqcsim'."
 endif
else
 ifeq (,$(shell which cargo))
	@echo ""
	@echo "cargo was not found. This probably means that you don't have rust"
	@echo "installed. Run the following:"
	@echo ""
	@echo "    curl https://sh.rustup.rs -sSf | sh"
	@echo ""
	@echo "and follow the instructions. Specifically, after installing, either log"
	@echo "out and back in to reload your profile, or run the source script specified"
	@echo "by rustup. Then try again!"
 else
  ifeq (,$(shell which cargo-make))
	cargo install cargo-make
  endif
	cargo make $@
 endif
endif
