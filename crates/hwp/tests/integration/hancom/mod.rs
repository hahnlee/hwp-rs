use hwp::HWP;
use std::fs;

use crate::utils::get_tests_path;

#[test]
fn check_hwp_document_format() {
    let path = get_tests_path("integration/hancom/files/한글문서파일형식_5.0_revision1.3.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, true);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    // 배포용 문서는 ViewTexts가 있어야 한다
    assert_eq!(hwp.view_texts.is_some(), true);

    // TODO: (@hahnlee) 정보 채우기
}

#[test]
fn check_hwp_equation_format() {
    let path = get_tests_path("integration/hancom/files/한글문서파일형식_수식_revision1.3.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, true);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    // 배포용 문서는 ViewTexts가 있어야 한다
    assert_eq!(hwp.view_texts.is_some(), true);

    // TODO: (@hahnlee) 정보 채우기
}
