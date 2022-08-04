pub fn os_type() -> &'static str {
  #[cfg(target_os = "linux")]
  return "linux";
  #[cfg(target_os = "windows")]
  return "windows";
  #[cfg(target_os = "macos")]
  return "macos";
}
