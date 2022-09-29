pub fn get_value(bits: u32, start: u32, end: u32) -> u32 {
    let target = bits >> start;

    let mut mask = 0;
    for _ in start..(end - start) {
        mask <<= 1;
        mask += 1;
    }

    return target & mask;
}

pub fn get_flag(bits: u32, position: u32) -> bool {
    let mask = 1 << position;
    return (bits & mask) == mask;
}
