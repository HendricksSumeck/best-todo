use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut out_dir: ::std::path::PathBuf =
        env::var_os("CARGO_MANIFEST_DIR")
            .unwrap()
            .into()
        ;
    out_dir.push("target");
    out_dir.push(env::var_os("PROFILE").unwrap());

    let dest_path = Path::new(&out_dir).join("config.json");

    let src_path = Path::new("config.json");

    fs::copy(src_path, dest_path).expect("Falha ao copiar o arquivo");
}