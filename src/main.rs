#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
extern crate gtk;
#[cfg(target_os = "linux")]
mod linux_gtk;

fn main() {
  windows::main();
}
