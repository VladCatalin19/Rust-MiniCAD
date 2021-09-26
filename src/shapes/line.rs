
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Line {
    p0: Point,
    p1: Point,
    color: Color
}

impl Line {
    pub fn new(p0: Point, p1: Point, color: Color) -> Line {
        return Line{p0: p0, p1: p1, color: color};
    }

    pub fn get_p0(&self) -> Point {
        return self.p0;
    }

    pub fn get_p1(&self) -> Point {
        return self.p1;
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }
}

impl Shape for Line {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_line(&self);
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Line p0: {} p1: {} color: {}",
                      self.p0, self.p1, self.color);
    }
}
