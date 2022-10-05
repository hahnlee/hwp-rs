use hwp::HWP;
use std::fs;

use crate::utils::get_tests_path;

#[test]
fn check_annual_report() {
    let path = get_tests_path("integration/naver_documents/files/annual_report.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.0.2.2");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);

    // TODO: (@hahnlee) 정보 채우기
}

#[test]
fn check_work_report() {
    let path = get_tests_path("integration/naver_documents/files/work_report.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.0.2.4");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);

    // TODO: (@hahnlee) 정보 채우기
}
