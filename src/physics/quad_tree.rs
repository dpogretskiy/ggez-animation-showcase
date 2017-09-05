use super::*;

use std::borrow::BorrowMut;
use std::cell::RefCell;

pub trait Positioned {
    fn to_rect(&self) -> Rect;
}

pub struct QuadTree<T> {
    level: usize,
    bounds: Rect,
    objects: Vec<T>,
    nodes: Option<RefCell<Box<[QuadTree<T>; 4]>>>,
}

impl<T> QuadTree<T>
where
    T: Clone + Sized + Positioned,
{
    pub fn new(level: usize, bounds: Rect) -> QuadTree<T> {
        QuadTree {
            level,
            bounds,
            objects: vec![],
            nodes: None,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    fn split(&mut self) {
        let sub_width = self.bounds.w / 2.0;
        let sub_height = self.bounds.h / 2.0;

        let x = self.bounds.x;
        let y = self.bounds.y;

        let level = self.level + 1;

        self.nodes = Some(RefCell::new(Box::new([
            QuadTree::new(level, Rect::rect(x + sub_width, y, sub_width, sub_height)),
            QuadTree::new(level, Rect::rect(x, y, sub_width, sub_height)),
            QuadTree::new(level, Rect::rect(x, y + sub_height, sub_width, sub_height)),
            QuadTree::new(
                level,
                Rect::rect(x + sub_width, y + sub_height, sub_width, sub_height),
            ),
        ])));
    }

    pub fn insert(&mut self, object: T) {
        if let Some(ref nodes) = self.nodes {
            let index = get_index(&self.bounds, &object.to_rect());
            if index != -1 {
                nodes.borrow_mut()[index as usize].insert(object);
                return;
            }
        }

        self.objects.push(object);

        if self.objects.len() > MAX_OBJECTS && self.level < MAX_LEVELS {
            if self.nodes.is_none() {
                self.split();
            }

            for o in self.objects.drain(..) {
                let ix = get_index(&self.bounds, &o.to_rect());

                if ix != -1 {
                    if let Some(ref nodes) = self.nodes {
                        nodes.borrow_mut()[ix as usize].insert(o);
                    }
                }
            }
        }
    }

    fn retreive_rec(&self, ret: &mut Vec<T>, rect: Rect) {
        let ix = get_index(&self.bounds, &rect);
        if ix != -1 {
            if let Some(ref nodes) = self.nodes {
                nodes.borrow()[ix as usize].retreive_rec(ret, rect);
            }
        }

        ret.extend_from_slice(self.objects.as_slice());
    }

    pub fn retrieve(&self, rect: Rect) -> Vec<T> {
        let mut ret = vec![];
        self.retreive_rec(&mut ret, rect);
        ret
    }
}

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    fn rect(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect { x, y, w, h }
    }
}

impl Positioned for MovingObject {
    fn to_rect(&self) -> Rect {
        let xy = self.position - self.aabb.half_size() + self.aabb.offset;
        let wh = self.aabb.half_size() * 2.0;
        Rect::rect(xy.x, xy.y, wh.x, wh.y)
    }
}

fn get_index(bounds: &Rect, rect: &Rect) -> isize {
    let vertical_midpoint = bounds.x + bounds.w / 2.0;
    let horizontal_midpoint = bounds.y + bounds.h / 2.0;

    let top_quadrant = rect.y < horizontal_midpoint && rect.y + rect.h < horizontal_midpoint;
    let bottom_quadrant = rect.y > horizontal_midpoint;

    let mut index = -1;

    if rect.x < vertical_midpoint && rect.x + rect.w < vertical_midpoint {
        if top_quadrant {
            index = 1;
        } else if bottom_quadrant {
            index = 2;
        }
    } else if rect.x > vertical_midpoint {
        if top_quadrant {
            index = 0;
        } else if bottom_quadrant {
            index = 3;
        }
    }
    index
}

const MAX_OBJECTS: usize = 10;
const MAX_LEVELS: usize = 5;
