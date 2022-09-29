#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub build_number: u8,
}

impl Version {
    pub fn from_byte(byte: [u8; 4]) -> Self {
        let [build_number, micro, minor, major] = byte;

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
            // TODO: (@hahnlee) 에러 던지기
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
