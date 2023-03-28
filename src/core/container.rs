use crate::core::component::{ Component };

pub trait Container : Component {
    fn add_child(&self);
    fn remove_child<T: Component>(&mut self, component: &T);
}