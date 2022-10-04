use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub build_number: u8,
}

impl Version {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        let [build_number, micro, minor, major] = bytes;

        // TODO: (@hahnlee) 각 숫자 검증하기
        Self {
            major,
            minor,
            micro,
            build_number,
        }
    }

    pub fn from_str(version: &str) -> Self {
        let version: Vec<u8> = version
            .split(".")
            .map(|v| v.parse::<u8>().unwrap())
            .collect();

        if version.len() != 4 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        // TODO: (@hahnlee) 각 숫자 검증하기
        Self {
            major: version[0],
            minor: version[1],
            micro: version[2],
            build_number: version[3],
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.micro, self.build_number
        )
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [self.build_number, self.micro, self.minor, self.major]
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.lt(other) {
            return Some(Ordering::Less);
        }

        Some(Ordering::Greater)
    }

    fn lt(&self, other: &Version) -> bool {
        if self.major != other.major {
            return self.major < other.major;
        }

        if self.minor != other.minor {
            return self.minor < other.minor;
        }

        if self.micro != other.micro {
            return self.micro < other.micro;
        }

        return self.build_number < other.build_number;
    }

    fn le(&self, other: &Version) -> bool {
        return self.lt(other) || self.eq(other);
    }

    fn gt(&self, other: &Version) -> bool {
        return !self.lt(other) && !self.eq(other);
    }

    fn ge(&self, other: &Version) -> bool {
        return self.gt(other) || self.eq(other);
    }
}

#[cfg(test)]
mod tests {
    use crate::hwp::version::Version;

    #[test]
    fn test_version_compare() {
        let left = Version::from_str("5.1.2.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left == right, true);
        assert_eq!(left >= right, true);
        assert_eq!(left <= right, true);

        let left = Version::from_str("6.1.2.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left > right, true);
        assert_eq!(left >= right, true);

        let left = Version::from_str("5.2.2.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left > right, true);
        assert_eq!(left >= right, true);

        let left = Version::from_str("5.1.3.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left > right, true);
        assert_eq!(left >= right, true);

        let left = Version::from_str("5.1.2.4");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left > right, true);
        assert_eq!(left >= right, true);

        let left = Version::from_str("4.2.3.5");
        let right = Version::from_str("5.1.2.4");
        assert_ne!(left > right, true);
        assert_ne!(left >= right, true);

        let left = Version::from_str("4.1.2.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left < right, true);
        assert_eq!(left <= right, true);

        let left = Version::from_str("5.0.2.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left < right, true);
        assert_eq!(left <= right, true);

        let left = Version::from_str("5.1.0.3");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left < right, true);
        assert_eq!(left <= right, true);

        let left = Version::from_str("5.1.2.2");
        let right = Version::from_str("5.1.2.3");
        assert_eq!(left < right, true);
        assert_eq!(left <= right, true);
    }
}
