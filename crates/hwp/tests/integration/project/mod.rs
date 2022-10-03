use hwp::HWP;
use std::fs;

use crate::utils::get_tests_path;

#[test]
fn check_hello_world() {
    let path = get_tests_path("integration/project/files/hello_world.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);
    assert_eq!(
        hwp.body_texts.sections[0].paragraphs[0].to_string(),
        "Hello World!"
    );
}