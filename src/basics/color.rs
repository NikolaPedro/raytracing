use std::string::ToString;

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { 
        Self { r, g, b } 
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}