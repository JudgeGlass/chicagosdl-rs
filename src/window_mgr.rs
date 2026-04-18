use std::thread;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

extern crate sdl2;

use crate::compositor::*;
use crate::renderer::Renderer;
use crate::texture::ChicagoSDLTexture;

pub struct WindowMgr {
    pub context: Sdl,
    pub renderer: Renderer,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub running: bool,
    pub window_width: u32,
    pub window_height: u32,
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
            },
            texture_creator,
        )
    }

    pub fn prg_loop(
        &mut self,
        ui_textures: &ChicagoSDLTexture,
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

                Event::MouseButtonDown { x: _, y: _, .. } => {}

                _ => {}
            }
        }
    }

    pub fn update(&self) {
        println!("Mouse X: {} Y: {}", self.mouse_x, self.mouse_y)
    }

    pub fn render(
        &mut self,
        _ui_textures: &ChicagoSDLTexture,
        _font_textures: &mut ChicagoSDLTexture,
    ) {
        let canvas = &mut self.renderer.canvas;

        canvas.set_draw_color(Color::RGB(0x00, 0x80, 0x80));
        canvas.clear();

        draw_window_frame(canvas, 0, 0, self.window_width, self.window_height);
        draw_button_normal(canvas, _font_textures, 5, 35, 100, 25, String::from("test"));
        draw_button_pushed(
            canvas,
            _font_textures,
            5,
            15 + 35 + 5,
            100,
            25,
            String::from("test"),
        );
        draw_input_buffer(canvas, 5, 100, 20, 1);
        draw_progress_bar(canvas, _font_textures, 5, 150, 200, 64, 0.57);

        draw_button_normal(canvas, _font_textures, 50, 50, 16, 16, String::from(""));
        _ui_textures.render_texture(canvas, 0, 50, 50, 1, 16);

        draw_checkbox(
            canvas,
            _font_textures,
            _ui_textures,
            120,
            220,
            true,
            String::from("This is a checkbox"),
        );

        draw_checkbox(
            canvas,
            _font_textures,
            _ui_textures,
            120,
            250,
            false,
            String::from("This is a checkbox"),
        );

        draw_string_shadowed(
            canvas,
            _font_textures,
            60,
            60,
            &String::from("This is a test string"),
            0xFFFFFF,
            1,
        );

        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
}
