# Governance Layer

An exploration of building the governance layer on the IC blockhain.

## Goals

1) Role: Provides the governing legal framework
2) Resolves: Combines legal and technical smart contracts that are both, legally enforceable through arbitration as well
   as technical enforceable through the enforcement layer.
3) Responsibilities:
   a. Creates, Reads, Updates, Deletes legal frameworks.
   b. Creates, Reads, Updates, Deletes contracts under a legal framework.
   c. Establishes amendment rules for legal frameworks and linked contracts.
4) Requires: None

## Concepts

The Governance layer, once deployed as a canister, aims to provide an API to programmatically deploy legal frameworks to
the blockchain. In its simplest form, a consensus jurisdiction consists of a DAO with a governing law attaches to it.
The Governing layer encodes the process of creating a new consensus jurisdiction as a sequence of steps that require the
details of the DAO as well as the governing law, as shown in figure 2. The difference to an ordinary DAO is that the
governing law of the DAO is written as private law legally enforceable through arbitration. Note, at this stage the
governing law serves as a frame of reference without technical implementation. Rather, the DAO will be created with a
link to the publishes governing law.

## Development

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd governance/
dfx help
dfx canister --help
```

## Running the project locally

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

