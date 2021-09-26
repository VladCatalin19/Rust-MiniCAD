
use crate::shape_visitor::shape_visitor::ShapeVisitor;

pub trait Shape {
    fn accept(&self, shape_visitor: &mut dyn ShapeVisitor);
}
