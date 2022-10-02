/// 개체 요소 속성
#[derive(Debug)]
pub struct ElementProperties {}

impl ElementProperties {}

// NT32 4 개체가 속한 그룹 내에서의 X offset
// INT32 4 개체가 속한 그룹 내에서의 Y offset
// WORD 2 몇 번이나 그룹 되었는지
// WORD 2 개체 요소의 local file version
// UINT32 4 개체 생성 시 초기 폭
// UINT32 4 개체 생성 시 초기 높이
// UINT32 4 개체의 현재 폭
// UINT32 4 개체의 현재 높이
// UINT32 4
// 속성
// 값 설명
// 0 horz flip
// 1 vert flip
// HWPUNIT16 2 회전각
// INT32 4 회전 중심의 x 좌표(개체 좌표계)
// INT32 4 회전 중심의 y 좌표(개체 좌표계)
// n Rendering 정보(표 79 참조)