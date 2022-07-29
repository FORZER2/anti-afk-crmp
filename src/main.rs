use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

fn main() {
    let delay = 5;

    println!("Anti-AFK by FORZER [YouTube] :3");
    println!("Anti-AFK запуститься через {} секунд.", delay);
    
    thread::sleep(Duration::from_secs(delay));
    
    let mut enigo = Enigo::new();

    println!("Anti-AFK запущен!");
    loop {
        enigo.key_click(Key::Layout('w'));
        thread::sleep(Duration::from_secs(delay));
        enigo.key_click(Key::Layout('s'));
        thread::sleep(Duration::from_secs(delay));
    }
}
