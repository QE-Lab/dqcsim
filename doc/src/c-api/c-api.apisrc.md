# C API

The C API allows any language that supports C bindings to use DQCsim for making
plugins or host programs. It consists of a header file (`dqcsim.h`) and an
associated dynamic library (`libdqcsim.so` on Linux, `dqcsim.dylib` on macOS).
These will be installed automatically in the `include` and `lib` directories
that Python is aware of when DQCsim is installed using `pip3 install dqcsim`.

@@@rust_module_doc@@@
