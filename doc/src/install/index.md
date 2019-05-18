# Installation

Here's how to install DQCsim. If you're on Linux or macOS, it's easy: just
install Python 3.5+ and follow one of the three installation methods listed
below. If you're on Windows, you'll unfortunately have to wait, since DQCsim
has a [dependency that doesn't support Windows](https://github.com/servo/ipc-channel).

## Recommended method (requires superuser access)

The recommended way to install DQCsim is through Python's package manager in
the usual way:

    $ sudo pip3 install dqcsim

Besides the Python module, this also installs the development headers and
dynamic libraries needed to develop C/C++ plugins or host programs. On most
distributions Python installs into `/usr/local`, which should be part of your
compiler's search paths already.

## Installation into your home directory

If you don't have superuser access, you can also install to your home directory
as follows:

    $ pip3 install dqcsim --user

This will normally install the package into `~/.local`. You should probably
check if `~/.local/bin` is in your `$PATH` environment variable, otherwise the
command-line interface and plugins may not work out of the box. If you're
developing in C or C++, you'll also have to add the following to
`CFLAGS`: `-I ~/.local/include -L ~/.local/lib`.

## Installation into a venv

You can also install into a
[venv](https://docs.python.org/3/library/venv.html#creating-virtual-environments).
This is particularly useful if you want to have multiple versions installed at
the same time. To create a venv and install into it, run the following:

    $ mkdir -p <your-install-directory>
    $ cd <your-install-directory>
    $ python3 -m venv <your-install-directory>
    $ source <your-install-directory>/bin/activate
    (venv) $ pip3 install dqcsim

To leave the `venv`, run

    (venv) $ deactivate

If you're developing in C or C++, you'll also have to add the following to
`CFLAGS`: `-I <your-install-directory>/include -L <your-install-directory>/lib`.
