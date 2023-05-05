---
title: Account Compression Program
---

This on-chain program provides an interface for composing smart-contracts to create and use SPL ConcurrentMerkleTrees. The primary application of using SPL ConcurrentMerkleTrees is to make edits to off-chain data with on-chain verification.

## Motivation

-  The high throughput of the Solana blockchain has increased the creation of Non fungible assets i.e NFTs due to their custodial ownership and censorship resistance characteristics. However, the practical use cases of these NFTs are limited by network storage costs when these are created at scale. It's rather inexpensive to mint a single non fungible token, however as you increase the quantity the cost of storing the asset's data on-chain becomes uneconomical.

- To fix this we must ensure the cost per token is as close to zero as possible. The solution is to store a compressed hash of the asset data on chain while maintaining the actual data off chain in a database. The program provides a way to verify the off chain data on chain and also make concurrent writes to the data. In order to do this we introduced a new data structure called a Concurrent Merkle Tree that avoids proof collision while making concurrent writes.


## Background

The account compression program is currently being used for the [Metaplex Bubblegum Program](https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/)

To solve the problem of the high on-chain storage cost per unit of these assets, we need to store a compressed fingerprint on-chain that can verify the off-chain asset data. To do this we need
  - Concurrent Merkle Trees
    - The concurrent merkle trees allow us to compress all the data into a single root hash stored on-chain while allowing concurrent replacements and appends to the data.
  - Program indexer
    - The indexer is in charge of indexing the latest writes to the tree on chain so you know which nodes have been replaced and which have been appended to so you can avoid proof collision
  - Off-chain Database
    - The db stores the actual asset data on chain as we are only storing the merkle root on chain and we need to be able to verify the data on chain.

The crux of this is the concurrent merkle tree and we shall learn about it in the next section.

## Source

The Account Compression Program's source is available on
[github](https://github.com/solana-labs/solana-program-library).


## Interface
The Account Compression Program is written in rust and also has a typescript sdk for interacting with the program.

### Rust Packages
| Name                         | Description                                                                        | Program                                                                                                                       |
| ---------------------------- | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `solarti-account-compression`    | SDK for interacting with account compression program                               | [Rust Crate](https://crates.io/crates/solarti-account-compression) and [Rust Docs](https://docs.rs/solarti-account-compression)       |
| `solarti-noop`                   | SDK for interacting with no op program, primarily for circumventing log truncation | [Rust Crate](https://crates.io/crates/solarti-noop) and [Rust Docs](https://docs.rs/solarti-noop)                                     |
| `solarti-concurrent-merkle-tree` | SDK for creating SPL ConcurrentMerkleTrees                                         | [Rust Crate](https://crates.io/crates/solarti-concurrent-merkle-tree) and [Rust Docs](https://docs.rs/solarti-concurrent-merkle-tree) |

### TypeScript Packages
| Name                              | Description                                          | Package                                                              |
| --------------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------- |
| `@solarti/solarti-account-compression` | SDK for interacting with account compression program | [NPM](https://www.npmjs.com/package/@solarti/solarti-account-compression) |

## Testing and Development

Testing contracts locally requires the SDK to be built. 

With a built local SDK, the test suite can be run with:

1. `yarn link @solarti/solarti-account-compression`
2. `yarn`
3. `yarn test`
