#[cfg(windows)]
pub const EOL: &'static str = "\r\n";
#[cfg(not(windows))]
pub const EOL: &'static str = "\n";
