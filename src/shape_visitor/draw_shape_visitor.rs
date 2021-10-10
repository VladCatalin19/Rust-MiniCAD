
use crate::shape_visitor::shape_visitor::ShapeVisitor;

use crate::shapes::{canvas::Canvas,
                    line::Line,
                    triangle::Triangle,
                    square::Square,
                    rectangle::Rectangle,
                    diamond::Diamond,
                    polygon::Polygon,
                    circle::Circle};

use crate::utils::{ generic_error::GenericError, color::Color, point::Point };

use std::string::String;
use std::error::Error;
use std::vec::Vec;
use std::collections::VecDeque;

extern crate image;
use image::{RgbaImage, Rgba};

pub struct DrawShapeVisitor {
    image: RgbaImage
}

impl DrawShapeVisitor {
    pub fn new() -> Self {
        return DrawShapeVisitor{image: RgbaImage::new(0, 0)};
    }
    pub fn write_image(&self, file_name: &String) -> Result<(), Box<dyn Error>> {
        return match self.image.save(file_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                let new_error_string = format!("Cannot open {}: {}", file_name, err);
                return Err(Box::new(GenericError::new(new_error_string)));
            }
        };
    }
}

impl ShapeVisitor for DrawShapeVisitor {
    fn visit_canvas(&mut self, canvas: &Canvas) -> Result<(), Box<dyn Error>> {
        self.image = RgbaImage::new(canvas.get_width(), canvas.get_height());
        let fill_color = Rgba(color_to_rgba(&canvas.get_color()));
        
        for pixel in self.image.pixels_mut() {
            *pixel = fill_color;
        }
        return Ok(());
    }

    fn visit_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        let color = Rgba(color_to_rgba(&line.get_color()));
        // Generalized Integer Bresenham's Algorithm for all quadrants
        // Stolen from https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let mut x0 = line.get_p0().x;
        let mut y0 = line.get_p0().y;
        let x1 = line.get_p1().x;
        let y1 = line.get_p1().y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 {1} else {-1};
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 {1} else {-1};
        let mut err = dx + dy;

        loop {
            put_pixel(&mut self.image, x0, y0, color);
            if (x0 == x1) && (y0 == y1) {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
        return Ok(());
    }

    fn visit_triangle(&mut self, triangle: &Triangle) -> Result<(), Box<dyn Error>> {
        let p0 = triangle.get_p0();
        let p1 = triangle.get_p1();
        let p2 = triangle.get_p2();
        let polygon = Polygon::new(Vec::from([p0, p1, p2]), triangle.get_outline_color(), triangle.get_fill_color());
        return self.visit_polygon(&polygon);
    }

    fn visit_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        let rectangle = Rectangle::new(square.get_top_left(), square.get_side(), square.get_side(),
                                       square.get_outline_color(), square.get_fill_color());
        return self.visit_rectangle(&rectangle);
    }

    fn visit_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        let outline_color = Rgba(color_to_rgba(&rectangle.get_outline_color()));
        let fill_color = Rgba(color_to_rgba(&rectangle.get_fill_color()));

        for x in 0..rectangle.get_width() {
            for y in 0..rectangle.get_height() {
                let x_coord = rectangle.get_top_left().x + (x as i32);
                let y_coord = rectangle.get_top_left().y + (y as i32);

                let color: Rgba<u8>;
                if (x == 0) || (x == rectangle.get_width() - 1) || (y == 0) || (y == rectangle.get_height() - 1) {
                    color = outline_color;
                } else {
                    color = fill_color;
                }

                put_pixel(&mut self.image, x_coord, y_coord, color);
            }
        }
        return Ok(());
    }

    fn visit_diamond(&mut self, diamond: &Diamond) -> Result<(), Box<dyn Error>> {
        let mut p0 = diamond.get_center();
        p0.x -= (diamond.get_horizontal_diagonal() / 2) as i32;
        let mut p1 = diamond.get_center();
        p1.y -= (diamond.get_vertical_diagonal() / 2) as i32;
        let mut p2 = diamond.get_center();
        p2.x += (diamond.get_horizontal_diagonal() / 2) as i32;
        let mut p3 = diamond.get_center();
        p3.y += (diamond.get_vertical_diagonal() / 2) as i32;
        let polygon = Polygon::new(Vec::from([p0, p1, p2, p3]), diamond.get_outline_color(), diamond.get_fill_color());
        return self.visit_polygon(&polygon);
    }

    fn visit_polygon(&mut self, polygon: &Polygon) -> Result<(), Box<dyn Error>> {
        let outline_color = Rgba(color_to_rgba(&polygon.get_outline_color()));
        let fill_color = Rgba(color_to_rgba(&polygon.get_fill_color()));
        let points = polygon.get_points();
        let center = get_polygon_center(points);

        draw_polygon_outline(points, polygon.get_outline_color(), self)?;
        return flood_fill(&mut self.image, center.x, center.y, outline_color, fill_color);
    }

    fn visit_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        let outline_color = Rgba(color_to_rgba(&circle.get_outline_color()));
        let fill_color = Rgba(color_to_rgba(&circle.get_fill_color()));
        // Circle generation using Brasenham's algorithm
        // https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
        let xc = circle.get_center().x;
        let yc = circle.get_center().y;
        let mut x: i32 = 0;
        let mut y: i32 = circle.get_radius() as i32;
        let mut d: i32 = 3 - 2 * (circle.get_radius() as i32);

        while y >= x {
            put_pixels_in_8_parts_around_circle(&mut self.image, xc, yc, x, y, outline_color);
            x += 1;
            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
        }
        return flood_fill(&mut self.image, xc, yc, outline_color, fill_color);
    }
}

