use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

use super::bullet::Image;

#[derive(Debug)]
pub struct BorderFill {
    /// 3D 효과의 유무
    pub effect_3d: bool,
    /// 그림자 효과의 유무
    pub effect_shadow: bool,
    /// Slash 대각선 모양
    pub slash_diagonal_shape: SlashDiagonalShape,
    /// BackSlash 대각선 모양
    pub back_slash_diagonal_shape: BackSlashDiagonalShape,
    /// Slash 대각선 꺾은선 여부
    pub broken_slash_diagonal_line: bool,
    /// BackSlash 대각선 꺾은선 여부
    pub broken_back_slash_diagonal_line: bool,
    /// Slash 대각선 모양 180도 회전 여부
    pub slack_diagonal_line_rotated: bool,
    /// BackSlash 대각선 모양 180도 회전 여부
    pub back_slack_diagonal_line_rotated: bool,
    /// 중심선 유무
    pub center_line: bool,
    /// 선 정보
    pub borders: [Border; 4],
    /// 대각선
    pub diagonal_border: Border,
    /// 채우기 정보
    pub fill: Fill,
}

impl FromRecordCursor for BorderFill {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_BORDER_FILL as u32);

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u16::<LittleEndian>().unwrap();
        let effect_3d = get_flag(attribute, 0);
        let effect_shadow = get_flag(attribute, 1);
        let slash_diagonal_shape =
            SlashDiagonalShape::from_u16(get_value_range(attribute, 2, 4)).unwrap();
        let back_slash_diagonal_shape =
            BackSlashDiagonalShape::from_u16(get_value_range(attribute, 5, 7)).unwrap();
        let broken_slash_diagonal_line = if get_value_range(attribute, 8, 9) > 0 {
            true
        } else {
            false
        };
        let broken_back_slash_diagonal_line = get_flag(attribute, 10);
        let slack_diagonal_line_rotated = get_flag(attribute, 11);
        let back_slack_diagonal_line_rotated = get_flag(attribute, 11);
        let center_line = get_flag(attribute, 13);

        // NOTE: (@hahnlee) 공식문서와 순서가 다르다
        let borders = [
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
        ];

        let diagonal_border = Border::from_reader(&mut reader);

        let fill = Fill::from_reader(&mut reader);

        Self {
            effect_3d,
            effect_shadow,
            slash_diagonal_shape,
            back_slash_diagonal_shape,
            broken_slash_diagonal_line,
            broken_back_slash_diagonal_line,
            slack_diagonal_line_rotated,
            back_slack_diagonal_line_rotated,
            center_line,
            borders,
            diagonal_border,
            fill,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum SlashDiagonalShape {
    None = 0b000,
    Slash = 0b010,
    LeftTopToBottomEdge = 0b011,
    LeftTopToRightEdge = 0b110,
    LeftTopToBottomRightEdge = 0b111,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum BackSlashDiagonalShape {
    None = 0b000,
    BackSlash = 0b010,
    RightTopToBottomEdge = 0b011,
    RightTopToLeftEdge = 0b110,
    RightTopToBottomLeftEdge = 0b111,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum BorderKind {
    /// 실선
    Solid,
    /// 긴 점선
    Dash,
    /// 점선
    Dot,
    /// -.-.-.-.
    DashDot,
    /// -..-..-..
    DashDotDot,
    /// Dash보다 긴 선분의 반복
    LongDash,
    /// Dot보다 큰 동그라미의 반복
    Circle,
    /// 2중선
    DoubleSlim,
    /// 가는선 + 굵은선 2중선
    SlimThick,
    /// 굵은선 + 가는선 2중선
    TickSlim,
    /// 가는선 + 굵은선 + 가는선 3중선
    SlimTickSlim,
    /// 물결
    Wave,
    /// 물결 2중선
    DoubleWave,
    /// 두꺼운 3D
    Tick3D,
    /// 두꺼운 3D(광원 반대)
    Tick3DInset,
    /// 3D 단선
    Slim3D,
    /// 3D 단선(광원 반대)
    Slim3DInset,
}

#[derive(Debug, Clone)]
pub struct Border {
    pub width: u8,
    pub kind: BorderKind,
    pub color: ColorRef,
}

impl Border {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            kind: BorderKind::from_u8(reader.read_u8().unwrap()).unwrap(),
            width: reader.read_u8().unwrap(),
            color: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fill {
    /// 채우기 종류
    pub kind: FillKind,
    /// 채우기 내용
    pub content: FillContent,
}

impl Fill {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = FillKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        let content = match kind {
            FillKind::Color => FillContent::Color(ColorFill::from_reader(reader)),
            FillKind::Gradation => FillContent::Gradation(GradationFill::from_reader(reader)),
            FillKind::Image => FillContent::Image(ImageFill::from_reader(reader)),
            FillKind::None => {
                // NOTE: (@hahnlee) 추가정보의 길이, 항상 0이다
                assert_eq!(reader.read_u32::<LittleEndian>().unwrap(), 0);
                FillContent::None(())
            }
        };

        Self { kind, content }
    }

    pub fn as_color_fill(&self) -> Result<&ColorFill, ()> {
        match &self.content {
            FillContent::Color(color) => Ok(color),
            _ => Err(()),
        }
    }

    pub fn as_gradation_fill(&self) -> Result<&GradationFill, ()> {
        match &self.content {
            FillContent::Gradation(gradation) => Ok(gradation),
            _ => Err(()),
        }
    }

    pub fn as_image_fill(&self) -> Result<&ImageFill, ()> {
        match &self.content {
            FillContent::Image(image) => Ok(image),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum FillKind {
    /// 채우기 없음
    None = 0x00000000,
    /// 단색 채우기
    Color = 0x00000001,
    /// 이미지 채우기
    Image = 0x00000002,
    /// 그라데이션 채우기
    Gradation = 0x00000004,
}

#[derive(Debug, Clone)]
pub enum FillContent {
    None(()),
    Color(ColorFill),
    Gradation(GradationFill),
    Image(ImageFill),
}

#[derive(Debug, Clone)]
pub struct ColorFill {
    /// 배경색
    pub background_color: ColorRef,
    /// 무늬색
    pub pattern_color: ColorRef,
    /// 무늬종류
    pub pattern_kind: PatternKind,
    /// 투명도
    pub alpha: u8,
}

impl ColorFill {
    fn from_reader<T: Read>(reader: &mut T) -> Self {
        let background_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let pattern_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let pattern_kind =
            PatternKind::from_i32(reader.read_i32::<LittleEndian>().unwrap() + 1).unwrap();

        // NOTE: (@hahnlee) HWPX에 정의되어 있음
        let alpha = reader.read_u8().unwrap();

        // NOTE: (@hahnlee) 추가정보의 길이, 여기서는 무시한다
        assert_eq!(reader.read_u32::<LittleEndian>().unwrap(), 0);

        Self {
            background_color,
            pattern_color,
            pattern_kind,
            alpha,
        }
    }
}

/// 채우기 무늬 종류
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum PatternKind {
    /// 없음
    None,
    /// - - - -
    Horizontal,
    /// |||||
    Vertical,
    /// \\\\\
    BackSlash,
    /// /////
    Slash,
    /// +++++
    Cross,
    /// xxxxx
    CrossDiagonal,
}

#[derive(Debug, Clone)]
pub struct GradationFill {
    /// 그러데이션 유형
    pub kind: GradationKind,
    /// 그러데이션의 기울임(시작 각)
    pub angle: u32,
    /// 그러데이션의 가로 중심(중심 X 좌표)
    pub center_x: u32,
    /// 그러데이션의 세로 중심(중심 Y 좌표)
    pub center_y: u32,
    /// 그러데이션 번짐 정도
    pub step: u32,
    /// 색상이 바뀌는 곳의 위치
    pub change_points: Vec<u32>,
    /// 색상
    pub colors: Vec<ColorRef>,
    /// 번짐 정도의 중심
    pub step_center: u8,
    /// 투명도
    pub alpha: u8,
}

impl GradationFill {
    /// NOTE: (@hahnlee) 전체적으로 문서 오류가 있어 바이트가 다르다
    fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = GradationKind::from_u8(reader.read_u8().unwrap()).unwrap();
        let angle = reader.read_u32::<LittleEndian>().unwrap();
        let center_x = reader.read_u32::<LittleEndian>().unwrap();
        let center_y = reader.read_u32::<LittleEndian>().unwrap();
        let step = reader.read_u32::<LittleEndian>().unwrap();
        let count = reader.read_u32::<LittleEndian>().unwrap();
        let mut change_points = vec![];
        if count > 2 {
            change_points.push(reader.read_u32::<LittleEndian>().unwrap());
        }
        let mut colors = vec![];
        for _ in 0..count {
            colors.push(ColorRef::from_u32(
                reader.read_u32::<LittleEndian>().unwrap(),
            ));
        }

        // NOTE: (@hahnlee) 추가정보 개수, 항상 1이다
        assert_eq!(reader.read_u32::<LittleEndian>().unwrap(), 1);

        let step_center = reader.read_u8().unwrap();
        let alpha = reader.read_u8().unwrap();

        Self {
            kind,
            angle,
            center_x,
            center_y,
            step,
            change_points,
            colors,
            step_center,
            alpha,
        }
    }
}

/// 그러데이션 유형
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum GradationKind {
    /// 줄무늬형
    Linear = 1,
    /// 원형
    Radial = 2,
    /// 원뿔형
    Conical = 3,
    /// 사각형
    Square = 4,
}

#[derive(Debug, Clone)]
pub struct ImageFill {
    /// 이미지 채우기 유형
    pub kind: ImageFillKind,
    /// 이미지 정보
    pub image: Image,
    /// 문서에 미정의된 값
    pub unknown: Vec<u8>,
}

impl ImageFill {
    fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = ImageFillKind::from_u8(reader.read_u8().unwrap()).unwrap();
        let image = Image::from_reader(reader);

        // NOTE: (@hahnlee) 추가정보 개수, 항상 0이다
        assert_eq!(reader.read_u32::<LittleEndian>().unwrap(), 0);

        let mut unknown = vec![];
        reader.read_to_end(&mut unknown).unwrap();

        Self {
            kind,
            image,
            unknown,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ImageFillKind {
    /// 바둑판식으로-모두
    Tile,
    /// 바둑판식으로-가로/위
    TileHorizontalTop,
    /// 바둑판식으로-가로/아래
    TileHorizontalBottom,
    /// 바둑판식으로-세로/왼쪽
    TileVerticalLeft,
    /// 바둑판식으로-세로/오른쪽
    TileVerticalRight,
    /// 크기에 맞추어
    Total,
    /// 가운데로
    Center,
    /// 가운데 위로
    CenterTop,
    /// 가운데 아래로
    CenterBottom,
    /// 왼쪽 가운데로
    CenterLeft,
    /// 왼쪽 위로
    LeftTop,
    /// 왼쪽 아래로
    LeftBottom,
    /// 오른쪽 가운데로
    RightCenter,
    /// 오른쪽 위로
    RightTop,
    /// 오른쪽 아래로
    RightBottom,
    /// NONE
    None,
}
