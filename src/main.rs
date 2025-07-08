extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Pong in Rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let viewport = Rect::new(0, 0, 800, 600);
    canvas.set_viewport(viewport);
    canvas.set_clip_rect(viewport);

    let mut left_rect = Rect::new(50, 50, 40, 80);
    let mut ball = Rect::new(viewport.w/2 - 10, viewport.h/2 - 10, 20, 20);
    let right_rect = Rect::new(710, 50, 40, 80);
    
    let mut event_pump = sdl_context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    'running: loop {
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // canvas.present();
        canvas.set_draw_color(Color::RGB(255,255,255));
        canvas.fill_rect(left_rect).unwrap();
        canvas.fill_rect(right_rect).unwrap();
        canvas.fill_rect(ball).unwrap();
        // canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    if left_rect.y <520{
                    left_rect.y += 10;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    if left_rect.y > 0 {

                    left_rect.y -= 10;
                    }
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        // if left_rect.has_intersection(test_rect){
        //     println!("Intersect");
        // }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}