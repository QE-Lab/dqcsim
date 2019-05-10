import os, platform, shutil, sys
from distutils.command.bdist import bdist as _bdist
from distutils.command.sdist import sdist as _sdist
from distutils.command.build import build as _build
from distutils.command.clean import clean as _clean
from setuptools.command.egg_info import egg_info as _egg_info
from setuptools import setup, Extension, find_packages
from wheel.bdist_wheel import bdist_wheel as _bdist_wheel

with open('rust/Cargo.toml', 'r') as f:
    version = next(filter(lambda x: x.startswith('version = '), f.readlines()), 'version = "?.?.?"').split('"')[1]

target_dir = os.getcwd() + "/target"
py_target_dir = target_dir + "/python"
include_dir = target_dir + "/include"
release_dir = target_dir + "/release"
build_dir = py_target_dir + "/build"
dist_dir = py_target_dir + "/dist"

class clean(_clean):
    def run(self):
        _clean.run(self)
        shutil.rmtree(py_target_dir)

class build(_build):
    def initialize_options(self):
        _build.initialize_options(self)
        self.build_base = build_dir

    def run(self):
        from plumbum import local, FG
        with local.cwd("rust"):
            try:
                cargo = local.get('cargo')
                rustc = local.get('rustc')
            except Exception as e:
                rustup = local['curl']['--proto']['=https']['--tlsv1.2']['-sSf']['https://sh.rustup.rs']
                (rustup | local['sh']) & FG
            local['cargo']["build"]["--release"]["--features"]["bindings"] & FG
        
        local['mkdir']("-p", py_target_dir)
        sys.path.append("python/tools")
        import add_swig_directives
        add_swig_directives.run(include_dir + "/dqcsim-py.h", py_target_dir + "/dqcsim.i")
        local['swig']("-v", "-python", "-py3", "-outdir", py_target_dir, "-o", py_target_dir + "/dqcsim.c", py_target_dir + "/dqcsim.i")
        _build.run(self)

class bdist(_bdist):
    def finalize_options(self):
        _bdist.finalize_options(self)
        self.dist_dir = dist_dir

class bdist_wheel(_bdist_wheel):
    def run(self):
        _bdist_wheel.run(self)
        impl_tag, abi_tag, plat_tag = self.get_tag()
        archive_basename = "{}-{}-{}-{}".format(self.wheel_dist_name, impl_tag, abi_tag, plat_tag)
        wheel_path = os.path.join(self.dist_dir, archive_basename + '.whl')
        if platform.system() == "Darwin":
            from delocate.delocating import delocate_wheel
            delocate_wheel(wheel_path)
        elif platform.system() == "Linux":
            # This only works for manylinux
            if 'AUDITWHEEL_PLAT' in os.environ:
                from auditwheel.repair import repair_wheel
                repair_wheel(wheel_path, abi=os.environ['AUDITWHEEL_PLAT'], lib_sdir=".libs", out_dir=self.dist_dir, update_tags=True)

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
        "Programming Language :: C++",
        "Programming Language :: Rust",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        
        "Topic :: Scientific/Engineering"
    ],

    data_files = [
        ('bin', [
            'target/release/dqcsim',
            'target/release/dqcsfenull',
            'target/release/dqcsopnull',
            'target/release/dqcsbenull'
        ]),
        ('include', [
            'target/include/dqcsim.h',
            'target/include/dqcsim_raw.cpp'
        ]),
        ('lib', [
            'target/release/libdqcsim.' + ('so' if platform.system() == "Linux" else 'dylib')
        ])
    ],

    packages = find_packages('python'),
    package_dir = {
        '': 'python',
    },

    cmdclass = {
        'bdist': bdist,
        'bdist_wheel': bdist_wheel,
        'build': build,
        'clean': clean,
        'egg_info': egg_info,
        'sdist': sdist,
    },

    ext_modules = [
        Extension(
            'dqcsim._dqcsim',
            [py_target_dir + "/dqcsim.c"],
            libraries = ['dqcsim'],
            library_dirs = [release_dir],
            runtime_library_dirs = [release_dir],
            include_dirs = [include_dir],
            extra_compile_args = ['-std=c99']
        )
    ],

    setup_requires = [
        'plumbum',
        'delocate; platform_system == "Darwin"',
    ],

    install_requires = [
        'cbor',
    ],
    
    tests_require = [
        'nose'
    ],
    test_suite = 'nose.collector',

    zip_safe = False,
)
