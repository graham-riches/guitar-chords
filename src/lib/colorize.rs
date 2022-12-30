//! Crude ANSI terminal coloring
//! 
//! Provides some crude ANSI coloring functionality with escapes
//! for normal strings, which lets you build up strings with color 
//! in any context (e.g., inside a function)

/// Available colors for both foreground and background coloring
pub enum Color {
    Default = -30,
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 60,
    BrightRed = 61,
    BrightGreen = 62,
    BrightYellow = 63,
    BrightBlue = 64,
    BrightMagenta = 65,
    BrightCyan = 66,
    BrightWhite = 67,
}

/// Provides foreground and background terminal coloring functionality
pub trait Colored {

    /// Change foreground color of a string/str
    /// 
    /// # Arguments
    /// `c` - The color to use
    /// 
    /// # Examples
    /// ```
    /// use colorize::{Colored, Color};
    /// let s = "hello_world".fg(Color::BrightBlue);
    /// ```
    fn fg(&self, c: Color) -> String;

    /// Change the background color of a string/str
    /// 
    /// # Arguments
    /// `c` - The color to use
    /// 
    /// # Examples
    /// ```
    /// use colorize::{Colored, Color};
    /// let s = "hello_world".bg(Color::BrightBlue);
    /// ```
    fn bg(&self, c: Color) -> String;
}

/// Implementation of Colored trait for str/&str
impl Colored for str {
    fn fg(&self, c: Color) -> String {
        format!("\x1b[{}m{}\x1b[0m", c as i32 + 30, self)
    }

    fn bg(&self, c: Color) -> String {
        format!("\x1b[{}m{}\x1b[0m", c as i32 + 40, self)
    }
}