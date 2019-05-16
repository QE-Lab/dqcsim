# Installation

If you're a user or a plugin developer (Python, C, or C++), the recommended way
to install DQCsim is through Python's package manager:

    $ sudo pip3 install dqcsim

This will install just the DQCsim core files and so-called "null" plugins for
testing. So you'll also want to install plugins in addition. This is currently
TODO, because there are no supported plugins yet. However, the current idea is
that these will also be distributed through pip, with a dependency on `dqcsim`.
For instance, you should be able to install `dqcsim-qx` through pip to get the
QX simulator with appropriate DQCsim bindings.

## Installation without superuser access

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
