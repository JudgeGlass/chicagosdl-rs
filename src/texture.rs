use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub fn load_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    file: &str,
) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(file)
}

pub struct ChicagoSDLTexture<'a> {
    pub texture: Texture<'a>,
    pub file: String,
    pub pw: i32,
    pub ph: i32,
    pub col: i32,
    pub row: i32,
}

impl<'a> ChicagoSDLTexture<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        image_path: String,
        pw: i32,
        ph: i32,
        col: i32,
        row: i32,
    ) -> Result<ChicagoSDLTexture<'a>, String> {
        let texture = load_texture(texture_creator, &image_path)?;
        Ok(ChicagoSDLTexture {
            texture,
            file: image_path,
            pw,
            ph,
            col,
            row,
        })
    }

    pub fn render_texture(
        &self,
        canvas: &mut WindowCanvas,
        index: i32,
        x: i32,
        y: i32,
        scale: i32,
        rowa: i32,
    ) {
        let sx = (index % rowa) * self.pw;
        let sy = (index / rowa) * self.ph;
        canvas
            .copy(
                self.texture(),
                Rect::new(sx, sy, self.pw as u32, self.ph as u32),
                Rect::new(x, y, (self.pw * scale) as u32, (self.ph * scale) as u32),
            )
            .unwrap();
    }

    pub fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    pub fn color_mod(&mut self, color: i32) {
        let r = (color & 0xFF0000) >> 16;
        let g = (color & 0x00FF00) >> 8;
        let b = color & 0xFF;
        self.texture.set_color_mod(r as u8, g as u8, b as u8);
    }

    pub fn into_texture(self) -> Texture<'a> {
        self.texture
    }
}
