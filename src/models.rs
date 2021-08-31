use glam::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

pub type Point = Rc<RefCell<RopePoint>>;

pub fn new_point(pos: Vec2, locked: bool) -> Point {
    Rc::new(RefCell::new(RopePoint::new(pos, locked)))
}

#[derive(Clone, Debug)]
pub struct RopePoint {
    pub current_pos: Vec2,
    pub prev_pos: Vec2,
    pub locked: bool,
}

impl RopePoint {
    pub fn new<P: Into<Vec2>>(current_pos: P, locked: bool) -> Self {
        let point = current_pos.into();
        RopePoint { current_pos: point, prev_pos: point, locked }
    }
}

#[derive(Debug)]
pub struct Rope {
    pub end1: Point,
    pub end2: Point,
    pub length: f32,
}

impl Rope {
    pub fn new(end1: Point, end2: Point) -> Self {
        let length = end1.borrow().current_pos.distance(end2.borrow().current_pos);
        Rope { end1, end2, length }
    }
}