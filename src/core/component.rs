use super::container::Container;

pub trait Component {
    fn get_parent<T: Container>(&self) -> &T;
    fn set_parent<T: Container>(&self, parent: &T);
}
