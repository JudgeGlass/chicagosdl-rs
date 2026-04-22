use std::thread;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

extern crate sdl2;

use crate::component::Component;
use crate::compositor::*;
use crate::renderer::Renderer;
use crate::texture::ChicagoSDLTexture;

use crate::button::Button;
use crate::window::Window;

pub struct WindowMgr {
    pub context: Sdl,
    pub renderer: Renderer,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub running: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub mouse1_pressed: bool,
    pub components: Vec<Box<dyn Component>>,
}

impl WindowMgr {
    pub fn new(
        window_title: String,
        window_width: u32,
        window_height: u32,
    ) -> (WindowMgr, TextureCreator<WindowContext>) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(&window_title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let renderer = Renderer::new(canvas);
        let texture_creator = renderer.canvas.texture_creator();

        (
            WindowMgr {
                context: sdl_context,
                renderer,
                mouse_x: 0,
                mouse_y: 0,
                running: true,
                window_width,
                window_height,
                mouse1_pressed: false,
                components: Vec::new(),
            },
            texture_creator,
        )
    }

    pub fn prg_loop(
        &mut self,
        ui_textures: &mut ChicagoSDLTexture,
        font_textures: &mut ChicagoSDLTexture,
    ) {
        while self.running {
            self.event();
            self.update();
            self.render(ui_textures, font_textures);
        }
    }

    pub fn close(&self) {}

    pub fn event(&mut self) {
        let events = &mut self.context.event_pump().unwrap();
        self.mouse_x = events.mouse_state().x() as u32;
        self.mouse_y = events.mouse_state().y() as u32;
        self.mouse1_pressed = false;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        self.running = false;
                    }
                }

                Event::MouseButtonDown { x: _, y: _, .. } => {
                    self.mouse1_pressed = true;
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        // println!("Mouse X: {} Y: {}", self.mouse_x, self.mouse_y)
        let mut components = std::mem::take(&mut self.components);
        for comp in components.iter_mut() {
            comp.update(self);
        }
        self.components = components;
    }

    pub fn render(
        &mut self,
        _ui_textures: &mut ChicagoSDLTexture,
        _font_textures: &mut ChicagoSDLTexture,
    ) {
        let canvas = &mut self.renderer.canvas;

        canvas.set_draw_color(Color::RGB(0x00, 0x80, 0x80));
        canvas.clear();

        for comp in &self.components {
            comp.render(canvas, _font_textures, _ui_textures);
        }

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}
