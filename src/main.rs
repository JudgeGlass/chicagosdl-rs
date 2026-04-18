use crate::texture::ChicagoSDLTexture;
use crate::window_mgr::WindowMgr;

mod compositor;
mod renderer;
mod texture;
mod window_mgr;

fn main() {
    let (mut sdl_instance, texture_creator) = WindowMgr::new(String::from("ChicagoSDL"), 800, 600);

    // Load once during startup, then pass references around.
    let ui_textures = ChicagoSDLTexture::new(
        &texture_creator,
        String::from("res/uiAtlas.png"),
        8,
        8,
        16,
        16,
    )
    .expect("Error loading texture");

    sdl_instance.prg_loop(&ui_textures);
}
