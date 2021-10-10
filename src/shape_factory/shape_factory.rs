
use crate::shapes::{ shape::Shape,
                     canvas::Canvas,
                     circle::Circle,
                     diamond::Diamond,
                     line::Line,
                     polygon::Polygon,
                     rectangle::Rectangle,
                     square::Square,
                     triangle::Triangle };

use crate::utils::{ generic_error::GenericError,
                    color::Color,
                    point::Point };
use std::string::String;
use std::str::SplitWhitespace;
use std::error::Error;

fn create_parse_error(name: String) -> Box<dyn Error> {
    return Box::new(GenericError::new(name));
}

pub struct ShapeFactory {}

impl ShapeFactory {
    pub fn parse_shape(line: &String) -> Result<Box<dyn Shape>, Box<dyn Error>> {
        let mut line_split_iterator = line.split_whitespace();
        let first_element = match line_split_iterator.next() {
            None => {
                return Err(create_parse_error(format!("Line: {} could not be split by whitespaces", line)));
            },
            Some(some) => some
        };
        
        if first_element == "CANVAS" {
            return parse_canvas(&mut line_split_iterator);
        } else if first_element == "LINE" {
            return parse_line(&mut line_split_iterator);
        } else if first_element == "SQUARE" {
            return parse_square(&mut line_split_iterator);
        } else if first_element == "RECTANGLE" {
            return parse_rectangle(&mut line_split_iterator);
        } else if first_element == "CIRCLE" {
            return parse_circle(&mut line_split_iterator);
        } else if first_element == "TRIANGLE" {
            return parse_triangle(&mut line_split_iterator);
        } else if first_element == "DIAMOND" {
            return parse_diamond(&mut line_split_iterator);
        } else if first_element == "POLYGON" {
            return parse_polygon(&mut line_split_iterator);
        }

        let error_string = format!("Invalid line format: {}", line);
        let error = GenericError::new(error_string);
        return Err(Box::new(error));
    } 
}

fn parse_color_hex(line_split: &mut SplitWhitespace, shape: &String) -> Result<[u8; 3], Box<dyn Error>> {
    match line_split.next() {
        None => {
            return Err(create_parse_error(format!("{}'s color does not have rgb component", shape)));
        },
        Some(hex_str) => match hex_str.get(1..) {
            None => {
                return Err(create_parse_error(format!("{}'s color string does not have more than one character",
                                                      shape)));
            },
            Some(some) => match u32::from_str_radix(some, 16) {
                Ok(res) => {
                    let [_, r, g, b] = res.to_be_bytes();
                    return Ok([r, g, b]);
                },
                Err(err) => {
                    return Err(create_parse_error(format!("Could not parse {}'s color hex representation {}: {}",
                                                          shape, hex_str, err)));
                }
            }
        }
    };
}

fn parse_u8(line_split: &mut SplitWhitespace, name: &String, attribute: &String) -> Result<u8, Box<dyn Error>> {
    match line_split.next() {
        None => {
            return Err(create_parse_error(format!("{} does not seem to have a {}", name, attribute)));
        },
        Some(height_str) => match u8::from_str_radix(height_str, 10) {
            Ok(num) => return Ok(num),
            Err(err) => {
                return Err(create_parse_error(format!("Could not convert {}'s {} to u8: {}", name, attribute, err)));
            }
        }
    }
}

fn parse_i32(line_split: &mut SplitWhitespace, name: &String, attribute: &String) -> Result<i32, Box<dyn Error>> {
    match line_split.next() {
        None => {
            return Err(create_parse_error(format!("{} does not seem to have a {}", name, attribute)));
        },
        Some(height_str) => match i32::from_str_radix(height_str, 10) {
            Ok(num) => return Ok(num),
            Err(err) => {
                return Err(create_parse_error(format!("Could not convert {}'s {} to i32: {}", name, attribute, err)));
            }
        }
    }
}

fn parse_u32(line_split: &mut SplitWhitespace, name: &String, attribute: &String) -> Result<u32, Box<dyn Error>> {
    match line_split.next() {
        None => {
            return Err(create_parse_error(format!("{} does not seem to have a {}", name, attribute)));
        },
        Some(height_str) => match u32::from_str_radix(height_str, 10) {
            Ok(num) => return Ok(num),
            Err(err) => {
                return Err(create_parse_error(format!("Could not convert {}'s {} to u32: {}", name, attribute, err)));
            }
        }
    }
}

