C++ API sources
===============

This folder contains the source files for the C++ API.

The C++ API is pretty much just a thin wrapper around the C API, which is
generated from the Rust sources in `rust/src/bindings` by
`rust/tools/build.rs` when DQCsim is built. This process, based on the
`cbindgen` crate, generates three equivalent header files in `target/include`:

 - `dqcsim.h` - C-style header without namespaces.
 - `cdqcsim` - C++-style header placing everything in `dqcsim::raw`.
 - `dqcsim-py.h` - simplified internal header, further processed by the Python
   API buildsystem to generate Python bindings with SWIG.

The C++ header simply includes `cdqcsim` and then adds things on top in the
`dqcsim::wrap` namespace.

In order to make the C++ header *slightly* more manageable, it is split up into
multiple header source files (`cpp/include`). All `#include "..."` directives
are then "prepreprocessed" by `build.rs` to turn them into a single header
file, which is added to `target/include` as well:

 - `dqcsim` - C++ header based on `cdqcsim` and the C++ API headers.

It is equivalent to include `cpp/include/dqcsim` and `target/include/dqcsim`
provided that the `target/include` path is added to the preprocessor search
path for `cdqcsim`, as the "prepreprocess" stage uses directives that the
C preprocessor also understands.

Tests
-----

The `test` folder, along with the `CMakeLists.txt` file test the C and C++ API.
You need to use the `CMakeLists.txt` in the root of the repository though. From
there, run

    mkdir -p build
    cd build
    cmake .. -DBUILD_TESTS=ON -DCMAKE_BUILD_TYPE=debug
    make -j
    CTEST_OUTPUT_ON_FAILURE=1 make test

to run the tests.

Examples
--------

The `examples` folder contains some of the examples listed in the C++ API
documentation, along with Makefiles to build them easily. These Makefiles
assume that DQCsim is installed in your `/usr` directory though, and will
always use this installed version rather than the source tree! Nevertheless,
they test some things that are difficult to test with GoogleTest (multiple
processes and such) and it's thus a good idea to install DQCsim and run them
before committing any changes. Just running `make` in the `examples` folder
will run them all sequentially.
