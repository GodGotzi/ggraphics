use crate::core::component::{ Component };

pub trait Container : Component {
    fn addChild(&self);
    fn removeChild<T: Component>(&mut self, component: &T);
}