fn parse_color(line_split: &mut SplitWhitespace, shape: &String, attribute: &String) -> Result<Color, Box<dyn Error>>{
    let [r, g, b] = parse_color_hex(line_split, shape)?;
    let a = parse_u8(line_split, &format!("{}'s {}", shape, attribute), &String::from("alpha"))?;
    return Ok(Color::new(r, g, b, a));
}

fn parse_point(line_split: &mut SplitWhitespace, shape: &String, attribute: &String) -> Result<Point, Box<dyn Error>> {
    let point_name = format!("{}.{}", shape, attribute);
    let x = parse_i32(line_split, &point_name, &String::from("x"))?;
    let y = parse_i32(line_split, &point_name, &String::from("y"))?;
    return Ok(Point::new(x, y));
}

fn parse_canvas(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let height = parse_u32(line_split, &String::from("Canvas"), &String::from("height"))?;
    let width = parse_u32(line_split, &String::from("Canvas"), &String::from("width"))?;
    let color = parse_color(line_split, &String::from("Canvas"), &String::from("color"))?;
    return Ok(Box::new(Canvas::new(height, width, color)));
}

fn parse_line(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let p0 = parse_point(line_split, &String::from("Line"), &String::from("first point"))?;
    let p1 = parse_point(line_split, &String::from("Line"), &String::from("second point"))?;
    let color = parse_color(line_split, &String::from("Line"), &String::from("color"))?;
    return Ok(Box::new(Line::new(p0, p1, color)));
}

fn parse_square(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let top_left = parse_point(line_split, &String::from("Square"), &String::from("top left"))?;
    let side = parse_u32(line_split, &String::from("Square"), &String::from("side"))?;
    let outline_color = parse_color(line_split, &String::from("Square"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Square"), &String::from("fill color"))?;
    return Ok(Box::new(Square::new(top_left, side, outline_color, fill_color)));
}

fn parse_rectangle(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let top_left = parse_point(line_split, &String::from("Rectangle"), &String::from("top left"))?;
    let height = parse_u32(line_split, &String::from("Rectangle"), &String::from("height"))?;
    let width = parse_u32(line_split, &String::from("Rectangle"), &String::from("width"))?;
    let outline_color = parse_color(line_split, &String::from("Rectangle"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Rectangle"), &String::from("fill color"))?;
    return Ok(Box::new(Rectangle::new(top_left, height, width, outline_color, fill_color)));
}

fn parse_circle(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let center = parse_point(line_split, &String::from("Circle"), &String::from("center"))?;
    let radius = parse_u32(line_split, &String::from("Circle"), &String::from("radius"))?;
    let outline_color = parse_color(line_split, &String::from("Circle"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Circle"), &String::from("fill color"))?;
    return Ok(Box::new(Circle::new(center, radius, outline_color, fill_color)));
}

fn parse_triangle(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let p0 = parse_point(line_split, &String::from("Triangle"), &String::from("first point"))?;
    let p1 = parse_point(line_split, &String::from("Triangle"), &String::from("second point"))?;
    let p2 = parse_point(line_split, &String::from("Triangle"), &String::from("third point"))?;
    let outline_color = parse_color(line_split, &String::from("Triangle"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Triangle"), &String::from("fill color"))?;
    return Ok(Box::new(Triangle::new(p0, p1, p2, outline_color, fill_color)));
}

fn parse_diamond(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let center = parse_point(line_split, &String::from("Diamond"), &String::from("center"))?;
    let horizontal_diagonal = parse_u32(line_split, &String::from("Diamond"), &String::from("horizontal diagonal"))?;
    let vertical_diagonal = parse_u32(line_split, &String::from("Diamond"), &String::from("vertical diagonal"))?;
    let outline_color = parse_color(line_split, &String::from("Diamond"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Diamond"), &String::from("fill color"))?;
    return Ok(Box::new(Diamond::new(center, horizontal_diagonal, vertical_diagonal, outline_color, fill_color)));
}

fn parse_polygon(line_split: &mut SplitWhitespace) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let points_number = parse_u32(line_split, &String::from("Polygon"), &String::from("number of points"))?;

    let mut points: Vec<Point> = Vec::new();
    points.reserve(points_number as usize);

    for point_index in 0..points_number {
        let point = parse_point(line_split, &String::from("Polygon"), &format!("point {}", point_index))?;
        points.push(point);
    }
    let outline_color = parse_color(line_split, &String::from("Polygon"), &String::from("outline color"))?;
    let fill_color = parse_color(line_split, &String::from("Polygon"), &String::from("fill color"))?;
    return Ok(Box::new(Polygon::new(points, outline_color, fill_color)));
}
