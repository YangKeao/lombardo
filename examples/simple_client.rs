extern crate storm;

fn main() {
    let mut display = storm::client::Display::connect(None);
    println!("Connected to display");

    display.get_registry();
    display.disconnect();
    println!("Disconnected from display");
}
