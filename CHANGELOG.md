## [0.0.13] - 2020-01-20
- Merge remote-tracking branch origin/master into meas-basis-and-prep
- Bump proc-macro-error from 0.4.4 to 0.4.5
- Merge branch master of github.com:mbrobbel/dqcsim
- Add file with some useful command-line one-liners
- Fix matrices losing dimensionality in transit
- Expose matrix unitary check and fix docs
- Bump structopt from 0.3.7 to 0.3.8
- Bump humantime from 1.3.0 to 2.0.0
- Update C++ API for gate type changes
- Update python API, sort of
- Update C/book documentation for gate type changes
- Fix Rust docstring
- Add dqcs_gate_type API, rename dqcs_gate_is_custom
- BREAKING CHANGE: meas bases and prep gate in rust
- Update cmake in examples workflow
- Fix deadlock for no-op operator callbacks
- Log python errors in callbacks
- Fix missing dqcs_log_format symbol
- Make fetchcontent work, and improve examples
- Minor changes to CMake example
- Add CMake example
- Add install target stuff in cmake
- Condition lib64/libdqcsim.so on Linux OS
- Add python 3.8 to classifiers in setup.py
- Fix setup.py workflow trigger
- Package libdqcsim.so in lib64 as well

## [0.0.12] - 2020-01-17
-  Add Matrix type and add gate map mechanism #390
-  Modify the gate processing reference algorithm to allow for non-Z measurements in the future #393
-  Fix Y measurement in Python & C++ API #394

## [0.0.11] - 2020-01-15
-  8ac019f Fix python path in manylinux image

## [0.0.10] - 2020-01-15
-  19cfa7a Add macos-py38 and manylinux2014 wheels
-  a054a59 Make internal CBOR representation canonical

## [0.0.9] - 2020-01-13
-  Add C++ header-only library

## [0.0.8] - 2020-01-09

-  66e4e6f Fix path to wheels in pypi publish job
-  b91729d Update Rust in Crates.io publish job

## [0.0.7] - 2020-01-09

