use chip_8_emulator::open_window;
fn main() {
    match open_window() {
        Err(e) => println!("Error: {}", e),
        _ => {}
    };
}
