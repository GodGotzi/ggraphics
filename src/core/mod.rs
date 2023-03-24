pub mod component;
pub mod container;
pub mod screen;

pub struct Div {
    parent: Option<&Component>,
    childs: Vec<Component>,
}