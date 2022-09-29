/// MS Visual C++의 seed-random 함수의 구현체
///
/// [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator) 방식의 의사 랜덤함수로
/// 패턴의 예측가능성이 높으니, 배포용문서 복호화를 제외한 암호화 구현에 사용하지 말것
///
/// [stack overflow](https://stackoverflow.com/questions/6793065/understanding-the-algorithm-of-visual-cs-rand-function)
pub struct SRand {
    rand_state: u32,
}

impl SRand {
    pub fn new(seed: u32) -> SRand {
        SRand { rand_state: seed }
    }

    pub fn rand(&mut self) -> i32 {
        self.rand_state = self.rand_state.wrapping_mul(214013).wrapping_add(2531011);
        return ((self.rand_state >> 16) as i32) & 0x7FFF;
    }
}
