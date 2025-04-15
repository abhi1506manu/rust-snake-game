use piston_window::{self, clear, types::Color, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

mod draw;
mod snake;
mod game;

use game::Game;
use draw::to_coord_u32;

const BLACK_COLOR:Color = [0.5,0.5,0.5,1.0];



fn main() {
    let (width, height) = (20,20);

    println!("width height {} {}",to_coord_u32(width),to_coord_u32(height));

    let window_result = WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
    .exit_on_esc(true)
    .build();
let mut window: PistonWindow = match window_result {
    Ok(win) => win,
    Err(e) => {
        eprintln!("Failed to build window: {:?}", e);
        std::process::exit(1);
    }
};
    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g,_|{
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg|{
            game.update(arg.dt);
        });
        
    }
}
