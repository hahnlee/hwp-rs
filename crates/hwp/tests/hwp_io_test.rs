use hwp::HWP;
use std::{path::PathBuf, fs};

fn get_tests_path(sub_path: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push(&sub_path);
    path
}

#[test]
fn check_body_size() {
    let path = get_tests_path("files/hello_world.hwp");
    let file = fs::read(path).unwrap();
    let hwp = HWP::from_bytes(file);
    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.body.sections.len(), 1);
}
