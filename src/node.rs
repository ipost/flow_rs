use std::collections::HashSet;

use crate::attr::{Attr, Shape};

#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub attrs: HashSet<Attr>,
}

impl Node {
    pub fn new(id: String) -> Self {
        Node {
            id: id,
            attrs: HashSet::new(),
        }
    }

    pub fn labelled(mut self, l: String) -> Self {
        self.attrs.insert(Attr::Label(l));
        self
    }

    pub fn diamond(mut self) -> Self {
        self.attrs.insert(Attr::Shape(Shape::Diamond));
        self
    }

    pub fn rectangle(mut self) -> Self {
        self.attrs.insert(Attr::Shape(Shape::Rectangle));
        self
    }

    pub fn terminal(self) -> Self {
        self.red().bold()
    }

    pub fn red(mut self) -> Self {
        self.attrs.insert(Attr::Color("red".to_string()));
        self
    }

    pub fn bold(mut self) -> Self {
        self.attrs.insert(Attr::PenWidth(3));
        self
    }
}
