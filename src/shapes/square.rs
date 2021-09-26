
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Square {
    top_left: Point,
    side: u32,
    outline_color: Color,
    fill_color: Color
}

impl Square {
    pub fn new(top_left: Point, side: u32,
           outline_color: Color, fill_color: Color)
        -> Square {
        return Square{top_left: top_left, side: side,
                      outline_color: outline_color, fill_color: fill_color};
    }

    pub fn get_top_left(&self) -> Point {
        return self.top_left;
    }

    pub fn get_width(&self) -> u32 {
        return self.side;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Square {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_square(&self);
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Square top left point: {} side: {} outline color: {} fill color: {}",
                      self.top_left, self.side,
                      self.outline_color, self.fill_color);
    }
}
