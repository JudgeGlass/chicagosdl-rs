use crate::component::Component;

use crate::ChicagoSDLTexture;
use crate::WindowMgr;
use crate::component::is_in_bounds;
use crate::compositor::*;
use sdl2::render::WindowCanvas;

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub text: String,
    pub enabled: bool,
    pub callback: fn(),
    pushed: bool,
    push_time: i32,
}

impl Component for Button {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        font_texture: &mut ChicagoSDLTexture,
        ui_texture: &mut ChicagoSDLTexture,
    ) {
        if self.enabled && !self.pushed {
            draw_button_normal(
                canvas,
                font_texture,
                self.x,
                self.y,
                self.width,
                self.height,
                &self.text,
            );
        } else {
            draw_button_pushed(
                canvas,
                font_texture,
                self.x,
                self.y,
                self.width,
                self.height,
                &self.text,
            );
        }
    }

    fn update(&mut self, window: &mut WindowMgr) {
        self.push_handler();
        if window.mouse1_pressed && self.enabled {
            if is_in_bounds(
                window.mouse_x as i32,
                window.mouse_y as i32,
                self.x,
                self.y,
                self.width as i32,
                self.height as i32,
            ) {
                self.pushed = true;
                self.push_time = 5;
                (self.callback)();
            }
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn enable(&mut self) {
        self.enabled = true;
    }
}

fn default_func() {}
impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: String) -> Button {
        Button {
            x,
            y,
            width,
            height,
            text,
            enabled: true,
            callback: default_func,
            pushed: false,
            push_time: 0,
        }
    }

    pub fn on_click(&mut self, callback: fn()) {
        self.callback = callback;
    }

    fn push_handler(&mut self) {
        if self.push_time > 0 {
            self.push_time -= 1;
        } else {
            self.pushed = false;
        }
    }
}
