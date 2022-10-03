use std::ops::{AddAssign, BitAnd, Shl, ShlAssign, Shr};

use num::{one, zero, Integer};

pub fn get_value_range<T: Integer>(bits: T, start: T, end: T) -> T
where
    T: Shl<Output = T> + Shr<Output = T> + ShlAssign + AddAssign + BitAnd<Output = T> + Copy,
    std::ops::Range<T>: IntoIterator,
{
    let target = bits >> start;

    let mut mask = zero::<T>();
    for _ in start..(end + num::one()) {
        mask <<= one::<T>();
        mask += one::<T>();
    }

    return target & mask;
}

pub fn get_value<T: Integer>(bits: T, start: T) -> T
where
    T: Shl<Output = T> + Shr<Output = T> + ShlAssign + AddAssign + BitAnd<Output = T> + Copy,
    std::ops::Range<T>: IntoIterator,
{
    return get_value_range(bits, start, start);
}

pub fn get_flag<T: Integer>(bits: T, position: T) -> bool
where
    T: Shl<Output = T> + BitAnd<Output = T> + Copy,
{
    let mask = one::<T>() << position;
    return (bits & mask) == mask;
}
