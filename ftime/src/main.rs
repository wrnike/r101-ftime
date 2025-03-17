use chrono::Local;
use owo_colors::OwoColorize;

const FERRIS: &str = r#"
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \
"#;

fn main() {
    print!("\x1B[2J");
    print!("\x1B[?25l");

    loop {
        let local = Local::now().format("%H:%M:%S");
        println!("   {}", local.blue());
        println!("{}", FERRIS.yellow());
        print!("\x1B[2J");
        print!("\x1B[7A");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
