use candid_parser::Principal;
use convert_case::{Case, Casing};
use serde::Deserialize;
use std::{collections::HashMap, env, ffi::OsStr, fs, io, path::PathBuf};

#[derive(Deserialize)]
struct CanId {
    ic: Principal,
}

fn read_candid_ids() -> io::Result<HashMap<String, CanId>> {
    let can_ids_file = fs::File::open("did/canister_ids.json")?;
    let reader = io::BufReader::new(can_ids_file);
    Ok(serde_json::from_reader(reader).expect("invalid candid ids"))
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=did/*");

    let can_ids = read_candid_ids()?;

    let mut candid_config = candid_parser::bindings::rust::Config::new();
    candid_config.set_target(candid_parser::bindings::rust::Target::Agent);
    let mut did_mod_contents = String::new();

    // create $OUT_DIR/did
    let out_dir = env::var("OUT_DIR").unwrap();
    let did_dir = PathBuf::from(&out_dir).join("did");
    fs::create_dir_all(&did_dir)?;

    // Auto generate bindings for did files
    for didinfo in fs::read_dir("did")? {
        let didpath = didinfo?.path();
        if didpath.extension() != Some(OsStr::new("did")) {
            continue;
        }
        let file_name = didpath.file_stem().unwrap().to_str().unwrap();

        // compile bindings from did
        candid_config.set_canister_id(
            can_ids
                .get(file_name)
                .unwrap_or_else(|| panic!("no canister id for {file_name}"))
                .ic,
        );
        let service_name: String = file_name.to_case(Case::Pascal);
        candid_config.set_service_name(service_name);
        let (type_env, actor) = candid_parser::pretty_check_file(&didpath).unwrap_or_else(|e| {
            panic!(
                "invalid did file: {}, err: {e}",
                didpath.as_os_str().to_string_lossy()
            )
        });
        let bindings = candid_parser::bindings::rust::compile(&candid_config, &type_env, &actor);

        // write bindings to $OUT_DIR/did/<did file>.rs
        let mut binding_file = did_dir.clone();
        binding_file.push(file_name);
        binding_file.set_extension("rs");
        fs::write(&binding_file, bindings)?;

        // #[path = "$OUT_DIR/did/<did file>.rs"] pub mod <did file>;
        did_mod_contents.push_str(&format!(
            "#[path = \"{}\"] pub mod {};\n",
            binding_file.to_string_lossy(),
            file_name
        ));
    }

    // create mod file for did bindings
    // ideally i'd like to manually write this file in src/canister/mod.rs
    // but can't, due to https://github.com/rust-lang/rfcs/issues/752
    let binding_mod_file = PathBuf::from(&out_dir).join("did").join("mod.rs");
    fs::write(binding_mod_file, did_mod_contents)?;
    Ok(())
}
