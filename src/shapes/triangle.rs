
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Triangle {
    p0: Point,
    p1: Point,
    p2: Point,
    outline_color: Color,
    fill_color: Color
}

impl Triangle {
    fn new(p0: Point, p1: Point, p2: Point, outline_color: Color, fill_color: Color)
    -> Triangle {
        return Triangle{p0: p0, p1: p1, p2: p2,
                        outline_color: outline_color, fill_color: fill_color};
    }

    pub fn get_p0(&self) -> Point {
        return self.p0;
    }

    pub fn get_p1(&self) -> Point {
        return self.p1;
    }

    pub fn get_p2(&self) -> Point {
        return self.p2;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Triangle {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_triangle(&self);
    }
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Triangle p0: {} p1: {} p2: {} outline color: {} fill color: {}",
                      self.p0, self.p1, self.p2,
                      self.outline_color, self.fill_color);
    }
}
