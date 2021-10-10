
use crate::shape_visitor::shape_visitor::ShapeVisitor;

use crate::shapes::{canvas::Canvas,
                    line::Line,
                    triangle::Triangle,
                    square::Square,
                    rectangle::Rectangle,
                    diamond::Diamond,
                    polygon::Polygon,
                    circle::Circle};

pub struct PrintShapeVisitor {

}

impl PrintShapeVisitor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return PrintShapeVisitor{};
    }
}

impl ShapeVisitor for PrintShapeVisitor {
    fn visit_canvas(&mut self, canvas: &Canvas) {
        println!("{}", canvas);
    }

    fn visit_line(&mut self, line: &Line) {
        println!("{}", line);
    }

    fn visit_triangle(&mut self, triangle: &Triangle) {
        println!("{}", triangle);
    }

    fn visit_square(&mut self, square: &Square) {
        println!("{}", square);
    }

    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        println!("{}", rectangle);
    }

    fn visit_diamond(&mut self, diamond: &Diamond) {
        println!("{}", diamond);
    }

    fn visit_polygon(&mut self, polygon: &Polygon) {
        println!("{}", polygon);
    }

    fn visit_circle(&mut self, circle: &Circle) {
        println!("{}", circle);
    }
}
