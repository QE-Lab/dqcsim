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
