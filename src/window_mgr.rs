use std::thread;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

extern crate sdl2;

use crate::renderer::Renderer;

pub struct WindowMgr {
    pub context: Sdl,
    pub renderer: Renderer,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub running: bool,
}

impl WindowMgr {
    pub fn new(window_title: String, window_width: u32, window_height: u32) -> WindowMgr {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(&window_title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let renderer = Renderer::new(canvas);

        WindowMgr {
            context: sdl_context,
            renderer,
            mouse_x: 0,
            mouse_y: 0,
            running: true,
        }
    }

    pub fn prg_loop(&mut self) {
        while self.running {
            self.event();
            self.update();
            self.render();
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

                Event::MouseButtonDown { x, y, .. } => {}

                _ => {}
            }
        }
    }

    pub fn update(&self) {
        println!("Mouse X: {} Y: {}", self.mouse_x, self.mouse_y)
    }

    pub fn render(&mut self) {
        let canvas = &mut self.renderer.canvas;

        canvas.set_draw_color(Color::RGB(0x00, 0x80, 0x80));
        canvas.clear();

        self.renderer.canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
    // pub fn add_component(&self, component: Component)
    // pub fn remove_component(&self, component: Component)
}
