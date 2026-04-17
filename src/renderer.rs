use sdl2::pixels;
use sdl2::render::WindowCanvas;

pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(mut canvas: WindowCanvas) -> Renderer {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Renderer { canvas }
    }
}
