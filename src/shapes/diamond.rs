
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Diamond {
    center: Point,
    horizontal_diagonal: u32,
    vertical_diagonal: u32,
    outline_color: Color,
    fill_color: Color
}

impl Diamond {
    pub fn new(center: Point, horizontal_diagonal: u32, vertical_diagonal: u32,
           outline_color: Color, fill_color: Color)
    -> Self {
        return Diamond{center: center,
                       horizontal_diagonal: horizontal_diagonal,
                       vertical_diagonal: vertical_diagonal, 
                       outline_color: outline_color,
                       fill_color: fill_color};
    }

    pub fn get_center(&self) -> Point {
        return self.center;
    }

    pub fn get_horizontal_diagonal(&self) -> u32 {
        return self.horizontal_diagonal;
    }

    pub fn get_vertical_diagonal(&self) -> u32 {
        return self.vertical_diagonal;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Diamond {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_diamond(&self);
    }
}

impl std::fmt::Display for Diamond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Diamond center: {} horizontal diagonal: {} vertical diagonal: {} outlone color: {} fill color: {}",
                      self.center, self.horizontal_diagonal, self.vertical_diagonal,
                      self.outline_color, self.fill_color);
    }
}
