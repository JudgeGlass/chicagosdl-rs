use sdl2::render::WindowCanvas;

use crate::ChicagoSDLTexture;
use crate::WindowMgr;

pub trait Component {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        font_texture: &mut ChicagoSDLTexture,
        ui_texture: &mut ChicagoSDLTexture,
    );
    fn update(&mut self, window: &mut WindowMgr);

    fn disable(&mut self);
    fn enable(&mut self);
}

pub fn is_in_bounds(x: i32, y: i32, sx: i32, sy: i32, width: i32, height: i32) -> bool {
    x > sx && x < sx + width && y > sy && y < sy + height
}
