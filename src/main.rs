extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

use crate::window_mgr::WindowMgr;

mod renderer;
mod window_mgr;

fn main() {
    let mut sdl_instance: WindowMgr = WindowMgr::new(String::from("ChicagoSDL"), 800, 600);

    sdl_instance.prg_loop();
}
