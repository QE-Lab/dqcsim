# Release

This chapter documents the release procedure of DQCsim. A release consists of a git tag object and an accompanying GitHub release. Build artifacts for the tag reference are generated and published by the [Release](https://github.com/mbrobbel/dqcsim/actions?query=workflow%3ARelease) workflow.

## Prepare

- Create a new branch from an up-to-date `master` branch: `git checkout master`, `git pull origin master` and `git checkout -b release-1.2.3`.
- Update version number in `rust/Cargo.toml`. Run `cargo check` in the root of the repository to reflect the version update in the `Cargo.lock` file.
- Update version number in the project command in `CMakeLists.txt`.
- Add new entry to `CHANGELOG.md`. To list all commits since the previous tag reference use: `git log $(git describe --tags --abbrev=0)..HEAD --oneline | xargs -L1 echo`.
- Stage the release commit by adding modified files and committing: `git add rust/Cargo.toml Cargo.lock CMakeLists.txt CHANGELOG.md` and `git commit -m "Release 1.2.3"`.
- Push the branch and create a pull request on GitHub.

## Release

- Merge the pull request with reviewers approval and all checks passed.
- Update your local repository: `git checkout master` and `git pull origin master`.
- Create the tag object: `git tag -s 1.2.3`. Add the `CHANGELOG.md` entry for this version as tag message.
- Push the tag object: `git push origin 1.2.3`.

## Validate

- The release workflow is triggered by pushing the tag object. The workflow creates a GitHub release and uploads the generated artifacts to [PyPi](https://pypi.org/) and [Crates.io](https://crates.io/). Validate the workflow passes without errors.
