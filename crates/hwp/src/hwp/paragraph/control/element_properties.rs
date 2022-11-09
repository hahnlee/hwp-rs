use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    doc_info::border_fill::{BorderKind, Fill},
    record::{tags::BodyTextRecord, RecordCursor},
    utils::bits::{get_flag, get_value_range},
};

type TransformationMatrix = [[f64; 3]; 3];

/// 개체 요소 속성
#[derive(Debug, Clone)]
pub struct ElementProperties {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 개체가 속한 그룹 내에서의 X offset
    pub offset_x: i32,
    /// 개체가 속한 그룹 내에서의 Y offset
    pub offset_y: i32,
    /// 몇 번이나 그룹 되었는지
    pub group_level: u16,
    /// 개체 요소의 local file version
    pub local_file_version: u16,
    /// 개체 생성 시 초기 폭
    pub original_width: u32,
    /// 개체 생성 시 초기 높이
    pub original_height: u32,
    /// 개체의 현재 폭
    pub current_width: u32,
    /// 개체의 현재 높이
    pub current_height: u32,
    /// 좌우로 뒤집여진 상태인지 여부
    pub horizontal_flip: bool,
    /// 상하로 뒤집여진 상태인지 여부
    pub vertical_flip: bool,
    /// 회전각
    pub angle: i16,
    /// 회전 중심의 x 좌표(개체 좌표계)
    pub center_x: i32,
    /// 회전 중심의 y 좌표(개체 좌표계)
    pub center_y: i32,
    /// 이동 행렬
    pub translation_matrix: TransformationMatrix,
    /// 크기 변환행렬 모음
    pub scale_matrices: Vec<TransformationMatrix>,
    /// 회전 변환행렬 모음
    pub rotation_matrices: Vec<TransformationMatrix>,
    /// 자식 ID (컨테이너에서만 사용)
    pub children_ids: Option<Vec<u32>>,
    /// HWPX의 inst_id 속성과 같은 값 (컨테이너에서만 사용)
    pub instance_id: Option<u32>,
    /// 테두리선 정보
    pub outline: Option<Outline>,
    /// 채우기 정보
    pub fill: Option<Fill>,
    /// 그림자 정보
    pub shadow: Option<Shadow>,
}

impl ElementProperties {
    pub fn from_record_cursor(cursor: &mut RecordCursor, from_gso: bool) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, BodyTextRecord::HWPTAG_SHAPE_COMPONENT as u32);

        let mut reader = record.get_data_reader();
        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();
        if from_gso {
            assert_eq!(ctrl_id, reader.read_u32::<LittleEndian>().unwrap());
        }

        let offset_x = reader.read_i32::<LittleEndian>().unwrap();
        let offset_y = reader.read_i32::<LittleEndian>().unwrap();
        let group_level = reader.read_u16::<LittleEndian>().unwrap();
        let local_file_version = reader.read_u16::<LittleEndian>().unwrap();
        let original_width = reader.read_u32::<LittleEndian>().unwrap();
        let original_height = reader.read_u32::<LittleEndian>().unwrap();
        let current_width = reader.read_u32::<LittleEndian>().unwrap();
        let current_height = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let horizontal_flip = get_flag(attribute, 0);
        let vertical_flip = get_flag(attribute, 1);

        let angle = reader.read_i16::<LittleEndian>().unwrap();
        let center_x = reader.read_i32::<LittleEndian>().unwrap();
        let center_y = reader.read_i32::<LittleEndian>().unwrap();

        let count = reader.read_u16::<LittleEndian>().unwrap();
        let translation_matrix = read_transformation_matrix(&mut reader);

        let mut scale_matrices = vec![];
        let mut rotation_matrices = vec![];
        for _ in 0..count {
            scale_matrices.push(read_transformation_matrix(&mut reader));
            rotation_matrices.push(read_transformation_matrix(&mut reader));
        }

        // 컨테이너 컨트롤 추가 정보
        let mut children_ids = None;
        let mut instance_id = None;
        if ctrl_id == make_4chid!('$', 'c', 'o', 'n') && record.size as u64 > reader.position() {
            let count = reader.read_u16::<LittleEndian>().unwrap();
            let mut ids = vec![];
            for _ in 0..count {
                ids.push(reader.read_u32::<LittleEndian>().unwrap());
            }
            children_ids = Some(ids);
            instance_id = Some(reader.read_u32::<LittleEndian>().unwrap());
            assert_eq!(record.size as u64, reader.position());
        }

        let outline = if record.size as u64 > reader.position() {
            Some(Outline::from_reader(&mut reader, ctrl_id))
        } else {
            None
        };

        let fill = if record.size as u64 > reader.position() {
            Some(Fill::from_reader(&mut reader))
        } else {
            None
        };

        let shadow = if record.size as u64 > reader.position() {
            Some(Shadow::from_reader(&mut reader))
        } else {
            None
        };

        assert_eq!(record.size as u64, reader.position());

        Self {
            ctrl_id,
            offset_x,
            offset_y,
            group_level,
            local_file_version,
            original_width,
            original_height,
            current_width,
            current_height,
            horizontal_flip,
            vertical_flip,
            angle,
            center_x,
            center_y,
            translation_matrix,
            scale_matrices,
            rotation_matrices,
            children_ids,
            instance_id,
            outline,
            fill,
            shadow,
        }
    }
}

