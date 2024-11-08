# Token Lending program

A lending protocol for the Token program on the Miraland blockchain inspired by Aave and Compound.

Full documentation is available at https://spl.miraland.io/token-lending

Web3 bindings are available in the `./js` directory.

## Audit

The repository [README](https://github.com/miraland-labs/miraland-program-library#audits)
contains information about program audits.

### On-chain programs

| Cluster | Program Address                                                                                                                                    |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Devnet  | [`6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH`](https://explorer.miraland.io/address/6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH?cluster=devnet) |

### Documentation

-   [CLI docs](https://github.com/miraland-labs/miraland-program-library/tree/master/token-lending/cli)
-   [Client library docs](https://miraland-labs.github.io/miraland-program-library/token-lending/)

### Deploy a lending program (optional)

This is optional! You can skip these steps and use the [Token Lending CLI](./cli/README.md) with one of the on-chain programs listed above to create a lending market and add reserves to it.

1. [Install the Miraland CLI](https://docs.miraland.io/cli/install-miraland-cli-tools)

1. Install the Token and Token Lending CLIs:

    ```shell
    cargo install solarti-token-cli
    cargo install solarti-token-lending-cli
    ```

1. Clone the SPL repo:

    ```shell
    git clone https://github.com/miraland-labs/miraland-program-library.git
    ```

1. Go to the new directory:

    ```shell
    cd miraland-program-library
    ```

1. Generate a keypair for yourself:

    ```shell
    miraland-keygen new -o owner.json

    # Wrote new keypair to owner.json
    # ================================================================================
    # pubkey: JAgN4SZLNeCo9KTnr8EWt4FzEV1UDgHkcZwkVtWtfp6P
    # ================================================================================
    # Save this seed phrase and your BIP39 passphrase to recover your new keypair:
    # your seed words here never share them not even with your mom
    # ================================================================================
    ```

    This pubkey will be the owner of the lending market that can add reserves to it.

1. Generate a keypair for the program:

    ```shell
    miraland-keygen new -o lending.json

    # Wrote new keypair to lending.json
    # ============================================================================
    # pubkey: 6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH
    # ============================================================================
    # Save this seed phrase and your BIP39 passphrase to recover your new keypair:
    # your seed words here never share them not even with your mom
    # ============================================================================
    ```

    This pubkey will be your Program ID.

1. Open `./token-lending/program/src/lib.rs` in your editor. In the line

    ```rust
    miraland_program::declare_id!("6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH");
    ```

    replace the Program ID with yours.

1. Build the program binaries:

    ```shell
    cargo build
    cargo build-sbf
    ```

1. Prepare to deploy to devnet:

    ```shell
    miraland config set --url https://api.devnet.miraland.io
    ```

1. Score yourself some sweet MLN:

    ```shell
    miraland airdrop -k owner.json 2
    miraland airdrop -k owner.json 2
    miraland airdrop -k owner.json 2
    ```

    You'll use this for transaction fees, rent for your program accounts, and initial reserve liquidity. If you run
    into issues with the airdrop command, see the [docs](https://docs.miraland.io/cli/transfer-tokens#airdrop-some-tokens-to-get-started) for more info.

1. Deploy the program:

    ```shell
    miraland program deploy \
      -k owner.json \
      --program-id lending.json \
      target/deploy/spl_token_lending.so

    # Program Id: 6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH
    ```

    If the deployment doesn't succeed, follow [this guide](https://docs.miraland.io/cli/deploy-a-program#resuming-a-failed-deploy) to resume it.

1. Wrap some of your MLN as an Solarti Token:

    ```shell
    solarti-token wrap \
       --fee-payer owner.json \
       2.0 \
       -- owner.json

    # Wrapping 2 MLN into AJ2sgpgj6ZeQazPPiDyTYqN9vbj58QMaZQykB9Sr6XY
    ```

    You'll use this for initial reserve liquidity. Note the Solarti Token account pubkey (e.g. `AJ2sgpgj6ZeQazPPiDyTYqN9vbj58QMaZQykB9Sr6XY`).

1. Use the [Token Lending CLI](./cli/README.md) to create a lending market and add reserves to it.
