use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("problems.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let src_dir = Path::new(&manifest_dir).join("src");
    let mut modules = Vec::new();

    if let Ok(entries) = fs::read_dir(&src_dir) {
        entries.flatten().for_each(|entry| {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
                && file_name.starts_with("lc")
                && file_name.ends_with(".rs")
            {
                let module_name = &file_name[..file_name.len() - 3];
                modules.push(module_name.to_string());
            }
        });
    }

    modules.sort();

    let module_declarations: String = modules
        .iter()
        .map(|module| {
            let full_path = src_dir.join(format!("{}.rs", module));
            format!("#[path = \"{}\"]\npub mod {};", full_path.display(), module)
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(&dest_path, module_declarations).unwrap();

    println!("cargo:rerun-if-changed=src/");
}
