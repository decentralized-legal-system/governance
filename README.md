# Governance Layer

[![MIT licensed][mit-badge]][mit-url]
![Audit][audit-url]
![Clippy][clippy-url]

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/deepcausality-rs/deep_causality/blob/main/LICENSE

[audit-url]: https://github.com/deepcausality-rs/deep_causality/actions/workflows/audit.yml/badge.svg
[clippy-url]: https://github.com/deepcausality-rs/deep_causality/actions/workflows/rust-clippy.yml/badge.svg

[//]: # ([test-url]: https://github.com/deepcausality-rs/deep_causality/actions/workflows/run_tests.yml/badge.svg)

An exploration of building the governance layer on the IC blockhain.

## 🔥 Goals

1) Role: Provides the governing legal framework
2) Resolves: Combines legal and technical smart contracts that are both, legally enforceable through arbitration as well
   as technical enforceable through the enforcement layer.
3) Responsibilities:
   a. Creates, Reads, Updates, Deletes legal frameworks.
   b. Creates, Reads, Updates, Deletes contracts under a legal framework.
   c. Establishes amendment rules for legal frameworks and linked contracts.
4) Requires: None

## 🗺️ Concepts

The Governance layer, once deployed as a canister, aims to provide an API to programmatically deploy legal frameworks to
the blockchain. In its simplest form, a consensus jurisdiction consists of a DAO with a governing law attaches to it.
The Governing layer encodes the process of creating a new consensus jurisdiction as a sequence of steps that require the
details of the DAO as well as the governing law, as shown in figure 2. The difference to an ordinary DAO is that the
governing law of the DAO is written as private law legally enforceable through arbitration. Note, at this stage the
governing law serves as a frame of reference without technical implementation. Rather, the DAO will be created with a
link to the publishes governing law.

## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work:

* [clippy](https://github.com/rust-lang/rust-clippy)
* [nextest](https://nexte.st/)
* [outdated](https://github.com/kbknapp/cargo-outdated)
* [udeps](https://crates.io/crates/cargo-udeps)
* [audit](https://crates.io/crates/cargo-audit)

```bash 
    make build          Builds the code base incrementally (fast).
    make check          Checks the code base for security vulnerabilities.
    make fix            Auto-fixes linting issues as reported by cargo and clippy.
    make test           Runs all tests across all crates.
```

## 👷 Development

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd governance/
dfx help
dfx canister --help
```

## 🏠 Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, 
and will be run automatically any time you run `dfx deploy`.

## 👨‍💻👩‍💻 Contribution

Contributions are welcomed especially related to documentation, example code, and fixes.
If unsure where to start, open an issue and ask. For more significant code contributions,
please run make test and make check locally before opening a PR.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in deep_causality by you,
shall be licensed under the MIT license without additional terms or conditions.

## 📜 Licence

This project is licensed under the [MIT license](LICENSE).


## 💻 Author

* Marvin Hansen, [Emet-Labs](https://emet-labs.com/).
* Github GPG key ID: 369D5A0B210D39BC
* GPG Fingerprint: 4B18 F7B2 04B9 7A72 967E 663E 369D 5A0B 210D 39BC