use std::env;
use std::fs;
use sys_info;
use std::process::Command as StdCommand;
use std::io;
use std::{thread, time::Duration};

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

/// Returns memory usage percentage (used/total * 100).
pub fn get_mem_usage() -> String {
    match sys_info::mem_info() {
        Ok(mem) => {
            let used = mem.total.saturating_sub(mem.avail) as f64;
            let total = mem.total as f64;
            if total > 0.0 {
                format!("{:.0}%", used / total * 100.0)
            } else {
                "Unknown".to_string()
            }
        }
        Err(_) => "Unknown".to_string(),
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
                return format!("{}d {:02}h {:02}m", days, hours, mins);
            }
        }
    }
    "Unknown".to_string()
}

/// Returns disk usage as used percentage.
pub fn get_disk_info() -> String {
    match sys_info::disk_info() {
        Ok(disk) => {
            let used = (disk.total.saturating_sub(disk.free)) as f64;
            let total = disk.total as f64;
            if total > 0.0 {
                format!("{:.0}%", used / total * 100.0)
            } else {
                "Unknown".to_string()
            }
        }
        Err(_) => "Unknown".to_string(),
    }
}

/// Returns CPU usage percentage by sampling /proc/stat twice.
pub fn get_cpu_usage() -> String {
    let stat1 = fs::read_to_string("/proc/stat").unwrap_or_default();
    let vals1: Vec<u64> = stat1
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .take(8)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    if vals1.len() < 4 {
        return "Unknown".to_string();
    }
    let idle1 = vals1[3] + vals1.get(4).cloned().unwrap_or(0);
    let total1: u64 = vals1.iter().sum();
    thread::sleep(Duration::from_millis(100));
    let stat2 = fs::read_to_string("/proc/stat").unwrap_or_default();
    let vals2: Vec<u64> = stat2
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .take(8)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    if vals2.len() < 4 {
        return "Unknown".to_string();
    }
    let idle2 = vals2[3] + vals2.get(4).cloned().unwrap_or(0);
    let total2: u64 = vals2.iter().sum();
    let diff_total = total2.saturating_sub(total1);
    let diff_idle = idle2.saturating_sub(idle1);
    if diff_total == 0 {
        return "0%".to_string();
    }
    let usage = (diff_total - diff_idle) as f64 / diff_total as f64 * 100.0;
    format!("{:.0}%", usage)
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
