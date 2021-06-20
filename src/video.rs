use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Video {
    canvas: sdl2::render::WindowCanvas,
}

impl Video {
    pub fn new(sdl_context: &sdl2::Sdl) -> Video {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip-8 Emulator", 640, 320)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Video { canvas: canvas }
    }
    pub fn update(&mut self, video_out: &[[u8; 64]; 32]) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..video_out.len() {
            for j in 0..video_out[i].len() {
                if video_out[i][j] == 0x1 {
                    let x: i32 = (j * 10) as i32;
                    let y: i32 = (i * 10) as i32;
                    self.canvas
                        .fill_rect(Rect::new(x, y, 10, 10))
                        .expect("Unable to draw rectange.");
                }
            }
        }
        self.canvas.present();
    }
}