-  1d01a4f Fix documentation deploy condition
-  52ff27a Update workflow triggers
-  e5d96fc Update url to workflows
-  8dd4538 Merge pull request #342 from mbrobbel/github-actions
-  b39da13 Update badges
-  d96e657 Add Release workflow
-  65df953 Remove templates
-  2081056 Add Documentation workflow
-  fe9de2e Bump quick-error from 1.2.2 to 1.2.3
-  c9540e6 Bump proc-macro2 from 1.0.6 to 1.0.7
-  aae809c Bump rustversion from 1.0.0 to 1.0.1
-  cbe523d Bump cbindgen from 0.12.0 to 0.12.1
-  dcf0c96 Bump structopt from 0.3.6 to 0.3.7
-  eff57b9 Add C++ coverage
-  a037997 Merge pull request #353 from mbrobbel/update-channels
-  e247b40 Update crossbeam-channel and ipc-channel
-  0e271dd Bump structopt from 0.3.5 to 0.3.6
-  25cd9c4 Bump strum_macros from 0.16.0 to 0.17.1
-  125a6c1 Bump strum from 0.16.0 to 0.17.1
-  6b1fa80 Bump whoami from 0.6.0 to 0.7.0
-  94298aa Add Python coverage
-  3e41901 Update Rust workflow
-  1109ca8 Add Coverage workflow
-  db366f8 Add Python workflow
-  f666eb0 Add C++ workflow
-  d911f51 Add Rust workflow
-  54c6315 Merge pull request #343 from mbrobbel/rust-1.40
-  644ace1 Updates for Rust 1.40
-  c3498f6 Bump cbindgen from 0.11.1 to 0.12.0
-  9278465 Bump serde from 1.0.103 to 1.0.104
-  3b9afa7 Bump log from 0.4.8 to 0.4.10
-  5e19263 Merge pull request #332 from mbrobbel/issue-329
-  92bd35f Fix incorrect terminology in documentation
-  567d66d Bump bincode from 1.2.0 to 1.2.1
-  7710c3e Bump cbindgen from 0.11.0 to 0.11.1
-  0665816 Merge pull request #324 from mbrobbel/codecov
-  f256190 Update codecov.yml
-  666a86a Merge pull request #322 from mbrobbel/fix-r-gate
-  4b59cc5 Update codecov.yml
-  88a796e Add codecov.yml
-  5205298 Fix R gate in Python
-  1f8df72 Bump unicode-width from 0.1.6 to 0.1.7
-  beece2f Merge pull request #320 from mbrobbel/docgen-cleanup
-  1e465d4 Replace underline header style with # for consistency
-  e9cac7a Clean up documentation generation
-  0773ca2 Bump serde_json from 1.0.43 to 1.0.44
-  8af48cf Bump cc from 1.0.47 to 1.0.48
-  44dbafb Bump serde_json from 1.0.42 to 1.0.43
-  2a525af Bump cbindgen from 0.10.0 to 0.11.0
-  fe1fbd0 Bump libc from 0.2.65 to 0.2.66
-  733605d Bump mio from 0.6.19 to 0.6.21
-  085d54e Merge pull request #309 from mbrobbel/cmake-compile-units
-  55b1423 Update test unit definition
-  d60445d Fix CMake paths
-  bc39af8 Update travis.yml
-  38abdca Use debug build in cpp ci
-  93cc438 Allow Debug builds of dqcsim crate from CMake
-  77d58f6 Update test executable output path in coverage job
-  85f5c27 Fix build of clean project
-  0773f90 Re-enable disabled plugin tests
-  5323f34 Update CMake in CI
-  4bb499a Use CompileUnits CMake module for dqcsim-cpp
-  bdeded9 Bump num-traits from 0.2.9 to 0.2.10
-  d7eaf53 Bump serde_json from 1.0.41 to 1.0.42
-  166885a Bump serde from 1.0.102 to 1.0.103
-  82a5a80 Bump chrono from 0.4.9 to 0.4.10
-  31f10d5 Bump structopt from 0.3.4 to 0.3.5
-  1b6bd5a Bump cbindgen from 0.9.1 to 0.10.0
-  22fd703 Bump wasm-bindgen from 0.2.54 to 0.2.55
-  4d4c86e Bump float-cmp from 0.5.3 to 0.6.0
-  d31b2f1 Bump num-traits from 0.2.8 to 0.2.9
-  ed1eb80 Bump synstructure from 0.12.2 to 0.12.3
-  c594f70 Bump unicode-segmentation from 1.5.0 to 1.6.0
-  cd3c140 Bump structopt from 0.3.3 to 0.3.4
-  648ad09 Bump wasm-bindgen from 0.2.53 to 0.2.54

## [0.0.6] - 2019-11-0

