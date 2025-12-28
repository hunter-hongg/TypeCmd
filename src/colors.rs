//! Colorized console output utilities

/// ANSI color codes
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const GRAY: &str = "\x1b[90m";
pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";

/// Colorize text with the given ANSI color code
pub fn colorize(text: &str, color_code: &str) -> String {
    format!("{}{}{}", color_code, text, RESET)
}

/// Print error message in red
pub fn print_error(msg: &str) {
    println!("{}错误: {}{}", RED, msg, RESET);
}

/// Print success message in green
pub fn print_success(msg: &str) {
    println!("{}{}{}", GREEN, msg, RESET);
}

/// Print info message in blue
pub fn print_info(msg: &str) {
    println!("{}{}{}", BLUE, msg, RESET);
}

/// Print warning message in yellow
pub fn print_warn(msg: &str) {
    println!("{}警告: {}{}", YELLOW, msg, RESET);
}

/// Print message in gray
pub fn print_gray(msg: &str) {
    println!("{}{}{}", GRAY, msg, RESET);
}

/// Print message in cyan
pub fn print_cyan(msg: &str) {
    println!("{}{}{}", CYAN, msg, RESET);
}

/// Format text as bold
pub fn bold(text: &str) -> String {
    format!("{}{}{}", BOLD, text, RESET)
}