//! Build Injective proto files. This build script clones the CosmosSDK and Injective version
//! specified in the COSMOS_SDK_REV and INJECTIVE_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build_injective::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.47.1";
const WASMD_REV: &str = "v0.45.1";

/// The injective-core commit or tag to be cloned and used to build the proto files
const INJECTIVE_REV: &str = "v1.14.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../injective-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the injective-core submodule is located
const INJECTIVE_DIR: &str = "../../dependencies/injective-core/";
/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../../dependencies/wasmd/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "./tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_submodule(INJECTIVE_DIR, INJECTIVE_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let injective_project = CosmosProject {
        name: "injective".to_string(),
        version: INJECTIVE_REV.to_string(),
        project_dir: INJECTIVE_DIR.to_string(),
        include_mods: vec![
            "auction".to_string(),
            "exchange".to_string(),
            "insurance".to_string(),
            "oracle".to_string(),
            "peggy".to_string(),
            "permissions".to_string(),
            "tokenfactory".to_string(),
            "wasmx".to_string(),
        ],
    };
    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        include_mods: vec![
            "auth".to_string(),
            "authz".to_string(),
            "bank".to_string(),
            "base".to_string(),
            "distribution".to_string(),
            "feegrant".to_string(),
            "gov".to_string(),
            "params".to_string(),
            "staking/v1beta1/genesis.proto".to_string(),
            "staking/v1beta1/staking.proto".to_string(),
            "staking/v1beta1/tx.proto".to_string(),
        ],
    };
    let wasmd_project = CosmosProject {
        name: "cosmwasm".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        include_mods: vec!["wasm".to_string()],
    };

    let injective_code_generator = CodeGenerator::new(out_dir, tmp_build_dir, injective_project, vec![cosmos_project, wasmd_project]);

    injective_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
