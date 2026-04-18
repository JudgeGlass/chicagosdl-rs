use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn draw_window_frame(
    canvas: &mut WindowCanvas,
    window_x: u32,
    window_y: u32,
    window_width: u32,
    window_height: u32,
) {
    canvas.set_draw_color(Color::RGB(0xC0, 0xC0, 0xC0));
    canvas
        .fill_rect(Rect::new(
            window_x as i32,
            window_y as i32,
            window_width,
            window_height,
        ))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    canvas
        .draw_rect(Rect::new(
            (window_x + 1) as i32,
            (window_y + 1) as i32,
            window_width - 2,
            window_height - 2,
        ))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0x0, 0x0C, 0x75));
    canvas
        .fill_rect(Rect::new(
            (window_x + 4) as i32,
            (window_y + 4) as i32,
            window_width - 8,
            24,
        ))
        .unwrap();
}

pub fn draw_button_normal(
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: String,
) {
    canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
    canvas.draw_rect(Rect::new(x, y, width, height)).unwrap();
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    canvas
        .draw_rect(Rect::new(x, y, width - 1, height - 1))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
    canvas
        .draw_rect(Rect::new(x + 1, y + 1, width - 2, height - 2))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0xC0, 0xC0, 0xC0));
    canvas
        .fill_rect(Rect::new(x + 1, y + 1, width - 2, height - 2))
        .unwrap();

    // TODO: draw_string
}

pub fn draw_button_pushed(
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: String,
) {
    canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
    canvas.fill_rect(Rect::new(x, y, width, height)).unwrap();
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    canvas
        .fill_rect(Rect::new(x + 1, y + 1, width - 1, height - 1))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
    canvas
        .fill_rect(Rect::new(x + 1, y + 1, width - 2, height - 2))
        .unwrap();
    canvas.set_draw_color(Color::RGB(0xC0, 0xC0, 0xC0));
    canvas
        .fill_rect(Rect::new(x + 2, y + 2, width - 3, height - 3))
        .unwrap();

    // TODO: draw_string
}

pub fn draw_checkbox(canvas: &mut WindowCanvas, x: i32, y: i32, is_checked: bool, text: String) {
    // render_texture
    // draw_string
}

pub fn draw_input_buffer(canvas: &mut WindowCanvas, x: i32, y: i32, cols: i32, rows: i32) {
    let width: u32 = (cols * 8 + 10) as u32;
    let height: u32 = (rows * 8 + 10) as u32;

    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(Rect::new(x, y, width, height)).unwrap();

    canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
    canvas.draw_rect(Rect::new(x, y, width, height)).unwrap();

    canvas.set_draw_color(Color::RGB(0xC0, 0xC0, 0xC0));
    canvas
        .draw_rect(Rect::new(x + 1, y, width - 2, height))
        .unwrap();

    canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
    canvas
        .draw_rect(Rect::new(x + 2, y + 1, width - 3, height - 2))
        .unwrap();
}

pub fn draw_progress_bar(
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    percentage: f32,
) {
    canvas.set_draw_color(Color::BLACK);
    canvas.fill_rect(Rect::new(x, y, width, height)).unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas
        .fill_rect(Rect::new(x + 1, y + 1, width - 1, height - 1))
        .unwrap();

    canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
    canvas
        .fill_rect(Rect::new(x + 1, y + 1, width - 2, height - 1))
        .unwrap();

    canvas.set_draw_color(Color::RGB(0xC0, 0xC0, 0xC0));
    canvas
        .fill_rect(Rect::new(x + 2, y + 2, width - 3, height - 3))
        .unwrap();

    let str_percent = ((percentage * 100.0) as f32).to_string();

    canvas.set_draw_color(Color::RGB(0x0, 0x0C, 0x75));
    canvas
        .fill_rect(Rect::new(
            x + 2,
            y + 2,
            ((width - 2) as f32 * percentage) as u32,
            height - 4,
        ))
        .unwrap();
    // Draw the string
}
