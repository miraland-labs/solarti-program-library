# Token Swap Program

A Uniswap-like exchange for the Token program on the Miraland blockchain.

Full documentation is available at https://spl.miraland.top/token-swap

JavaScript bindings are available in the `./js` directory.

## Audit

The repository [README](https://github.com/miraland-labs/solarti-program-library#audits)
contains information about program audits.

## Building master

To build a development version of the Token Swap program, you can use the normal
build command for Miraland programs:

```sh
cargo build-sbf
```

## Testing

### Unit tests

Run unit tests from `./program/` using:

```sh
cargo test
```

### Fuzz tests

Using the Rust version of `honggfuzz`, we "fuzz" the Token Swap program every night.
Install `honggfuzz` with:

```sh
cargo install honggfuzz
```

From there, run fuzzing from `./program/fuzz` with:

```sh
cargo hfuzz run token-swap-instructions
```

If the program crashes or errors, `honggfuzz` dumps a `.fuzz` file in the workspace,
so you can debug the failing input using:

```sh
cargo hfuzz run-debug token-swap-instructions hfuzz_workspace/token-swap-instructions/*fuzz
```

This command attaches a debugger to the test, allowing you to easily see the
exact problem.

### Integration tests

You can test the JavaScript bindings and on-chain interactions using
`miraland-test-validator`, included in the Miraland Tool Suite.  See the
[CLI installation instructions](https://docs.miraland.top/cli/install-miraland-cli-tools).

From `./js`, install the required modules:

```sh
npm i
```

Then run all tests:

```sh
npm run start-with-test-validator
```
