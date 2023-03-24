mod style;

use pronto_graphics::Window;
use pronto_graphics::Color;

fn main() {
    let mut pg = Window::new(800, 600, "Circles!");

    loop {
        pg.circle((100., 100.), 15.);
        pg.fill_color(Color::BLUE);
        pg.circle((300., 450.), 24.);
        
        pg.update();
    }
}
