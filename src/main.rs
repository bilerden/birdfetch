mod info;
mod printer;

use std::env;
use std::thread;
use info::{get_user, get_hostname, get_os_name, get_kernel_release, get_shell, get_window_manager, get_terminal};
use printer::{colorize, print_owl, print_color_blocks, FOOTER_COLOR, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN};

fn main() {
    let env_color = env::var("ANSI_COLOR").unwrap_or_else(|_| "".to_string());
    let default_os_color = if !env_color.is_empty() {
        env_color.clone()
    } else {
        GREEN.to_string()
    };

    let user_handle = thread::spawn(get_user);
    let hostname_handle = thread::spawn(get_hostname);
    let os_handle = thread::spawn(get_os_name);
    let kernel_handle = thread::spawn(get_kernel_release);
    let shell_handle = thread::spawn(get_shell);
    let wm_handle = thread::spawn(get_window_manager);
    let term_handle = thread::spawn(get_terminal);

    let user = user_handle.join().unwrap();
    let hostname = hostname_handle.join().unwrap();
    let os_name = os_handle.join().unwrap();
    let kernel_rel = kernel_handle.join().unwrap();
    let shell = shell_handle.join().unwrap();
    let wm = wm_handle.join().unwrap();
    let terminal = term_handle.join().unwrap();

    let title = colorize(&format!("{}@{}", user, hostname), RED);
    let os_info = colorize(&format!("OS {}", os_name), &default_os_color);
    let kernel_info = colorize(&format!("Kernel {}", kernel_rel), YELLOW);
    let shell_info = colorize(&format!("Shell {}", shell), BLUE);
    let wm_info = colorize(&format!("WM {}", wm), MAGENTA);
    let terminal_info = colorize(&format!("Terminal {}", terminal), CYAN);

    print_owl(vec![title, os_info, kernel_info, shell_info, wm_info, terminal_info]);
    println!("{}", FOOTER_COLOR);
    print_color_blocks();
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
    fn test_get_window_manager() {
        let wm = get_window_manager();
        assert!(!wm.is_empty(), "Window manager should not be empty");
    }
}