-  f3435061c Add swap gate validation test
-  60bdeec47 Add some tests and a check in gate constructor to validatem matrix is unitary
-  dd0862fdb Bump blake2b_simd from 0.5.8 to 0.5.9
-  f48824f9e Add gates enum
-  aac63f266 Bump synstructure from 0.12.1 to 0.12.2
-  201507a36 Clippy
-  38ac8f92c Rustfmt
-  9f54d76e8 Add gates module with functions to generate common gates
-  5f287e304 Bump cc from 1.0.46 to 1.0.47
-  e0696ab64 Update some gate internals to improve Rust plugin dev
-  3f9fd8533 Bump proc-macro2 from 1.0.3 to 1.0.6-  8edfc62f6 Bump strum_macros from 0.15.0 to 0.16.0
-  62a98dc3f Bump toml from 0.5.4 to 0.5.5
-  15c69a3a0 Bump serde_derive from 1.0.101 to 1.0.102
-  7cd00ed71 Bump git-testament-derive from 0.1.7 to 0.1.8
-  adb746163 Bump unicode-segmentation from 1.3.0 to 1.5.0
-  3d7a72718 Bump wasm-bindgen from 0.2.52 to 0.2.53
-  5402a2e6e Bump serde from 1.0.101 to 1.0.102
-  76e0df781 Bump uuid from 0.8.0 to 0.8.1
-  28a4e8069 Bump toml from 0.5.3 to 0.5.4
-  ff06894d6 Bump git-testament from 0.1.6 to 0.1.7
-  b1d199380 Bump strum from 0.15.0 to 0.16.0
-  1cc67a774 Bump c2-chacha from 0.2.2 to 0.2.3
-  ff6ed6f25 Bump autocfg from 0.1.6 to 0.1.7
-  448cd99d5 Bump getrandom from 0.1.12 to 0.1.13
-  9abee25c3 Bump libc from 0.2.64 to 0.2.65
-  e2416192b Bump whoami from 0.5.3 to 0.6.0
-  ecb55961d Bump backtrace from 0.3.38 to 0.3.40
-  886ed60bf Bump ppv-lite86 from 0.2.5 to 0.2.6
-  827ff10ad Bump ipc-channel from 0.12.1 to 0.12.2
-  bf83ba829 Bump backtrace-sys from 0.1.31 to 0.1.32
-  72652d12c Bump cc from 1.0.45 to 1.0.46
-  7e71c31f9 Bump ipc-channel from 0.12.0 to 0.12.1
-  5a95211ac Bump libc from 0.2.62 to 0.2.64
-  8744cde07 Bump ryu from 1.0.1 to 1.0.2
-  d2457f478 Bump bitflags from 1.2.0 to 1.2.1
-  6d70643dd Bump half from 1.3.1 to 1.4.0
-  1bd220354 Bump failure from 0.1.5 to 0.1.6
-  21db07397 Bump ryu from 1.0.0 to 1.0.1
-  e60063a1e Bump failure_derive from 0.1.5 to 0.1.6
-  d5190c986 Bump structopt from 0.3.2 to 0.3.3
-  be3922cef Bump iovec from 0.1.2 to 0.1.4
-  c0ece439d Bump arrayvec from 0.4.11 to 0.4.12
-  d24a3e625 Bump nodrop from 0.1.13 to 0.1.14
-  713607063 Bump serde_yaml from 0.8.9 to 0.8.11
-  184f9c112 Bump half from 1.3.0 to 1.3.1
-  f0eb1c18e Bump serde_json from 1.0.40 to 1.0.41
-  964d1a1d5 Bump serde_cbor from 0.10.1 to 0.10.2
-  c0c453ed5 Update to Rust 1.38
-  f1fea0fe2 Bump bincode from 1.1.4 to 1.2.0
-  31d6f00b9 Bump cfg-if from 0.1.9 to 0.1.10
-  6c255e44d Bump bitflags from 1.1.0 to 1.2.0
-  f1fa8d126 Bump backtrace from 0.3.37 to 0.3.38
-  33086e280 Bump structopt from 0.3.1 to 0.3.2
-  181c6597c Bump rand from 0.7.1 to 0.7.2
-  5d09135ad Bump serde from 1.0.100 to 1.0.101
-  9c7c8f879 Bump serde_derive from 1.0.100 to 1.0.101
-  bdc442b9c Update Cargo.lock
-  e1e35044d Bump rand from 0.7.0 to 0.7.1

## [0.0.5] - 2019-09-10

