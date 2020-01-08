# Usage

The DQCsim C++ API consists of three files:

 - `dqcsim`: the primary include file for the C++ API.
 - `cdqcsim`: the C API wrapped in the `dqcsim::raw` namespace (similar to
   how C++ provides `cstdio` as a drop-in replacement for C's `stdio.h`).
 - the shared object file (`libdqcsim.so` on Linux, `dqcsim.dylib` on macOS).

These will be installed automatically in the `include` and `lib` directories
that Python is aware of when DQCsim is installed using
`sudo pip3 install dqcsim` (more detailed notes [here](../install/index.html)).

Once installed, you can use the API in your program by adding the following
include to your sources:

```C++
#include <dqcsim>

// Optionally:
using namespace dqcsim::wrap;
```

and adding `-ldqcsim` to your compiler command line, specifically the linker.
You may also need to add `-std=c++11` (or newer) if you haven't already, as
DQCsim uses features from C++11.

Note that the `dqcsim` header includes `cdqcsim`, so the above will also give
you access to the raw C API through `dqcsim::raw`. You can in fact mix the two,
if you like.

## Usage using CMake

TODO: Matthijs

## Usage after install without root

If you don't have root access on your development machine (or didn't want to
install DQCsim in your root directory), you'll also have to tell the compiler
where you installed DQCsim. You need the following flags for that:

 - `-I <path-to-dqcsim>/include`: tells the compiler where to find the header
   file.
 - `-L <path-to-dqcsim>/lib`: tells the linker where to find the shared object
   file.

At runtime, you may need to add the library directory to your runtime linker's
search path as well, using the `LD_LIBRARY_PATH` environment variable.

## Usage after building from source

If you've built DQCsim from its
[source repository](https://github.com/mbrobbel/dqcsim-rs), you need to use
the following paths:

 - `-I <dqcsim-repo>/target/include` for the header file;
 - `-L <dqcsim-repo>/target/release` or `-L <dqcsim-repo>/target/debug` for the
   shared object.

Again, you may need to add the latter to `LD_LIBRARY_PATH` as well.
