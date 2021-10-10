
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::color::Color;

use std::error::Error;

pub struct Canvas {
    height: u32,
    width: u32,
    color: Color
}

impl Canvas {
    pub fn new(height: u32, width: u32, color: Color) -> Self {
        return Canvas{height: height, width: width, color: color};
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }
}

impl Shape for Canvas {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) -> Result<(), Box<dyn Error>> {
        return shape_visitor.visit_canvas(&self);
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Canvas height: {} wirth: {} color: {}",
                      self.height, self.width, self.color);
    }
}
