use chip_8_emulator::open_window;
fn main() {
    let args = std::env::args();
    match open_window(args) {
        Err(e) => println!("Error: {}", e),
        _ => {}
    };
}