## Release Instructions

- Update version in `Cargo.toml`
- Move "Unreleased" section in [CHANGELOG.md](./CHANGELOG.md) into a header with the new version, and add a link at the bottom of the CHANGELOG to compares that version to HEAD
- Ensure tests and lints pass
- Commit those changes and tag that commit with the version number (**no `v` prefix!**)
- Push the changes and run `cargo publish`
