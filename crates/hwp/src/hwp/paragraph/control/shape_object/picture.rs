use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    doc_info::{border_fill::BorderKind, bullet::Image},
    paragraph::control::{
        common_properties::CommonProperties,
        element_properties::{ArrowSize, ArrowStyle, ElementProperties, EndCap},
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

/// 그림
#[derive(Debug, Clone)]
pub struct PictureControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 내용
    pub content: PictureRecord,
}

impl PictureControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);
        let content = PictureRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PictureRecord {
    /// 테두리
    pub outline: PictureOutline,
    /// 도형의 영역
    pub rect: Rect,
    /// 자르기 한 후 사각형의 left
    pub left: i32,
    /// 자르기 한 후 사각형의 top
    pub top: i32,
    /// 자르기 한 후 사각형의 right
    pub right: i32,
    /// 자르기 한 후 사각형의 bottom
    pub bottom: i32,
    /// 왼쪽 여백
    pub margin_left: i16,
    // 오른쪽 여백
    pub margin_right: i16,
    /// 위쪽 여백
    pub margin_top: i16,
    /// 아래쪽 여백
    pub margin_bottom: i16,
    /// 이미지 정보
    pub image: Image,
    /// 문서 내 각 개체에 대한 고유 아이디(instance ID)
    pub instance_id: Option<u32>,
    /// 그림 추가정보
    pub additional_properties: Option<PictureAdditionalProperties>,
}

impl PictureRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_PICTURE as u32
        );

        let mut reader = record.get_data_reader();
        let mut outline = PictureOutline::from_reader(&mut reader);

        // NOTE: (@hahnlee) 문서에 정의된 순서와 다름
        let rect = Rect::from_reader(&mut reader);

        let left = reader.read_i32::<LittleEndian>().unwrap();
        let top = reader.read_i32::<LittleEndian>().unwrap();
        let right = reader.read_i32::<LittleEndian>().unwrap();
        let bottom = reader.read_i32::<LittleEndian>().unwrap();

        let margin_left = reader.read_i16::<LittleEndian>().unwrap();
        let margin_right = reader.read_i16::<LittleEndian>().unwrap();
        let margin_top = reader.read_i16::<LittleEndian>().unwrap();
        let margin_bottom = reader.read_i16::<LittleEndian>().unwrap();

        let image = Image::from_reader(&mut reader);

        outline.alpha = reader.read_u8().unwrap();

        let instance_id = if record.size as u64 > reader.position() {
            Some(reader.read_u32::<LittleEndian>().unwrap())
        } else {
            None
        };

        if record.size as u64 > reader.position() {
            PictureEffect::from_reader(&mut reader);
        }

        let additional_properties = if record.size as u64 > reader.position() {
            Some(PictureAdditionalProperties::from_reader(&mut reader))
        } else {
            None
        };

        assert_eq!(reader.position(), record.size as u64);

        Self {
            outline,
            rect,
            left,
            right,
            top,
            bottom,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            image,
            instance_id,
            additional_properties,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PictureOutline {
    /// 선 색상
    pub color: ColorRef,
    /// 선 굵기
    pub width: u32,
    /// 선 종류
    pub kind: BorderKind,
    /// 선 끝 모양
    pub end_cap: EndCap,
    /// 화살표 시작 모양
    pub head_style: ArrowStyle,
    /// 화살표 끝 모양
    pub tail_style: ArrowStyle,
    /// 화살표 시작 크기
    pub head_size: ArrowSize,
    /// 화살표 끝 크기
    pub tail_size: ArrowSize,
    /// 시작부분 화살표 채움 여부
    pub head_fill: bool,
    /// 끝부분 화살표 채움 여부
    pub tail_fill: bool,
    /// 투명도
    pub alpha: u8,
}

// TODO: (@hahnlee) 모델 합치기
impl PictureOutline {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        // NOTE: (@hahnlee) 문서와 사이즈가 다름
        let width = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let kind = BorderKind::from_u32(get_value_range(attribute, 0, 5)).unwrap();
        let end_cap = EndCap::from_u32(get_value_range(attribute, 6, 9)).unwrap_or(EndCap::Flat);
        let head_style = ArrowStyle::from_u32(get_value_range(attribute, 10, 15)).unwrap();
        let tail_style = ArrowStyle::from_u32(get_value_range(attribute, 16, 21)).unwrap();
        let head_size = ArrowSize::from_u32(get_value_range(attribute, 22, 25)).unwrap();
        let tail_size = ArrowSize::from_u32(get_value_range(attribute, 26, 29)).unwrap();
        let head_fill = get_flag(attribute, 30);
        let tail_fill = get_flag(attribute, 31);

        Self {
            color,
            width,
            kind,
            end_cap,
            head_style,
            tail_style,
            head_size,
            tail_size,
            head_fill,
            tail_fill,
            alpha: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub left_top: Point,
    pub right_top: Point,
    pub right_bottom: Point,
    pub left_bottom: Point,
}

impl Rect {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            left_top: Point::from_reader(reader),
            right_top: Point::from_reader(reader),
            right_bottom: Point::from_reader(reader),
            left_bottom: Point::from_reader(reader),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            x: reader.read_i32::<LittleEndian>().unwrap(),
            y: reader.read_i32::<LittleEndian>().unwrap(),
        }
    }
}

