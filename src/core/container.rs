use core::component::{ Div };

trait Container {
    pub fn addChild(&self);
    pub fn removeChild(&mut self, component: &Component);
}

impl Container for Div {
    fn addChild(&mut self, component: Component) {
        
    }

    fn removeChild(&mut self, component: &Component) {
        
    }
}