-  cea4d3506 Merge pull request #225 from mbrobbel/dependabot/cargo/structopt-0.3.1
-  075caa32c Reflect structopt changes
-  20e23c29a Merge pull request #232 from mbrobbel/dependabot/cargo/humantime-1.3.0
-  dae2b24c0 Bump humantime from 1.2.0 to 1.3.0
-  542f6635b Merge pull request #231 from mbrobbel/dependabot/cargo/serde_derive-1.0.100
-  dda00c699 Merge pull request #230 from mbrobbel/dependabot/cargo/serde-1.0.100
-  54a64ac19 Bump serde_derive from 1.0.99 to 1.0.100
-  d00751dbb Bump serde from 1.0.99 to 1.0.100
-  e87439e78 Merge pull request #229 from mbrobbel/dependabot/cargo/cc-1.0.45
-  fbdcb6a32 Bump cc from 1.0.41 to 1.0.45
-  5e3ebf1af Merge pull request #226 from mbrobbel/dependabot/cargo/getrandom-0.1.12
-  62fa6e8c9 Merge pull request #224 from mbrobbel/dependabot/cargo/blake2b_simd-0.5.8
-  648dfc56e Merge pull request #222 from mbrobbel/dependabot/cargo/backtrace-0.3.37
-  cd2e7a3f5 Bump getrandom from 0.1.11 to 0.1.12
-  0f02c7b1f Bump structopt from 0.2.18 to 0.3.1
-  19352b0ea Bump blake2b_simd from 0.5.7 to 0.5.8
-  382b59fbe Bump backtrace from 0.3.35 to 0.3.37
-  b41fbd305 Merge pull request #220 from mbrobbel/dependabot/cargo/chrono-0.4.9
-  bc4485c75 Bump chrono from 0.4.8 to 0.4.9
-  31c6e8632 Merge pull request #219 from mbrobbel/dependabot/cargo/ansi_term-0.12.1
-  74d5113cc Bump ansi_term from 0.12.0 to 0.12.1
-  00767cad2 Merge pull request #218 from mbrobbel/dependabot/cargo/chrono-0.4.8
-  f2b9fb29f Bump chrono from 0.4.7 to 0.4.8
-  dc6cb8180 Merge pull request #216 from mbrobbel/dependabot/cargo/cc-1.0.41
-  3447132e4 Bump cc from 1.0.40 to 1.0.41
-  3c3a3025b Merge pull request #215 from mbrobbel/update-deps
-  ac68d00f3 Update deps
-  a5a89434c Merge pull request #209 from mbrobbel/dependabot/cargo/blake2b_simd-0.5.7
-  f8d548e4c Merge pull request #205 from mbrobbel/dependabot/cargo/backtrace-0.3.35
-  677373b95 Merge pull request #213 from mbrobbel/dependabot/cargo/lazy_static-1.4.0
-  5f067f945 Bump lazy_static from 1.3.0 to 1.4.0
-  1de96eac6 Bump backtrace from 0.3.34 to 0.3.35
-  d92670c86 Merge pull request #200 from mbrobbel/dependabot/cargo/libc-0.2.62
-  0913e821d Merge pull request #198 from mbrobbel/dependabot/cargo/toml-0.5.3
-  47d31bae9 Bump blake2b_simd from 0.5.6 to 0.5.7
-  1e44d0d42 Bump toml from 0.5.1 to 0.5.3
-  24b666473 Bump libc from 0.2.60 to 0.2.62
-  0b8299439 Merge pull request #211 from mbrobbel/dependabot/cargo/cbindgen-0.9.1
-  061254318 Bump cbindgen from 0.9.0 to 0.9.1
-  b0b9f304a Merge pull request #195 from mbrobbel/dependabot/cargo/cc-1.0.40
-  77111825b Bump cc from 1.0.38 to 1.0.40
-  dd4ca3322 Merge pull request #207 from mbrobbel/rust-1.37
-  370e3a008 Update to Rust 1.37
-  9225796ca Merge pull request #192 from mbrobbel/dependabot/cargo/redox_users-0.3.1
-  8eb7380a3 Merge pull request #191 from mbrobbel/dependabot/cargo/term-0.6.1
-  594d9599e Bump redox_users from 0.3.0 to 0.3.1
-  56e5a917c Bump term from 0.6.0 to 0.6.1
-  7094da4c4 Merge pull request #189 from mbrobbel/dependabot/cargo/backtrace-0.3.34
-  d6537490f Bump backtrace from 0.3.33 to 0.3.34
-  61760b00d Merge pull request #186 from mbrobbel/dependabot/cargo/getrandom-0.1.7
-  8e40bf3c3 Merge pull request #188 from mbrobbel/dependabot/cargo/dirs-2.0.2
-  0087265d0 Bump dirs from 2.0.1 to 2.0.2
-  ddf6dbcfd Merge pull request #187 from mbrobbel/dependabot/cargo/dirs-sys-0.3.4
-  3c24a51de Bump dirs-sys from 0.3.3 to 0.3.4
-  2c1e72a92 Merge pull request #184 from mbrobbel/dependabot/cargo/serde-1.0.98
-  c60526d5f Merge pull request #183 from mbrobbel/dependabot/cargo/serde_derive-1.0.98
-  27727cdea Bump getrandom from 0.1.6 to 0.1.7
-  5dc05de19 Bump serde from 1.0.97 to 1.0.98
-  2b2f2f71d Merge pull request #185 from mbrobbel/dependabot/cargo/log-0.4.8
-  4fa20085b Merge pull request #182 from mbrobbel/dependabot/cargo/term-0.6.0
-  08ba63680 Bump log from 0.4.7 to 0.4.8
-  3dc574ccb Bump serde_derive from 1.0.97 to 1.0.98
-  adf508c95 Bump term from 0.5.2 to 0.6.0
-  026a0f2bc Merge pull request #181 from mbrobbel/dependabot/cargo/rand_chacha-0.2.1
-  32ea5f49d Bump rand_chacha from 0.2.0 to 0.2.1
-  97f6eaa3a Merge pull request #180 from mbrobbel/dependabot/cargo/crossbeam-channel-0.3.9
-  5da588d4f Bump crossbeam-channel from 0.3.8 to 0.3.9
-  9bd959bba Merge pull request #179 from mbrobbel/dependabot/cargo/crossbeam-utils-0.6.6
-  ab72ef7bc Bump crossbeam-utils from 0.6.5 to 0.6.6
-  5c1ab28c6 Merge pull request #178 from mbrobbel/dependabot/cargo/cc-1.0.38
-  a444248fd Bump cc from 1.0.37 to 1.0.38
-  cb8185143 Merge pull request #175 from mbrobbel/dependabot/cargo/serde_derive-1.0.97
-  dbc9ce35f Merge pull request #176 from mbrobbel/dependabot/cargo/serde-1.0.97
-  2ebb0e881 Merge pull request #177 from mbrobbel/dependabot/cargo/whoami-0.5.3
-  52ce84017 Bump whoami from 0.5.2 to 0.5.3
-  6db4271b7 Merge pull request #172 from mbrobbel/dependabot/cargo/backtrace-sys-0.1.31
-  051a46ccb Merge branch master into dependabot/cargo/backtrace-sys-0.1.31
-  d5511a14e Merge pull request #171 from mbrobbel/dependabot/cargo/backtrace-0.3.33
-  61bc1a6b7 Bump serde from 1.0.94 to 1.0.97
-  9a99dc3eb Bump serde_derive from 1.0.94 to 1.0.97
-  f5c2ab907 Bump backtrace-sys from 0.1.30 to 0.1.31
-  673424462 Bump backtrace from 0.3.32 to 0.3.33
-  8fea67d88 Merge pull request #168 from mbrobbel/dependabot/cargo/autocfg-0.1.5
-  c8e886f1c Bump autocfg from 0.1.4 to 0.1.5
-  a170a5c17 Merge pull request #167 from mbrobbel/dependabot/cargo/atty-0.2.13
-  d09a11cff Bump atty from 0.2.12 to 0.2.13
-  34970338e Merge pull request #166 from mbrobbel/dependabot/cargo/libc-0.2.60
-  a11cfd23f Bump libc from 0.2.59 to 0.2.60
-  7a72bb092 Merge pull request #165 from mbrobbel/dependabot/cargo/ansi_term-0.12.0
-  9ba315701 Bump ansi_term from 0.11.0 to 0.12.0
-  adee6b531 Merge pull request #164 from mbrobbel/dependabot/cargo/atty-0.2.12
-  07e85f62d Merge pull request #163 from mbrobbel/dependabot/cargo/serde_cbor-0.10.1
-  e71e339e9 Merge pull request #162 from mbrobbel/dependabot/cargo/log-0.4.7
-  5cb028900 Bump atty from 0.2.11 to 0.2.12
-  e2ea71c12 Bump serde_cbor from 0.10.0 to 0.10.1
-  80a8a30e7 Bump log from 0.4.6 to 0.4.7
-  7bbcb8abb Merge pull request #161 from mbrobbel/dependabot/cargo/arrayvec-0.4.11
-  0759da957 Bump arrayvec from 0.4.10 to 0.4.11
-  5f4c9be93 Merge pull request #160 from mbrobbel/dependabot/cargo/libc-0.2.59
-  f640eac6e Bump libc from 0.2.58 to 0.2.59
-  88e100c8b Update python manylinux build tools

