#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
extern crate gtk;
#[cfg(target_os = "linux")]
mod linux_gtk;

fn main() {
  linux_gtk::main();
//  std::thread::sleep(std::time::Duration::from_millis(5000));
}
