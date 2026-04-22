use crate::component::Component;

use crate::ChicagoSDLTexture;
use crate::WindowMgr;
use sdl2::render::WindowCanvas;

pub struct Window {
    pub x: i32,
    pub y: i32,
}

impl Component for Window {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        font_texture: &mut ChicagoSDLTexture,
        ui_texture: &mut ChicagoSDLTexture,
    ) {
        println!("Render window")
    }

    fn update(&mut self, window: &mut WindowMgr) {
        println!("Update window")
    }

    fn disable(&mut self) {}

    fn enable(&mut self) {}
}
