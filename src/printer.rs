use atty::Stream;

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

/// Wraps text in the given color if stdout is a TTY.
pub fn colorize(text: &str, color: &str) -> String {
    if atty::is(Stream::Stdout) {
        format!("{}{}{}", color, text, RESET_COLOR)
    } else {
        text.to_string()
    }
}

/// Prints the owl ASCII art with aligned info lines, and additional info below.
pub fn print_owl(info: Vec<String>) {
    let lines = OWL_ART.lines().collect::<Vec<&str>>();
    let width = 20;
    let mut i = 0;
    for line in &lines {
        if i < info.len() {
            println!("{:<w$} {}", line, info[i], w = width);
        } else {
            println!("{}", line);
        }
        i += 1;
    }
    // print remaining info entries
    while i < info.len() {
        println!("{:<w$} {}", "", info[i], w = width);
        i += 1;
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
