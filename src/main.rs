
use crate::shapes::shape::Shape;

mod utils;
mod shapes;
mod shape_visitor;

fn main() {
    let circle_center = utils::point::Point::new(0, 0);
    let circle_radius: u32 = 30;
    let circle_color = utils::color::Color::new(125, 125, 125, 125);
    let circle = shapes::circle::Circle::new(circle_center, circle_radius, circle_color, circle_color);

    let mut print_visitor = shape_visitor::print_shape_visitor::PrintShapeVisitor::new();

    circle.accept(&mut print_visitor);
}


/*
use std::io::{BufRead, BufReader};
use std::fs::File;

let reader = BufReader::new(File::open("file.txt").expect("Cannot open file.txt"));

for line in reader.lines() {
    for word in line.unwrap().split_whitespace() {
        println!("word '{}'", word);
    }
}
*/