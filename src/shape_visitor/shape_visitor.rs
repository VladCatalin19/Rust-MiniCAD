
use crate::shapes::{canvas::Canvas,
                    line::Line,
                    triangle::Triangle,
                    square::Square,
                    rectangle::Rectangle,
                    diamond::Diamond,
                    polygon::Polygon,
                    circle::Circle};

use std::error::Error;

pub trait ShapeVisitor {
    fn visit_canvas(&mut self, canvas: &Canvas) -> Result<(), Box<dyn Error>>;
    fn visit_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>>;
    fn visit_triangle(&mut self, triangle: &Triangle) -> Result<(), Box<dyn Error>>;
    fn visit_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>>;
    fn visit_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>>;
    fn visit_diamond(&mut self, diamond: &Diamond) -> Result<(), Box<dyn Error>>;
    fn visit_polygon(&mut self, polygon: &Polygon) -> Result<(), Box<dyn Error>>;
    fn visit_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>>;
}
