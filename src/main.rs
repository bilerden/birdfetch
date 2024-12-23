use std::env;
use std::fs;
use uname::uname;
use std::process::{Command as StdCommand, Stdio};

static FOOTER_COLOR: &str = "\x1b[31m\x1b[0m"; 
static RESET_COLOR: &str = "\x1b[0m"; 

const OWL_ART: &str = r#"
 ^ ^
(O,O)
(   )
-"-"--
"#;

fn main() {
    let env_color = env::var("ANSI_COLOR").unwrap_or_else(|_| "".to_string());

    let user = env::var("USER").unwrap_or_else(|_| "unknown_user".to_string());
    let hostname = StdCommand::new("hostname")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown_host".to_string());
    let title = format!("\x1b[31m{}@{}\x1b[0m", user, hostname);

    let os_info = format!(
        "{}OS\x1b[0m {}{}",
        if !env_color.is_empty() {
            env_color
        } else {
            "\x1b[32m".to_string()
        },
        get_os_name(),
        "\x1b[37m"
    );
    
    let kernel_info = format!("\x1b[33mKernel\x1b[0m \x1b[37m{}", uname().map(|u| u.release).unwrap_or_else(|_| "Unknown kernel".to_string()));
    
    let shell_info = format!("\x1b[34mShell\x1b[0m \x1b[37m{}", env::var("SHELL").unwrap_or_else(|_| "Unknown shell".to_string()).strip_prefix("/usr/bin/").unwrap_or("Unknown shell"));
    
    let wm_info = format!("\x1b[35mWM\x1b[0m \x1b[37m{}", env::var("XDG_SESSION_DESKTOP").or_else(|_| get_window_manager_from_ps()).unwrap_or_else(|_| "Unknown WM".to_string()));
    
    let terminal_info = format!("\x1b[36mTerminal\x1b[0m \x1b[37m{}", env::var("TERM_PROGRAM").or_else(|_| env::var("TERM")).unwrap_or_else(|_| "Unknown terminal".to_string()));

    print_owl(title, os_info, kernel_info, shell_info, wm_info, terminal_info);
    println!("{}", FOOTER_COLOR);
    print_color_blocks();
}

fn print_owl(title: String, os_info: String, kernel_info: String, shell_info: String, wm_info: String, terminal_info: String) {
    let owl_lines = OWL_ART.lines().collect::<Vec<&str>>();
    let info = vec![
        (title, "\x1b[31m"),
        (os_info, "\x1b[32m"),
        (kernel_info, "\x1b[33m"),
        (shell_info, "\x1b[34m"),
        (wm_info, "\x1b[35m"),
        (terminal_info, "\x1b[36m"),
    ];

    for (i, line) in owl_lines.iter().enumerate() {
        if i < info.len() {
            println!("{:<20} {}", line, format!("{}{}{}", info[i].1, info[i].0, RESET_COLOR));
        } else {
            println!("{}", line);
        }
    }
}

fn print_color_blocks() {
    for i in 0..9 {
        print!("\x1b[48;5;{}m   ", i);
    }
    print!("{}", RESET_COLOR);
    println!();
}

fn get_window_manager_from_ps() -> Result<String, std::io::Error> {
    let output = StdCommand::new("ps")
        .arg("ax")
        .arg("-o")
        .arg("comm=")
        .stdout(Stdio::piped())
        .output()?;
        
    let wm_processes = vec!["i3", "gnome-shell", "xfwm4", "kwin_x11", "awesome", "openbox"];
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find(|line| wm_processes.iter().any(|&wm| line.contains(wm)))
        .map(|wm| wm.to_string())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Window Manager not found"))
}

fn get_os_name() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.splitn(2, '=').nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }
    }
    "Linux".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_name() {
        let os_name = get_os_name();
        assert!(!os_name.is_empty(), "OS name should not be empty");
    }

    #[test]
    fn test_get_window_manager_from_ps() {
        if let Ok(wm) = get_window_manager_from_ps() {
            assert!(!wm.is_empty(), "Window manager should not be empty");
        } else {
            println!("Skipping window manager test in CI/CD");
        }
    }
}
