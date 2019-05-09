import os
from distutils.command.bdist import bdist as _bdist
from distutils.command.sdist import sdist as _sdist
from distutils.command.build import build as _build
from setuptools.command.egg_info import egg_info as _egg_info
from setuptools import setup, Extension, find_packages

with open('rust/Cargo.toml', 'r') as f:
    version = next(filter(lambda x: x.startswith('version = '), f.readlines()), 'version = "?.?.?"').split('"')[1]

target_dir = "target"
py_target_dir = target_dir + "/python"
include_dir = target_dir + "/include"
debug_dir = target_dir + "/debug"
build_dir = py_target_dir + "/build"
dist_dir = py_target_dir + "/dist"

class build(_build):
    def initialize_options(self):
        _build.initialize_options(self)
        self.build_base = build_dir

    def run(self):
        from plumbum import local, FG
        with local.cwd("rust"):
            local['cargo']["build"]["--no-default-features"]["--features"]["bindings"] & FG
        local['mkdir']("-p", py_target_dir)
        local['python3']("python/tools/add_swig_directives.py", include_dir + "/dqcsim-py.h", py_target_dir + "/dqcsim.i")
        local['swig']("-v", "-python", "-py3", "-outdir", py_target_dir, "-o", py_target_dir + "/dqcsim.c", py_target_dir + "/dqcsim.i")
        _build.run(self)

class bdist(_bdist):
    def finalize_options(self):
        _bdist.finalize_options(self)
        self.dist_dir = dist_dir

class sdist(_sdist):
    def finalize_options(self):
        _sdist.finalize_options(self)
        self.dist_dir = dist_dir

class egg_info(_egg_info):
    def initialize_options(self):
        _egg_info.initialize_options(self)
        self.egg_base = py_target_dir

setup(
    name = 'dqcsim',
    version = version,
    
    author = 'Quantum Computer Architectures, Quantum & Computer Engineering, QuTech, Delft University of Technology',
    author_email = '',
    description = 'Python bindings for DQCsim',
    license = "Apache-2.0",
    url = "https://github.com/mbrobbel/dqcsim-rs",
    project_urls = {
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

    packages = find_packages('python'),
    package_dir = {
        '': 'python',
    },

    cmdclass = {
        'egg_info': egg_info,
        'build': build,
        'bdist': bdist,
        'sdist': sdist,
    },

    ext_modules = [
        Extension(
            'dqcsim._dqcsim',
            [py_target_dir + "/dqcsim.c"],
            libraries = ['dqcsim'],
            library_dirs = [debug_dir],
            include_dirs = [include_dir],
            extra_compile_args = ['-std=c99']
        )
    ],

    setup_requires = [
        'plumbum'
    ],

    install_requires = [
        'cbor',
    ],
    
    tests_require = ['nose'],
    test_suite = 'nose.collector',

    zip_safe = False,
)
