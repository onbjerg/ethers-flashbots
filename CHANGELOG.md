# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

## [0.8.1]

A small patch to fix the documentation on [docs.rs](https://docs.rs).

## [0.8.0]

### Added

- The basefee for a simulated block can now be specified.

## [0.7.0]

- Updated to ethers `^0.6.0`

## [0.6.0]

### Changed

- Relaxed version requirement for Ethers - version requirement is now `^0.5.0`.
- Disabled default Ethers features to allow for building on Windows (which lacks IPC support, see https://github.com/gakonst/ethers-rs/issues/393)

## [0.5.0]

### Added

- Revert reason is now parsed, if there is any.

### Fixed

- `value` on a simulated transaction was incorrectly assumed to be
  the amount of Ether sent in a transaction. It is now correctly
  parsed as `Bytes`, since it represents the return data (if any)
  of the transaction.

## [0.4.0]

### Changed

- Parameters are now validated before bundles are sent to the relay.
  Check [the documentation](https://docs.rs/ethers-flashbots/0.4.0/ethers_flashbots/enum.FlashbotsMiddlewareError.html#variant.MissingParameters) for more information.
- Bumped ethers to 0.5.1

### Added

- Added a helper to get the effective gas price of bundles and
  bundle transactions (`SimulatedBundle::effective_gas_price` and `SimulatedTransaction::effective_gas_price`).

## [0.3.1]

### Added

- Added a way to get stats about bundles (`FlashbotsMiddleware::get_bundle_stats`)
- Added a getter for the bundle hash of a pending bundle
- Added a way to get stats about your searcher identity (`FlashbotsMiddleware::get_user_stats`)

## [0.3.0]

### Fixes

- If your bundle contains a transaction that deploys a contract,
  the `SimulatedTransaction` will now have a destination (`to`) of
  `None` to distinguish this from the zero address.

## [0.2.2]

### Fixes

- Handle non-JSONRPC errors from the Flashbots relay

## [0.2.1]

### Fixes

- Correctly serializes bundle requests

## [0.2.0]

**NOTE**: This release is unfortunately broken, please update to [0.2.1]

### Added

- You can now wait for bundle inclusions (see `PendingBundle`).
- Added `BundleRequest::transaction_hashes`

## [0.1.3]

**NOTE**: This release is unfortunately broken, please update to [0.2.1]

### Added

- Added a revert reason to simulated transactions

## [0.1.2]

**NOTE**: This release is unfortunately broken, please update to [0.2.1]

### Changed

- Pinned ethers dependencies to specific versions

## [0.1.1]

**NOTE**: This release is unfortunately broken, please update to [0.2.1]

Initial release.

<!-- next-url !-->
[Unreleased]: https://github.com/onbjerg/ethers-flashbots/compare/{{tag_name}}...HEAD
[0.8.1]: https://github.com/onbjerg/ethers-flashbots/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/onbjerg/ethers-flashbots/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/onbjerg/ethers-flashbots/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/onbjerg/ethers-flashbots/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/onbjerg/ethers-flashbots/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/onbjerg/ethers-flashbots/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/onbjerg/ethers-flashbots/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/onbjerg/ethers-flashbots/compare/97dc88a0...0.1.1
