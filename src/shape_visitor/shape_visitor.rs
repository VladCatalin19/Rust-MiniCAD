
use crate::shapes::{canvas::Canvas,
                    line::Line,
                    triangle::Triangle,
                    square::Square,
                    rectangle::Rectangle,
                    diamond::Diamond,
                    polygon::Polygon,
                    circle::Circle};

pub trait ShapeVisitor {
    fn visit_canvas(&mut self, canvas: &Canvas);
    fn visit_line(&mut self, line: &Line);
    fn visit_triangle(&mut self, triangle: &Triangle);
    fn visit_square(&mut self, square: &Square);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
    fn visit_diamond(&mut self, diamond: &Diamond);
    fn visit_polygon(&mut self, polygon: &Polygon);
    fn visit_circle(&mut self, circle: &Circle);
}
