use std::env;
use std::fs;
use sys_info;
use std::process::{Command as StdCommand, Stdio};
use std::io;

/// Returns the current user name or a default placeholder.
pub fn get_user() -> String {
    env::var("USER").unwrap_or_else(|_| "unknown_user".to_string())
}

/// Returns the hostname using sys_info or a default placeholder.
pub fn get_hostname() -> String {
    sys_info::hostname().unwrap_or_else(|_| "unknown_host".to_string())
}

/// Reads /etc/os-release for PRETTY_NAME, falls back to "Linux".
pub fn get_os_name() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(name) = line.strip_prefix("PRETTY_NAME=") {
                return name.trim_matches('"').to_string();
            }
        }
    }
    "Linux".to_string()
}

/// Returns the OS kernel release via sys_info.
pub fn get_kernel_release() -> String {
    sys_info::os_release().unwrap_or_else(|_| "Unknown kernel".to_string())
}

/// Returns the shell name stripped of path or a default.
pub fn get_shell() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "Unknown shell".to_string())
        .strip_prefix("/usr/bin/")
        .unwrap_or("Unknown shell")
        .to_string()
}

/// Returns the window manager from XDG_SESSION_DESKTOP or via ps detection.
pub fn get_window_manager() -> String {
    env::var("XDG_SESSION_DESKTOP")
        .or_else(|_| get_window_manager_from_ps())
        .unwrap_or_else(|_| "Unknown WM".to_string())
}

fn get_window_manager_from_ps() -> Result<String, io::Error> {
    if let Ok(output) = StdCommand::new("wmctrl").arg("-m").output() {
        let wm_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !wm_name.is_empty() {
            return Ok(wm_name);
        }
    }
    let output = StdCommand::new("ps")
        .arg("ax")
        .arg("-o")
        .arg("comm=")
        .stdout(Stdio::piped())
        .output()?;

    let wm_processes = vec![
        "i3", "gnome-shell", "xfwm4", "kwin_x11", "awesome", "openbox", "sway", "bspwm", 
        "xmonad", "cinnamon", "mate-session", "fluxbox", "dwm", "herbstluftwm", "stumpwm"
    ];

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find(|line| wm_processes.iter().any(|&wm| line.contains(wm)))
        .map(|wm| wm.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Window Manager not found"))
}

/// Returns the terminal emulator name or a default.
pub fn get_terminal() -> String {
    env::var("TERM_PROGRAM").or_else(|_| env::var("TERM")).unwrap_or_else(|_| "Unknown terminal".to_string())
}

/// Returns memory usage as used/total MB.
pub fn get_mem_usage() -> String {
    match sys_info::mem_info() {
        Ok(mem) => {
            let total_mb = mem.total / 1024;
            let avail_mb = mem.avail / 1024;
            let used_mb = total_mb - avail_mb;
            format!("Mem {} / {} MB", used_mb, total_mb)
        }
        Err(_) => "Mem Unknown".to_string(),
    }
}

/// Returns load average for 1, 5, and 15 minutes.
pub fn get_load_avg() -> String {
    match sys_info::loadavg() {
        Ok(load) => format!("Load {:.2} {:.2} {:.2}", load.one, load.five, load.fifteen),
        Err(_) => "Load Unknown".to_string(),
    }
}

/// Returns system uptime (Linux: /proc/uptime, else Unknown).
pub fn get_uptime() -> String {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(sec_str) = content.split_whitespace().next() {
            if let Ok(secs_f) = sec_str.parse::<f64>() {
                let secs = secs_f as u64;
                let days = secs / 86400;
                let hours = (secs % 86400) / 3600;
                let mins = (secs % 3600) / 60;
                return format!("Uptime {}d {:02}h {:02}m", days, hours, mins);
            }
        }
    }
    "Uptime Unknown".to_string()
}

/// Returns disk usage as used/total GB.
pub fn get_disk_info() -> String {
    match sys_info::disk_info() {
        Ok(disk) => {
            let total_gb = disk.total / (1024 * 1024);
            let free_gb = disk.free / (1024 * 1024);
            let used_gb = total_gb - free_gb;
            format!("Disk {} / {} GB", used_gb, total_gb)
        }
        Err(_) => "Disk Unknown".to_string(),
    }
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
        // Allow unknown or skip if error
        assert!(!wm.is_empty(), "Window manager should not be empty");
    }
}
