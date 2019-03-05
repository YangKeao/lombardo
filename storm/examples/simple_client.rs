extern crate storm;

fn main() {
    let mut display = storm::socket::Display::connect(None);
    println!("Connected to display");

    display.get_registry();

    loop {}
    display.disconnect();
    println!("Disconnected from display");
}
