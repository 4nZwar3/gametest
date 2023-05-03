use colorful::{
    Colorful
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
    print!("[{}/{}]: ","y".light_gray(), "N".cyan());
}