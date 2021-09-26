
#[derive(Copy, Clone)]
pub struct Color {
    pub r: i8,
    pub g: i8,
    pub b: i8,
    pub a: i8
}

impl Color {
    pub fn new(r: i8, g: i8, b: i8, a: i8) -> Self {
        return Color{r: r, g: g, b: b, a: a};
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "#{:X}{:X}{:X} {}", self.r, self.g, self.b, self.a);
    }
}
