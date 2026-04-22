use crate::button::Button;
use crate::component::Component;
use crate::texture::ChicagoSDLTexture;
use crate::window_mgr::WindowMgr;

mod button;
mod component;
mod compositor;
mod renderer;
mod texture;
mod window;
mod window_mgr;

fn print_hello() {
    println!("Hello world");
}

fn main() {
    let (mut sdl_instance, texture_creator) = WindowMgr::new(String::from("ChicagoSDL"), 800, 600);

    // Load once during startup, then pass references around.
    let mut ui_textures = ChicagoSDLTexture::new(
        &texture_creator,
        String::from("res/uiAtlas.png"),
        16,
        16,
        16,
        16,
    )
    .expect("Error loading UI texture");

    let mut font_textures = ChicagoSDLTexture::new(
        &texture_creator,
        String::from("res/fontAtlas.png"),
        8,
        8,
        16,
        16,
    )
    .expect("Error loading font texture");

    let mut b = Button::new(50, 50, 70, 24, String::from("OK"));

    fn disable_button(b: Button) {}

    b.on_click(disable_button);

    sdl_instance.components.push(Box::new(b));

    sdl_instance.prg_loop(&mut ui_textures, &mut font_textures);
}
