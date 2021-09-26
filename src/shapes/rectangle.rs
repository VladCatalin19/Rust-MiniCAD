
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Rectangle {
    top_left: Point,
    height: u32,
    width: u32,
    outline_color: Color,
    fill_color: Color
}

impl Rectangle {
    pub fn new(top_left: Point, height: u32, width: u32, outline_color: Color, fill_color: Color)
    -> Rectangle {
        return Rectangle{top_left: top_left, height: height, width: width,
                         outline_color: outline_color, fill_color: fill_color};
    }

    pub fn get_top_left(&self) -> Point {
        return self.top_left;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Rectangle {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_rectangle(&self);
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle top left point: {} height: {} width: {} outline color: {} fill color: {}",
                      self.top_left, self.height, self.width,
                      self.outline_color, self.fill_color);
    }
}
