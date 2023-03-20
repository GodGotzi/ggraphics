trait Component {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_dimension(&self) -> (u32, U32);
    fn get_position_screen(&self) -> (u32, u32);
    fn get_parent_screen(&self) -> &ComponentScreen;
    fn get_screen() -> &ComponentScreen;
}

pub struct ComponentScreen {
    parent: &Component,
    childs: Vec<Component>,
}