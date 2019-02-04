use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub enum Shape {
    Rectangle,
    Diamond,
}
impl Shape {
    pub fn as_dot(&self) -> String {
        match self {
            Shape::Rectangle => "rectangle".to_string(),
            Shape::Diamond => "diamond".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Attr {
    Color(String),
    Label(String),
    PenWidth(usize),
    Shape(Shape),
}
impl PartialEq for Attr {
    fn eq(&self, other: &Attr) -> bool {
        // check whether the enum variant is the same, ignoring associated values
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
impl Hash for Attr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state)
    }
}
impl Eq for Attr {}
impl Attr {
    pub fn as_dot(&self) -> String {
        match self {
            Attr::PenWidth(w) => format!("penwidth={}", w),
            Attr::Color(c) => format!("color={}", escaped(c)),
            Attr::Label(l) => format!("label=\"{}\"", escaped(l)),
            Attr::Shape(s) => format!("shape=\"{}\"", escaped(&s.as_dot())),
        }
    }
}

fn escaped(s: &String) -> String {
    str::replace(s, "\"", "\\\"")
}
