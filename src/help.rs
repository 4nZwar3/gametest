use {
    std::io::{
        self,
        Write
    },
    colorful::{
        Colorful
    }
};
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn set_color(c: &str) -> &str {
    match c {
        "white" => "\x1b[39m",
        "whiter" => "\x1b[37m",
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "cyan" => "\x1b[34m",
        "purple" => "\x1b[35m",
        "blue" => "\x1b[36m",
        _ => "\x1b[30m",
    }
}
pub fn mv(line: usize, col: usize) {
    print!("\x1b[{};{}H", col, line);
}
pub fn yesorno() {
    print!("[{}/{}]: ","y".dark_gray(), "N".cyan());
}
pub fn get_char() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
pub fn answer() -> bool {
    let mut opc = String::new();
    io::stdin().read_line(&mut opc).unwrap();
    if let Ok('y') = opc.to_ascii_lowercase().trim().parse::<char>() {
        return true;
    };
    false
}
pub fn read_something(something: &str, place: &mut String) {
    clear_screen();
    print!("Insert {}: ", something);
    io::stdout().flush().expect("Error while flushing.");
    io::stdin().read_line(place).expect("Error while reading something.");
    *place = place.trim().to_string();
}