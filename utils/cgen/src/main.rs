extern crate cbindgen;

use std::env;
use std::path::Path;

fn token<P: AsRef<Path>>(crate_dir: P) {
    let output_file = crate_dir.as_ref().join("inc/token.h");
    println!("Generating {}", output_file.display());

    let config = cbindgen::Config {
        header: Some("/* Autogenerated Solarti Token program C Bindings */".to_string()),
        language: cbindgen::Language::C,
        line_length: 80,
        style: cbindgen::Style::Both,
        tab_width: 4,
        cpp_compat: true,
        pragma_once: true,
        enumeration: cbindgen::EnumConfig {
            prefix_with_name: true,
            ..cbindgen::EnumConfig::default()
        },
        export: cbindgen::ExportConfig {
            prefix: Some("Token_".to_string()),
            include: vec![
                "TokenInstruction".to_string(),
                "Mint".to_string(),
                "Account".to_string(),
                "Multisig".to_string(),
            ],
            exclude: vec!["DECIMALS".to_string()],
            ..cbindgen::ExportConfig::default()
        },
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Some(vec!["solana-program".to_string(), "solana-sdk".to_string()]),
            ..cbindgen::ParseConfig::default()
        },
        ..cbindgen::Config::default()
    };
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file(output_file);
}

fn token_swap<P: AsRef<Path>>(crate_dir: P) {
    let output_file = crate_dir.as_ref().join("inc/token-swap.h");
    println!("Generating {}", output_file.display());

    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file(output_file);
}

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&cargo_manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    token(&workspace_root.join("token/program"));
    token_swap(&workspace_root.join("token-swap/program"));
}
