
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};

pub struct Circle {
    center: Point,
    radius: u32,
    outline_color: Color,
    fill_color: Color
}

impl Circle {
    pub fn new(center: Point, radius: u32,
           outline_color: Color,
           fill_color: Color)
    -> Self {

        return Circle{center: center, radius: radius,
                      outline_color: outline_color, fill_color: fill_color};
    }

    pub fn get_center(&self) -> Point {
        return self.center;
    }

    pub fn get_radius(&self) -> u32 {
        return self.radius;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Circle {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_circle(&self);
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle center: {} radius: {} outline color: {} fill color: {}",
                      self.center, self.radius, self.outline_color, self.fill_color);
    }
}
