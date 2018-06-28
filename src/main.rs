#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
mod windows;

fn main() {
  windows::main();
//  std::thread::sleep(std::time::Duration::from_millis(5000));
}