pub struct PictureEffect {
    /// 그림자
    pub shadow: Option<Shadow>,
    /// 네온
    pub glow: Option<Glow>,
    /// 부드러운 가장자리
    pub soft_edge: Option<SoftEdge>,
    /// 반사
    pub reflection: Option<Reflection>,
}

impl PictureEffect {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let attribute = reader.read_u32::<LittleEndian>().unwrap();

        let shadow = if get_flag(attribute, 0) {
            Some(Shadow::from_reader(reader))
        } else {
            None
        };

        let glow = if get_flag(attribute, 1) {
            Some(Glow::from_reader(reader))
        } else {
            None
        };

        let soft_edge = if get_flag(attribute, 2) {
            Some(SoftEdge::from_reader(reader))
        } else {
            None
        };

        let reflection = if get_flag(attribute, 3) {
            Some(Reflection::from_reader(reader))
        } else {
            None
        };

        Self {
            shadow,
            glow,
            soft_edge,
            reflection,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shadow {
    /// 그림자 스타일
    pub style: ShadowStyle,
    /// 그림자 투명도
    pub alpha: f32,
    /// 그림자 흐릿하게
    pub radius: f32,
    /// 방향
    pub direction: f32,
    /// 거리
    pub distance: f32,
    /// 정렬
    pub align: AlignStyle,
    /// 기울기 각도(X)
    pub skew_x: f32,
    /// 기울기 각도(Y)
    pub skew_y: f32,
    /// 확대 비율(X)
    pub scale_x: f32,
    /// 확대 비율(Y)
    pub scale_y: f32,
    /// 도형과 함께 그림자 회전
    pub rotation: bool,
    /// 색상
    pub color: EffectColor,
}

impl Shadow {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            style: ShadowStyle::from_i32(reader.read_i32::<LittleEndian>().unwrap()).unwrap(),
            alpha: reader.read_f32::<LittleEndian>().unwrap(),
            radius: reader.read_f32::<LittleEndian>().unwrap(),
            direction: reader.read_f32::<LittleEndian>().unwrap(),
            distance: reader.read_f32::<LittleEndian>().unwrap(),
            align: AlignStyle::from_i32(reader.read_i32::<LittleEndian>().unwrap()).unwrap(),
            skew_x: reader.read_f32::<LittleEndian>().unwrap(),
            skew_y: reader.read_f32::<LittleEndian>().unwrap(),
            scale_x: reader.read_f32::<LittleEndian>().unwrap(),
            scale_y: reader.read_f32::<LittleEndian>().unwrap(),
            rotation: reader.read_i32::<LittleEndian>().unwrap() > 0,
            color: EffectColor::from_reader(reader),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ShadowStyle {
    Outside,
    Inside,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum AlignStyle {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct EffectColor {
    /// 색상타입
    pub kind: EffectColorKind,
    /// 값
    pub value: EffectColorValue,
    /// 색상효과
    pub color_effects: Vec<EffectColorEffect>,
}

impl EffectColor {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = EffectColorKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();
        let value = match kind {
            EffectColorKind::RGB => {
                EffectColorValue::RGB(reader.read_u32::<LittleEndian>().unwrap())
            }
            EffectColorKind::CMYK => {
                EffectColorValue::CMYK(reader.read_u32::<LittleEndian>().unwrap())
            }
            EffectColorKind::Scheme => EffectColorValue::Scheme(
                reader.read_f32::<LittleEndian>().unwrap(),
                reader.read_f32::<LittleEndian>().unwrap(),
                reader.read_f32::<LittleEndian>().unwrap(),
            ),
            EffectColorKind::System => EffectColorValue::Scheme(
                reader.read_f32::<LittleEndian>().unwrap(),
                reader.read_f32::<LittleEndian>().unwrap(),
                reader.read_f32::<LittleEndian>().unwrap(),
            ),
        };

        let count = reader.read_u32::<LittleEndian>().unwrap();
        let mut color_effects = vec![];
        for _ in 0..count {
            color_effects.push(EffectColorEffect::from_reader(reader));
        }

        Self {
            kind,
            value,
            color_effects,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum EffectColorKind {
    RGB,
    CMYK,
    Scheme,
    System,
}

#[derive(Debug, Clone)]
pub enum EffectColorValue {
    RGB(u32),
    CMYK(u32),
    Scheme(f32, f32, f32),
    System(f32, f32, f32),
}

#[derive(Debug, Clone, FromPrimitive)]
pub enum EffectColorEffectKind {
    Alpha,
    AlphaMod,
    AlphaOff,
    Red,
    RedMod,
    RedOff,
    Green,
    GreenMod,
    GreenOff,
    Blue,
    BlueMod,
    BlueOff,
    Hue,
    HueMod,
    HueOff,
    Sat,
    SatMod,
    SatOff,
    Lum,
    LumMod,
    LumOff,
    Shade,
    Tint,
    Gray,
    Comp,
    Gamma,
    InvGamma,
    Inv,
}

#[derive(Debug, Clone)]
pub struct EffectColorEffect {
    /// 색상 효과 종류
    pub kind: EffectColorEffectKind,
    /// 색상 효과 값
    pub value: f32,
}

impl EffectColorEffect {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            kind: EffectColorEffectKind::from_u32(reader.read_u32::<LittleEndian>().unwrap())
                .unwrap(),
            value: reader.read_f32::<LittleEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Glow {
    /// 네온 투명도
    pub alpha: f32,
    /// 네온 반경
    pub radius: f32,
    /// 색상
    pub color: EffectColor,
}

impl Glow {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            alpha: reader.read_f32::<LittleEndian>().unwrap(),
            radius: reader.read_f32::<LittleEndian>().unwrap(),
            color: EffectColor::from_reader(reader),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoftEdge {
    /// 부드러운 가장자리 반경
    pub radius: f32,
}

impl SoftEdge {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            radius: reader.read_f32::<LittleEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reflection {
    /// 반사 스타일
    pub align: AlignStyle,
    /// 반경
    pub radius: f32,
    /// 방향
    pub direction: f32,
    /// 거리
    pub distance: f32,
    /// 기울기 각도(X)
    pub skew_x: f32,
    /// 기울기 각도(Y)
    pub skew_y: f32,
    /// 확대 비율(X)
    pub scale_x: f32,
    /// 확대 비율(Y)
    pub scale_y: f32,
    /// 도형과 함께 그림자 회전
    pub rotation: bool,
    /// 시작 투명도
    pub start_alpha: f32,
    /// 시작 위치
    pub start_position: f32,
    /// 끝 투명도
    pub end_alpha: f32,
    /// 끝 위치
    pub end_position: f32,
    /// 오프셋 방향
    pub offset_direction: f32,
}

impl Reflection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            align: AlignStyle::from_i32(reader.read_i32::<LittleEndian>().unwrap()).unwrap(),
            radius: reader.read_f32::<LittleEndian>().unwrap(),
            direction: reader.read_f32::<LittleEndian>().unwrap(),
            distance: reader.read_f32::<LittleEndian>().unwrap(),
            skew_x: reader.read_f32::<LittleEndian>().unwrap(),
            skew_y: reader.read_f32::<LittleEndian>().unwrap(),
            scale_x: reader.read_f32::<LittleEndian>().unwrap(),
            scale_y: reader.read_f32::<LittleEndian>().unwrap(),
            rotation: reader.read_i32::<LittleEndian>().unwrap() > 0,
            start_alpha: reader.read_f32::<LittleEndian>().unwrap(),
            start_position: reader.read_f32::<LittleEndian>().unwrap(),
            end_alpha: reader.read_f32::<LittleEndian>().unwrap(),
            end_position: reader.read_f32::<LittleEndian>().unwrap(),
            offset_direction: reader.read_f32::<LittleEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PictureAdditionalProperties {
    /// 그림 최초 생성 시 기준 이미지 너비
    pub width: u32,
    /// 그림 최초 생성 시 기준 이미지 높이
    pub height: u32,
    /// 이미지 투명도
    pub alpha: u8,
}

impl PictureAdditionalProperties {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            width: reader.read_u32::<LittleEndian>().unwrap(),
            height: reader.read_u32::<LittleEndian>().unwrap(),
            alpha: reader.read_u8().unwrap(),
        }
    }
}
