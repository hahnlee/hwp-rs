use std::path::PathBuf;

pub fn get_tests_path(sub_path: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push(&sub_path);
    path
}
