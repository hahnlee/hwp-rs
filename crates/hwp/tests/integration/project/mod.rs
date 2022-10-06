use hwp::HWP;
use std::fs;

use crate::utils::get_tests_path;

#[test]
fn check_hello_world() {
    let path = get_tests_path("integration/project/files/hello_world.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

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

#[test]
fn check_range_tags() {
    let path = get_tests_path("integration/project/files/range.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

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

    assert_eq!(hwp.body_texts.sections[0].paragraphs[0].range_tags.len(), 3);
}


#[test]
fn check_bookmark() {
    let path = get_tests_path("integration/project/files/bookmark.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);
}

#[test]
fn check_over_type() {
    let path = get_tests_path("integration/project/files/over_type.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);
}


#[test]
fn check_dutmal() {
    let path = get_tests_path("integration/project/files/dutmal.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);
}