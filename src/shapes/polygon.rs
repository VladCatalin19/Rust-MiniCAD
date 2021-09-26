
use crate::shapes::shape::Shape;
use crate::shape_visitor::shape_visitor::ShapeVisitor;
use crate::utils::{point::Point, color::Color};
use std::vec::Vec;
use std::string::String;

pub struct Polygon {
    points: Vec<Point>,
    outline_color: Color,
    fill_color: Color
}

impl Polygon {
    pub fn new(points: Vec<Point>, outline_color: Color, fill_color: Color) -> Polygon {
        return Polygon{points: points, outline_color: outline_color, fill_color: fill_color};
    }

    pub fn get_points(&self) -> &Vec<Point> {
        return &self.points;
    }

    pub fn get_outline_color(&self) -> Color {
        return self.outline_color;
    }

    pub fn get_fill_color(&self) -> Color {
        return self.fill_color;
    }
}

impl Shape for Polygon {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) {
        shape_visitor.visit_polygon(&self);
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut points_strings: Vec<String> = Vec::new();
        points_strings.reserve_exact(self.points.len());

        for point in &self.points {
            points_strings.push(point.to_string());
        }

        return write!(f, "Polygon points: {} outline color: {} fill color: {}",
                      points_strings.join(", "), self.outline_color, self.fill_color);
    }
}
