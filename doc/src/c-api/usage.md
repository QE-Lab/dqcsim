# Usage

The DQCsim C API consists of two files: the header file (`dqcsim.h`) and an
associated dynamic library (`libdqcsim.so` on Linux, `dqcsim.dylib` on macOS).
These will be installed automatically in the `include` and `lib` directories
that Python is aware of when DQCsim is installed using
`sudo pip3 install dqcsim` (more detailed notes [here](../install/index.html)).

Once installed, you can use the API in your program by adding the following
include to your sources:

```C
#include <dqcsim.h>
```

and adding `-ldqcsim` to your compiler command line, specifically the linker.

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
[source repository](https://github.com/qe-lab/dqcsim), you need to use
the following paths:

 - `-I <dqcsim-repo>/target/include` for the header file;
 - `-L <dqcsim-repo>/target/release` or `-L <dqcsim-repo>/target/debug` for the
   shared object.

Again, you may need to add the latter to `LD_LIBRARY_PATH` as well.
