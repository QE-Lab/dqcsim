from distutils.core import setup, Extension
import os

try:
    os.environ['DQCSIM_RUST_RELEASE']
    libdir = '../../../target/release'
except KeyError:
    libdir = '../../../target/debug'

setup(name = 'dqcshost', version = '1.0',  \
    ext_modules = [Extension(
        '_dqcshost',
        ['gen/py/dqcshost.c'],
        library_dirs = [libdir],
        libraries = ['dqcshost'],
        include_dirs = ['gen/c']
    )])
