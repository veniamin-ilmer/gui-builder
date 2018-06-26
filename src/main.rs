extern crate winapi;

use std::ptr::{null,null_mut};
use winapi::um::winuser::*;
use winapi::shared::windef::{HWND, POINT, HBRUSH};
use winapi::shared::minwindef::{UINT,WPARAM,LPARAM,LRESULT};


extern "system" fn process_event (window_handle: HWND, msg_id: UINT,
                                w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        println!("{}", window_handle as usize);
        match msg_id {
            WM_CREATE => {},
            WM_CLOSE   => { DestroyWindow(window_handle); },
            WM_DESTROY =>  { PostQuitMessage(0); },
            _ => return DefWindowProcW(window_handle, msg_id, w_param, l_param)
        }
    }
    0
}

fn unicode_str(str: &str) -> Vec<u16> {
  use std::ffi::OsStr;
  use std::iter::once;
  use std::os::windows::ffi::OsStrExt;
  
  OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn create_button(window_handle: HWND) -> HWND {
  let class_name = unicode_str("Button");
  let title = unicode_str("Button");

  let x = 10;
  let y = 10;
  let width = 100;
  let height = 100;

  unsafe {
    let button_handle = CreateWindowExW(0,
                                        class_name.as_ptr(),
                                        title.as_ptr(),
                                        WS_CHILD | BS_NOTIFY | BS_TEXT | WS_VISIBLE,
                                        x, y, width, height,
                                        window_handle, null_mut(),
                                        null_mut(), null_mut());
    button_handle
  }
}

fn create_window() -> HWND {
  let class_name = unicode_str("test");
  let title = unicode_str("test");

  let x = 0;
  let y = 0;
  let width = 240;
  let height = 200;

  unsafe {
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(process_event),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: null_mut(),
        hIcon: null_mut(),
        hCursor: LoadCursorW(null_mut(), IDC_ARROW),
        hbrBackground: COLOR_WINDOW as HBRUSH,
        lpszMenuName: null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: null_mut()
    };
    RegisterClassExW(&class);

    let window_handle = CreateWindowExW(0,
                                        class_name.as_ptr(),
                                        title.as_ptr(),
                                        WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                                        x, y, width, height,
                                        null_mut(), null_mut(),
                                        null_mut(), null_mut());
    ShowWindow(window_handle, SW_SHOW);
    UpdateWindow(window_handle);
    window_handle
  }
}
 
fn handle_events() {
  let mut msg = MSG {
      hwnd: null_mut(),
      message: 0,
      wParam: 0,
      lParam: 0,
      time: 0,
      pt: POINT {
          x: 0,
          y: 0
      }
  };
  unsafe {
    while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }
  }
}
 
fn main() {
  let window_handle = create_window();
  create_button(window_handle);
  handle_events();
}