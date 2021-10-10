
use crate::shape_visitor::shape_visitor::ShapeVisitor;

use std::error::Error;

pub trait Shape {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor) -> Result<(), Box<dyn Error>>;
}
