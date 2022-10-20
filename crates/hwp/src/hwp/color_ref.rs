use super::utils::bits::get_value_range;

#[derive(Debug, Clone)]
pub struct ColorRef {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl ColorRef {
    pub fn from_u32(number: u32) -> Self {
        let red = get_value_range(number, 0, 7);
        let blue = get_value_range(number, 8, 15);
        let green = get_value_range(number, 16, 23);

        Self { red, blue, green }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.blue, self.green)
    }
}
