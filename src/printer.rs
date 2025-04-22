pub const FOOTER_COLOR: &str = "\x1b[31m\x1b[0m";
const RESET_COLOR: &str = "\x1b[0m";

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";

const OWL_ART: &str = r#"
 ^ ^
(O,O)
(   )
-"-"--
"#;

/// Wraps text in the given color with reset.
pub fn colorize(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, RESET_COLOR)
}

/// Prints the owl ASCII art with aligned info lines.
pub fn print_owl(info: Vec<String>) {
    let lines = OWL_ART.lines().collect::<Vec<&str>>();
    for (i, line) in lines.iter().enumerate() {
        if i < info.len() {
            println!("{:<20} {}", line, info[i]);
        } else {
            println!("{}", line);
        }
    }
}

/// Displays color swatches.
pub fn print_color_blocks() {
    for i in 0..9 {
        print!("\x1b[48;5;{}m   ", i);
    }
    print!("{}", RESET_COLOR);
    println!();
}
