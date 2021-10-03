
use crate::shapes::shape::Shape;
use std::vec::Vec;
use std::string::String;
use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::error::Error;

use crate::shape_factory::shape_factory::ShapeFactory;
use crate::utils::parse_error::ParseError;

use crate::shape_visitor::print_shape_visitor::PrintShapeVisitor;

pub struct Application {}

impl Application {
    pub fn run(input_file: &String, output_file: &String) -> Result<(), Box<dyn Error>> {
        let shapes = match read_shapes(&input_file) {
            Ok(shapes_vec) => shapes_vec,
            Err(err) => return Err(err)
        };
        return Ok(());
    }
}

fn read_shapes(input_file: &String) -> Result<Vec<Box<dyn Shape>>, Box<dyn Error>> {
    let file = match open_file(input_file) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let shapes_number = match read_shapes_number(&mut lines) {
        Ok(num) => num,
        Err(err) => return Err(err)
    };

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.reserve_exact(shapes_number);

    let mut print_visitor = PrintShapeVisitor::new();

    for line in lines {
        let shape = match read_shape_from_line(line) {
            Ok(shp) => shp,
            Err(err) => return Err(err)
        };
        shape.accept(&mut print_visitor);
        shapes.push(shape);
    }

    return Ok(shapes);
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
    return match lines.nth(0) {
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
