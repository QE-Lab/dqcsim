
Check the rust core:

    cargo fmt && cargo clippy --all-features --all-targets && cargo test --all-features --all-targets

Rebuild and reinstall DQCsim:

    sudo pip3 uninstall dqcsim -y && rm -rf target/python/dist && python3 setup.py build && python3 setup.py bdist_wheel && sudo pip3 install target/python/dist/*

To speed up the above:

    export DQCSIM_DEBUG=yes

Building documentation:

    make -C doc

