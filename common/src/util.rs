use std::{
    env,
    path::{Path, PathBuf},
};

pub fn from_crate_root(relative_path: &str) -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let full_path = Path::new(&manifest_dir).join(relative_path);
    full_path
}
