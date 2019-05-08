from setuptools import setup, Extension
import os

try:
    os.environ['DQCSIM_RUST_RELEASE']
    libdir = '../../target/release'
except KeyError:
    libdir = '../../target/release'

with open('../Cargo.toml', 'r') as f:
    version = next(filter(lambda x: x.startswith('version = '), f.readlines()), 'version = "?.?.?"').split('"')[1]

setup(
    name = 'dqcsim',
    version = version,
    
    author = 'Quantum Computer Architectures, Quantum & Computer Engineering, QuTech, Delft University of Technology',
    author_email = '',
    description = 'Python bindings for DQCsim',
    license = "Apache-2.0",
    url = "https://github.com/mbrobbel/dqcsim-rs",
    project_urls={
        "Bug Tracker": "https://github.com/mbrobbel/dqcsim-rs/issues",
        "Documentation": "https://mbrobbel.github.io/dqcsim-rs/",
        "Source Code": "https://github.com/mbrobbel/dqcsim-rs/",
    },

    classifiers = [
        "License :: OSI Approved :: Apache Software License",

        "Operating System :: POSIX :: Linux",
        "Operating System :: MacOS",

        "Programming Language :: C",
        
    ],

    packages = setuptools.find_packages(),
    package_dir = { 'dqcsim': 'dqcsim' },
    
    ext_modules = [setuptools.Extension(
        'dqcsim._dqcsim',
        ['gen/dqcsim.c'],
        library_dirs = [libdir],
        runtime_library_dirs = [os.environ['DQCSIM_HOME'] + '/lib'],
        libraries = ['dqcsim_api'],
        extra_compile_args = ['-std=c99']
    )], 

    setup_requires = ['pdoc3'],

    install_requires = [
        'cbor',
    ],
    
    test_suite = 'nose.collector',
    tests_require = ['nose'],    
)
