extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::rand;
use std::i32;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::prelude::*;
// use sdl2::ttf::Sdl2TtfContext;

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

    let mut rng = rand::rng();
    let mut diagonal = 0;
    let mut left_rect = Rect::new(50, 350, 40, 80);
    let mut ball = Rect::new(viewport.w/2 - 10, viewport.h/2 - 10, 20, 20);
    let mut right_rect = Rect::new(710, 50, 40, 80);

    let mut player_score =0;
    let mut enemy_score =0;

    let mut towards_player = true;

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
        if left_rect.has_intersection(ball){
            println!("Hit");
            diagonal = rng.random_range(-5..5);
            towards_player = false;
        }
        if right_rect.has_intersection(ball){
            println!("Hit ai paddle");
            diagonal = rng.random_range(-5..5);
            towards_player = true;
        }
        if towards_player {
            ball.x -= 3;

        }
        else {
            ball.x += 3;
        }
        if !is_touching_edge(&ball, 800, 600) {
            
            ball.y += diagonal; 
        }else {
            ball.y -= diagonal*2;
            diagonal = diagonal * -1;
        }
        if is_touching_win(&ball, 800, 600){
            println!("Win");
            //reset scene
            // add point for left side
            ball.x = viewport.w/2 - 10;
            ball.y = viewport.h/2 - 10;
            left_rect.y = viewport.h/2 - 80;

            right_rect.y = viewport.h/2 - 80;

            diagonal = rng.random_range(-5..5);
            towards_player = rng.random_bool(1.0/3.0);
            player_score += 1;
        }else if is_touching_loss(&ball, 800, 600)
        {
            println!("Lose");
            //reset scene
            // add point for right side
            ball.x = viewport.w/2 - 10;
            ball.y = viewport.h/2 - 10;
            left_rect.y = viewport.h/2 - 80;

            right_rect.y = viewport.h/2 - 80;

            diagonal = rng.random_range(-5..5);
            enemy_score += 1;
            towards_player = rng.random_bool(1.0/3.0);
        }


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn is_touching_edge(rect: &Rect, screen_width: i32, screen_height: i32) -> bool{
    rect.top() <= 0 || rect.bottom() > screen_height
}

fn is_touching_win(rect:&Rect, screen_width: i32, screen_height: i32) -> bool {
    rect.right() > screen_width
}
fn is_touching_loss(rect:&Rect, screen_width: i32, screen_height: i32) -> bool {
    rect.left() <= 0
}