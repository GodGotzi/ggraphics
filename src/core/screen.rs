pub struct Screen {
    width: u32,
    height: u32,
}

pub trait Drawable {
    fn draw();
}