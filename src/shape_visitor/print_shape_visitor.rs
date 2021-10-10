
use crate::shape_visitor::shape_visitor::ShapeVisitor;

use crate::shapes::{canvas::Canvas,
                    line::Line,
                    triangle::Triangle,
                    square::Square,
                    rectangle::Rectangle,
                    diamond::Diamond,
                    polygon::Polygon,
                    circle::Circle};

use std::error::Error;

pub struct PrintShapeVisitor {

}

impl PrintShapeVisitor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return PrintShapeVisitor{};
    }
}

impl ShapeVisitor for PrintShapeVisitor {
    fn visit_canvas(&mut self, canvas: &Canvas) -> Result<(), Box<dyn Error>> {
        println!("{}", canvas);
        return Ok(());
    }

    fn visit_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        println!("{}", line);
        return Ok(());
    }

    fn visit_triangle(&mut self, triangle: &Triangle) -> Result<(), Box<dyn Error>> {
        println!("{}", triangle);
        return Ok(());
    }

    fn visit_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        println!("{}", square);
        return Ok(());
    }

    fn visit_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        println!("{}", rectangle);
        return Ok(());
    }

    fn visit_diamond(&mut self, diamond: &Diamond) -> Result<(), Box<dyn Error>> {
        println!("{}", diamond);
        return Ok(());
    }

    fn visit_polygon(&mut self, polygon: &Polygon) -> Result<(), Box<dyn Error>> {
        println!("{}", polygon);
        return Ok(());
    }

    fn visit_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        println!("{}", circle);
        return Ok(());
    }
}
