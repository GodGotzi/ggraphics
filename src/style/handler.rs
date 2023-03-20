use crate::core::{ Screen, Component };
use crate::style::property::{ StyleProperty, PropertyType, PropertyValue };
use std::collections::HashMap;

pub struct StyleAction {
    action: fn(&Screen, &Component, T),

}


pub struct StyleHandler {
    screen: &Screen,

    action_map: HashMap<PropertyType, StyleAction>,
}