fn color_to_rgba(color: &Color) -> [u8; 4] {
    return [color.r, color.g, color.b, color.a];
}

fn is_pixel_inside_image(image: &RgbaImage, x: i32, y: i32) -> bool {
    return (x >= 0) && ((x as u32) < image.width()) && (y >= 0) && ((y as u32) < image.height());
}

fn put_pixel(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if is_pixel_inside_image(image, x, y) {
        image.put_pixel(x as u32, y as u32, color);
    }
}

fn put_pixels_in_8_parts_around_circle(image: &mut RgbaImage, x_circle: i32, y_cirlce: i32,
                                       x: i32, y: i32, color: Rgba<u8>) {
    let add_to_x = [  x, -x,  x, -x,  y, -y,  y, -y ];
    let add_to_y = [  y,  y, -y, -y,  x,  x, -x, -x ];
    for index in 0..add_to_x.len() {
        put_pixel(image, x_circle + add_to_x[index], y_cirlce + add_to_y[index], color);
    }
}

fn draw_polygon_outline(points: &Vec<Point>, outline_color: Color, draw_visitor: &mut DrawShapeVisitor)
-> Result<(), Box<dyn Error>> {

    for point_index in 0..points.len() {
        let p0 = points[point_index];
        let p1 = points[(point_index + 1) % points.len()];
        draw_visitor.visit_line(&Line::new(p0, p1, outline_color))?;
    }
    return Ok(());
}

fn get_polygon_center(points: &Vec<Point>) -> Point {
    let mut center = Point::new(-1, -1);

    for point in points {
        center.x += point.x;
        center.y += point.y;
    }
    center.x /= points.len() as i32;
    center.y /= points.len() as i32;

    return center;
}

fn flood_fill(image: &mut RgbaImage, x_start: i32, y_start: i32,
              outline_color: Rgba<u8>, fill_color: Rgba<u8>) -> Result<(), Box<dyn Error>> {

    if !is_pixel_inside_image(image, x_start, y_start) {
        let error_name = format!("Coordinates {} are outside canvas", Point::new(x_start, y_start));
        return Err(Box::new(GenericError::new(error_name)));
    }

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(Point::new(x_start, y_start));

    let mut visited = vec![vec![false; image.height() as usize]; image.width() as usize];

    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();

        if visited[point.x as usize][point.y as usize] {
            continue;
        }
        visited[point.x as usize][point.y as usize] = true;

        put_pixel(image, point.x, point.y, fill_color);

        let neighbors = [ Point::new(point.x + 1, point.y),
                          Point::new(point.x - 1, point.y),
                          Point::new(point.x, point.y + 1),
                          Point::new(point.x, point.y - 1) ];

        for neighbor in neighbors {
            if is_pixel_inside_image(image, neighbor.x, neighbor.y)
               && !is_the_same_color(*image.get_pixel(neighbor.x as u32, neighbor.y as u32), outline_color) {
                queue.push_back(neighbor);
            }
        }
    }
    return Ok(());
}

fn is_the_same_color(color1: Rgba<u8>, color2: Rgba<u8>) -> bool {
    for index in 0..3 {
        if color1[index] != color2[index] {
            return false;
        }
    }
    return true;
}
