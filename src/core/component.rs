use std::option::Option;
use std::collections::Vec;
use core::{ Div };

trait Component {
    pub fn get_parent(&self) -> &Component;
    fn set_parent(&self, parent: &Component);
}

impl Component for Div {
    fn new() -> Div {
        let childs: Vec<Component> = Vec::new();
        
        Div {
            parent: None,
            childs
        }
    }
    
    fn get_parent(&self) -> &Component {
        self.parent
    }

    fn set_parent(&mut self, parent: &Component) {
        self.parent = parent;
    }
}