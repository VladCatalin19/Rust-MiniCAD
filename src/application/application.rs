
use crate::shapes::shape::Shape;
use std::vec::Vec;
use std::string::String;
use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::error::Error;

use crate::shape_factory::shape_factory::ShapeFactory;
use crate::utils::parse_error::ParseError;

use crate::shape_visitor::draw_shape_visitor::DrawShapeVisitor;

pub struct Application {}

impl Application {
    pub fn run(input_file: &String, output_file: &String) -> Result<(), Box<dyn Error>> {
        let shapes = read_shapes(&input_file)?;
        let draw_visitor = draw_shapes(&shapes);
        write_image_to_file(output_file, &draw_visitor)?;
        return Ok(());
    }
}

fn read_shapes(input_file: &String) -> Result<Vec<Box<dyn Shape>>, Box<dyn Error>> {
    let file = open_file(input_file)?;

    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let shapes_number = read_shapes_number(&mut lines)?;

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.reserve_exact(shapes_number);

    for _ in 0..shapes_number {
        let line = read_line(&mut lines);
        let shape = read_shape_from_line(line)?;
        shapes.push(shape);
    }

    return Ok(shapes);
}

fn draw_shapes(shapes: &Vec<Box<dyn Shape>>) -> DrawShapeVisitor
{
    let mut draw_visitor = DrawShapeVisitor::new();

    for shape in shapes {
        shape.accept(&mut draw_visitor);
    }
    return draw_visitor;
}

fn write_image_to_file(input_file: &String, draw_visitor: &DrawShapeVisitor) -> Result<(), Box<dyn Error>> {
    return match draw_visitor.write_image(input_file) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err)
    };
}

fn open_file(input_file: &String) -> Result<File, Box<dyn Error>> {
    return match File::open(input_file) {
        Ok(res) => Ok(res),
        Err(err) => {
            let new_error_string = format!("Cannot open {}: {}", input_file, err);
            let new_error = std::io::Error::new(err.kind(), new_error_string);
            return Err(Box::new(new_error))
        }
    };
}

fn read_shapes_number(lines: &mut Lines<BufReader<File>>) -> Result<usize, Box<dyn Error>> {
    return match lines.next() {
        None => {
            let error_string = format!("Input file does not have at least one line");
            let error = ParseError::new(error_string);
            return Err(Box::new(error));
        },
        Some(first_line) => match first_line {
            Ok(first_line_string) => match first_line_string.parse::<usize>() {
                Ok(num) => Ok(num),
                Err(err) => {
                    let error_string = format!("First line of: {}", err);
                    let error = ParseError::new(error_string);
                    return Err(Box::new(error));
                }
            },
            Err(err) => {
                let error_string = format!("Error while parsing shapes number: {}", err);
                let error = ParseError::new(error_string);
                return Err(Box::new(error));
            }
        }
    };
}

fn read_line(lines: &mut Lines<BufReader<File>>) -> Result<String, std::io::Error> {
    return match lines.next() {
        None => {
            let error_string = format!("No more lines to parse");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error_string));
        },
        Some(some) => some
    };
}

fn read_shape_from_line(line: Result<String, std::io::Error>) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let line_string = match line {
        Ok(line_str) => line_str,
        Err(err) => {
            let error_string = format!("Error while parsing line: {}", err);
            let error = ParseError::new(error_string);
            return Err(Box::new(error));
        }
    };
    return ShapeFactory::parse_shape(&line_string);
}