/// 테두리선 정보
#[derive(Debug, Clone)]
pub struct Outline {
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
    /// 스타일
    pub style: OutlineStyle,
}

impl Outline {
    pub fn from_reader<T: Read>(reader: &mut T, ctrl_id: u32) -> Self {
        let color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        // NOTE: (@hahnlee) 문서와 사이즈가 다름
        let width = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let kind = BorderKind::from_u32(get_value_range(attribute, 0, 5)).unwrap();
        let default_cap = if ctrl_id == make_4chid!('$', 'p', 'i', 'c') {
            EndCap::Round
        } else {
            EndCap::Flat
        };
        let end_cap = EndCap::from_u32(get_value_range(attribute, 6, 9)).unwrap_or(default_cap);
        let head_style = ArrowStyle::from_u32(get_value_range(attribute, 10, 15)).unwrap();
        let tail_style = ArrowStyle::from_u32(get_value_range(attribute, 16, 21)).unwrap();
        let head_size = ArrowSize::from_u32(get_value_range(attribute, 22, 25)).unwrap();
        let tail_size = ArrowSize::from_u32(get_value_range(attribute, 26, 29)).unwrap();
        let head_fill = get_flag(attribute, 30);
        let tail_fill = get_flag(attribute, 31);

        let style = OutlineStyle::from_u8(reader.read_u8().unwrap()).unwrap();

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
            style,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum EndCap {
    Round,
    Flat,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum OutlineStyle {
    Normal,
    Outer,
    Inner,
}

/// 화살표 모양
/// NOTE: 창모양은 문서에 누락되어있음. (HWPX참고)
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ArrowStyle {
    /// 모양 없음
    None,
    /// 화살 모양
    Arrow,
    /// 창 모양
    Spear,
    /// 오목한 화살모양
    ConcaveArrow,
    /// 속이 빈 다이아몬드 모양
    EmptyDiamond,
    /// 속이 빈 원 모양
    EmptyCircle,
    /// 속이 빈 사각 모양
    EmptyBox,
    /// 속이 채워진 다이아몬드 모양
    FilledDiamond,
    /// 속이 채워진 원 모양
    FilledCircle,
    /// 속이 채워진 사각 모양
    FieldBox,
}

/// 화살표 사이즈
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ArrowSize {
    /// 작은-작은
    SmallSmall,
    /// 작은-중간
    SmallMedium,
    /// 작은-큰
    SmallLarge,
    /// 중간-작은
    MediumSmall,
    /// 중간-중간
    MediumMedium,
    /// 중간-큰
    MediumLarge,
    /// 큰-작은
    LargeSmall,
    /// 큰-중간
    LargeMedium,
    /// 큰-큰
    LargeLarge,
}

#[derive(Debug, Clone)]
pub struct Shadow {
    /// 그림자 종류
    pub kind: ShadowKind,
    /// 그림자 색상
    pub color: ColorRef,
    /// 그림자 간격 X
    pub offset_x: i32,
    /// 그림자 간격 Y
    pub offset_y: i32,
    /// 그림자 간격 투명도
    pub alpha: u8,
    /// 알 수 없는 바이트
    pub unknown: [u8; 5],
}

impl Shadow {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = ShadowKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        let color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        let offset_x = reader.read_i32::<LittleEndian>().unwrap();
        let offset_y = reader.read_i32::<LittleEndian>().unwrap();

        let mut unknown = [0u8; 5];
        reader.read_exact(&mut unknown).unwrap();

        let alpha = reader.read_u8().unwrap();

        Self {
            kind,
            color,
            offset_x,
            offset_y,
            unknown,
            alpha,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ShadowKind {
    /// 없음
    None,
    /// 왼쪽 위
    LeftTop,
    /// 오른쪽 위
    RightTop,
    /// 왼쪽 아래
    LeftBottom,
    /// 오른쪽 아래
    RightBottom,
    /// 왼쪽 뒤
    LeftBack,
    /// 오른쪽 뒤
    RightBack,
    /// 왼쪽 앞
    LeftFront,
    /// 오른쪽 앞
    RightFront,
    /// 작게
    Small,
    /// 크게
    Large,
}

fn read_transformation_matrix<T: Read>(reader: &mut T) -> TransformationMatrix {
    [
        [
            reader.read_f64::<LittleEndian>().unwrap(),
            reader.read_f64::<LittleEndian>().unwrap(),
            reader.read_f64::<LittleEndian>().unwrap(),
        ],
        [
            reader.read_f64::<LittleEndian>().unwrap(),
            reader.read_f64::<LittleEndian>().unwrap(),
            reader.read_f64::<LittleEndian>().unwrap(),
        ],
        [0.0, 0.0, 1.0],
    ]
}
