use std::clone::Clone;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum PropertyValue {
    Center,
    Left,
    Right,
    Flex
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum PropertyType {
    Margin,
    Display,
    JustifyContent
}

pub struct StyleProperty {
    property: String,
    is_float_property: bool
}

impl StyleProperty {
    pub fn new(property: String, is_float_property: bool) -> StyleProperty {
        StyleProperty {
            property,
            is_float_property
        }
    }

    pub fn is_float_property(&self) -> &bool {
        &self.is_float_property
    }

    pub fn get_property(&self) -> &String {
        &self.property
    }
}

pub struct StyleProperties {
    properties: HashMap<PropertyType, StyleProperty>
}

impl StyleProperties {
    pub fn new() -> StyleProperties {
        let properties_map: HashMap<PropertyType, StyleProperty> = HashMap::new();
        StyleProperties { properties: properties_map }
    }

    pub fn register_style(&mut self, property_type: PropertyType, property: String) {
        let style_property = StyleProperty::new(property, false);

        self.properties.insert(property_type, style_property);
    }

    pub fn dergister_style(&mut self, property_type: &PropertyType) {
        self.properties.remove(property_type);
    }
}

impl Clone for StyleProperty {
    fn clone(&self) -> Self {
        Self { property: self.property.clone(), is_float_property: self.is_float_property.clone() }
    }
}

