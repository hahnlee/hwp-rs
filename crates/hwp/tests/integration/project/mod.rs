use hwp::{
    hwp::doc_info::border_fill::{FillKind, GradationKind, PatternKind},
    HWP,
};
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

#[test]
fn check_color_fill() {
    let path = get_tests_path("integration/project/files/color_fill.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);

    let border_fill = hwp.doc_info.id_mappings.border_fills.last().unwrap();
    assert_eq!(border_fill.fill.kind, FillKind::Color);

    let color_fill = border_fill.fill.as_color_fill().unwrap();
    assert_eq!(color_fill.alpha, 0);
    assert_eq!(color_fill.pattern_kind, PatternKind::Vertical);
}

#[test]
fn check_gradation_fill() {
    let path = get_tests_path("integration/project/files/gradation_fill.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);

    let border_fill = hwp.doc_info.id_mappings.border_fills.last().unwrap();
    assert_eq!(border_fill.fill.kind, FillKind::Gradation);

    let gradation_fill = border_fill.fill.as_gradation_fill().unwrap();
    assert_eq!(gradation_fill.kind, GradationKind::Conical);
    assert_eq!(gradation_fill.angle, 30);
    assert_eq!(gradation_fill.center_x, 40);
    assert_eq!(gradation_fill.center_y, 20);
    assert_eq!(gradation_fill.step, 255);
    assert_eq!(gradation_fill.step_center, 50);
    assert_eq!(gradation_fill.alpha, 0);
}


#[test]
fn check_outline() {
    let path = get_tests_path("integration/project/files/outline.hwp");
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
fn check_shadow() {
    let path = get_tests_path("integration/project/files/shadow.hwp");
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
fn check_draw_text() {
    let path = get_tests_path("integration/project/files/draw_text.hwp");
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
fn check_image_fill() {
    let path = get_tests_path("integration/project/files/image_fill.hwp");
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
fn check_group_with_draw_text() {
    let path = get_tests_path("integration/project/files/group_with_draw_text.hwp");
    let file = fs::read(path).unwrap();

    let hwp = HWP::from_bytes(&file);

    assert_eq!(hwp.header.version.to_string(), "5.1.0.1");
    assert_eq!(hwp.header.flags.compressed, true);
    assert_eq!(hwp.header.flags.distributed, false);

    assert_eq!(hwp.header.license.ccl, false);
    assert_eq!(hwp.header.license.replication_restrictions, false);

    assert_eq!(hwp.body_texts.sections.len(), 1);
}