## [0.0.4] - 2019-07-05

-  db43b0402 Merge pull request #157 from mbrobbel/dependabot/cargo/serde_cbor-0.10.0
-  69af77d9c Update tests to reflect debug output change
-  8957b5ae4 Bump serde_cbor from 0.9.0 to 0.10.0
-  455691399 Merge pull request #159 from mbrobbel/dependabot/cargo/redox_syscall-0.1.56
-  aec379f64 Bump redox_syscall from 0.1.55 to 0.1.56
-  d9a840ff2 Update to Rust 1.36
-  757d8291a Merge pull request #158 from mbrobbel/dependabot/cargo/redox_syscall-0.1.55
-  fa96d9ee3 Bump redox_syscall from 0.1.54 to 0.1.55
-  08c0e2b93 Merge pull request #156 from mbrobbel/dependabot/cargo/serde_json-1.0.40
-  167e0e16a Merge pull request #155 from mbrobbel/dependabot/cargo/tempfile-3.1.0
-  4496bdad6 Bump serde_json from 1.0.39 to 1.0.40
-  f23cc9c4c Bump tempfile from 3.0.9 to 3.1.0
-  928f9ffc9 Merge pull request #154 from mbrobbel/dependabot/cargo/backtrace-sys-0.1.30
-  d98406371 Merge pull request #153 from mbrobbel/dependabot/cargo/tempfile-3.0.9
-  5ab6beca8 Merge pull request #152 from mbrobbel/dependabot/cargo/getrandom-0.1.6
-  778f5e293 Bump backtrace-sys from 0.1.29 to 0.1.30
-  63f12a5ac Bump tempfile from 3.0.8 to 3.0.9
-  aa60312a3 Bump getrandom from 0.1.4 to 0.1.6
-  39fddf355 Merge pull request #151 from mbrobbel/dependabot/cargo/getrandom-0.1.4
-  2735c841c Bump getrandom from 0.1.3 to 0.1.4
-  947d23930 Update rand to 0.7
-  26568a862 Merge pull request #136 from mbrobbel/dependabot/cargo/rand_chacha-0.2.0
-  4db5df875 Fixes for rand_chacha update
-  893c91456 Merge pull request #149 from mbrobbel/dependabot/cargo/serde_derive-1.0.94
-  ffdf85ac0 Bump serde_derive from 1.0.93 to 1.0.94
-  c11fade46 Merge pull request #146 from mbrobbel/dependabot/cargo/backtrace-sys-0.1.29
-  28eebe8d1 Bump backtrace-sys from 0.1.28 to 0.1.29
-  72d4b9a3b Merge pull request #148 from mbrobbel/dependabot/cargo/structopt-0.2.18
-  c9a9bc94b Merge pull request #145 from mbrobbel/dependabot/cargo/backtrace-0.3.32
-  2fa468db8 Bump structopt from 0.2.17 to 0.2.18
-  0a2cf4d25 Bump backtrace from 0.3.31 to 0.3.32
-  868428388 Bump rand_chacha from 0.1.1 to 0.2.0
-  3493e78ec Merge pull request #144 from mbrobbel/dependabot/cargo/cbindgen-0.9.0
-  ed88bf767 Bump cbindgen from 0.8.7 to 0.9.0
-  caf82faf0 Merge pull request #142 from mbrobbel/dependabot/cargo/serde_derive-1.0.93
-  6acae9413 Bump serde_derive from 1.0.92 to 1.0.93
-  b9aa690bb Merge pull request #143 from mbrobbel/dependabot/cargo/chrono-0.4.7
-  08493b053 Bump chrono from 0.4.6 to 0.4.7
-  70e9aa9e8 Merge pull request #141 from mbrobbel/dependabot/cargo/serde-1.0.93
-  6bd9ebec9 Merge pull request #140 from mbrobbel/dependabot/cargo/backtrace-0.3.31
-  34c85c854 Bump serde from 1.0.92 to 1.0.93
-  4f875b281 Bump backtrace from 0.3.30 to 0.3.31
-  8db50ee02 Merge pull request #139 from mbrobbel/dependabot/cargo/remove_dir_all-0.5.2
-  9232917d2 Bump remove_dir_all from 0.5.1 to 0.5.2
-  98e7827d4 Merge pull request #138 from mbrobbel/dependabot/cargo/termion-1.5.3
-  bee50e430 Bump termion from 1.5.2 to 1.5.3
-  61d6ff40e Bump num-complex from 0.2.2 to 0.2.3 (#137)
-  55504df90 Bump num-complex from 0.2.1 to 0.2.2 (#135)
-  d32624c48 Bump smallvec from 0.6.9 to 0.6.10 (#134)
-  591b0b5c9 Bump byteorder from 1.3.1 to 1.3.2 (#133)
-  2946a8d90 Bump bitflags from 1.0.4 to 1.1.0 (#132)
-  ee44e8b57 Bump backtrace from 0.3.26 to 0.3.30 (#129)
-  5402dd658 Azure Pipeline: use rustup to update Rust on linux (#131)
-  9a2095bd1 Bump serde_derive from 1.0.91 to 1.0.92 (#123)
-  83c859ef2 Bump libc from 0.2.55 to 0.2.58 (#127)
-  b686399bb Bump structopt from 0.2.16 to 0.2.17 (#126)
-  70782b869 Bump serde from 1.0.91 to 1.0.92 (#124)
-  85f453581 Bump rustc-demangle from 0.1.14 to 0.1.15 (#121)
-  3236b9232 Bump structopt from 0.2.15 to 0.2.16 (#119)
-  9bf276dc6 Bump mio from 0.6.18 to 0.6.19 (#118)
-  e4555765d Bump num-traits from 0.2.7 to 0.2.8 (#117)
-  0e26b6139 Bump bincode from 1.1.3 to 1.1.4 (#114)
-  9aad4daf7 Bump num-integer from 0.1.39 to 0.1.41 (#116)
-  83bd6c802 Bump cfg-if from 0.1.7 to 0.1.9 (#113)
-  76f0ba3c3 Bump synstructure from 0.10.1 to 0.10.2 (#115)
-  f34fcafc5 Bump num-traits from 0.2.6 to 0.2.7 (#111)
-  2d669e36b Bump unicode-segmentation from 1.2.1 to 1.3.0 (#110)
-  fd652074e Bump autocfg from 0.1.2 to 0.1.4 (#109)
-  da3e04a18 Bump tempfile from 3.0.7 to 3.0.8 (#108)

## [0.0.3] - 2019-05-25

- c06f6d838 Bump mio from 0.6.16 to 0.6.18 (#102)
- fda4c1d54 Bump backtrace from 0.3.24 to 0.3.26 (#103)
- ba5cefff2 Fix launching plugins stored in working directory (#107)
- 6034dbc45 Fix --stderr/stdout flags not using friendly parse (#106)
- bdd53c51e Fix non-deterministic/asynchronous behavior visible through host arbs (#105)
- c800fd6f1 Fix potential spurious RunResponse message (#104)
- 46b45160d Bump backtrace from 0.3.22 to 0.3.24 (#101)
- 38b96d3f4 Bump backtrace from 0.3.20 to 0.3.22 (#98)
- 0ac11a089 Update to Rust 1.3.5 (#99)
- 2fd555327 Bump backtrace from 0.3.15 to 0.3.20 (#96)
- 0d92bcd6d Merge branch master of github.com:mbrobbel/dqcsim
- 70212be93 Fix C API docs, broken since 5ad3c42

## [0.0.2] - 2019-05-22

- 42adcceb7 Fix non-deterministic python test case
- b5f850e33 Third try to get pdoc3 and python working again
- 5100851fd Second try to get pdoc3 working again
- 941a01193 Remove dqcsim.tests from pdoc3 docs after failure
- 3b251ecc2 Python tests (#93)
- 809173e2e Add backtrace when PluginThread panics and don't panic in drop (#92)
- 67447342c Fixed links broken by 5f31e7
- 5f31e7fc4 Fix links on the docs front page not working
- f658b7e52 Fix host arb sync issue #90 (#91)
- e79a18d9c Update gatestream docs to closer represent reality
- d174813ef Fix azure-pipelines badge
- 5ad3c42bd Azure Pipelines (#88)
- fc58cb9c2 Update .gitignore
- 8f46c2745 Rename repository from dqcsim-rs to dqcsim
- 04168fe89 Prevent panic in LogThread introduced by #78
- 2724f3c5c Fix panic in drop method of PluginProcess (#86)
- f6406bd49 Add info log message with random seed (#87)
- 1c2be2d54 Proposed fix for #83 (#84)
- e486c315f Fix mdbook test trying to compile bash with rustc
- 2852417d0 Documentation updates (#80)
- 72b42726c Bump libc from 0.2.54 to 0.2.55 (#79)
- bf9b01535 Fix crates.io badge link
- 88885c6d5 Prevent LogThread panic if stderr can't be wrapped. (#78)
- 19bfedf06 Clean up manylinux release builds in container.
- b03f1bee7 Disable travis badge in cargo manifest. Crates.io does not support com tld yet.
- d348f4654 Add README.md to pypi

## [0.0.1] - 2019-05-16

- Test release

[0.0.1]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.1
[0.0.2]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.2
[0.0.3]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.3
[0.0.4]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.4
[0.0.5]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.5
[0.0.6]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.6
[0.0.7]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.7
[0.0.8]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.8
[0.0.9]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.9
[0.0.10]: https://github.com/mbrobbel/dqcsim/releases/tag/0.0.10
