#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
extern crate gtk;
#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
mod linux_gtk;

#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
mod macos;

//#[cfg(target_os = "macos")]
//extern crate core_foundation;
//#[cfg(target_os = "macos")]
//extern crate core_graphics;

fn main() {
  macos::main();
}
