use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


pub mod chip8;
use crate::chip8::Chip8;


/// open sdl2 window and create a new chip8 display
/// draw display to screen in a loop
/// display might onl be updated when necessary instead of 60 FPS for better optimization
pub fn open_window() -> Result<(), std::io::Error> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut my_chip8 = Chip8::new_default(canvas);
    match my_chip8.load_file_to_mem() {
        Err(e) => return Err(e),
        _ => {}
    };
    my_chip8.start_device();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        //60 FPS        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
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

    #[test]
    pub fn instruction_00e0() {
        let mut test_device = get_test_device();
        test_device.display.pixels = [[0xB3; 8]; 32];
        test_device.memory[0x200] = 0x00;
        test_device.memory[0x201] = 0xe0;
        test_device.start_device();

        assert_eq!(test_device.display.pixels, [[0; 8]; 32]);
    }
}
