use crate::core::Container;

pub struct Div<T: Container> {
    parent: &T,
    components: Vec<&Component>
}

impl<T: Container> Component<T> for Div {

    fn get_parent(&self) -> &T {
        self.parent
    }

    fn set_parent(&mut self, parent: &T) {
        self.parent = parent;
    }

    fn addChild(&mut self, component: T) {
        
    }

    fn removeChild(&mut self, component: &T) {
        
    }
}