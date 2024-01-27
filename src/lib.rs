use sdl2::pixels::Color;
use std::env;


pub mod chip8;
pub mod instruction;
use crate::chip8::Chip8;

pub struct Config {
    filename: String,
    is_debug: bool
}

impl Config {
    pub fn build(filename: &str, is_debug: bool ) -> Config {
        Config {
            filename: String::from(filename), 
            is_debug: is_debug
        }
    }
}


/// open sdl2 window and create a new chip8 display
/// draw display to screen in a loop
/// display might onl be updated when necessary instead of 60 FPS for better optimization
pub fn open_window(args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    let cfg = match handle_args(args) {
        Ok(c) => c,
        Err(e) => return Err(e)
    };

    let Config {filename, is_debug} = cfg;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let event_pump = sdl_context.event_pump().unwrap();

    let mut my_chip8 = Chip8::new_default(canvas);
    match my_chip8.start_device(&filename, is_debug, event_pump) {
        Err(e) => println!("Error: {}", e),
        Ok(()) => {}
    };

    Ok(())
}

pub fn handle_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();
    let filename = match args.next() {
        Some(s) => s,
        None => return Err("No filename provided!")
    };

    let is_debug = env::var("CH8_DEBUG").is_ok();
    println!("IS_DEBUG: {}", is_debug);

    Ok(Config::build(&filename, is_debug))
}





#[cfg(test)]
mod tests {
    use crate::chip8::Chip8;
    use sdl2::pixels::Color;

    fn get_test_device() -> Chip8 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8 emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        Chip8::new_default(canvas)
    }
}
