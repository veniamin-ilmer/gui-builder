extern crate winapi;

use std::ptr::{null,null_mut};
use winapi::um::winuser::*;
use winapi::um::commctrl::{SetWindowSubclass, DefSubclassProc};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::basetsd::{UINT_PTR,DWORD_PTR};
use winapi::shared::windef::{HWND, POINT, HBRUSH, HMENU};
use winapi::shared::minwindef::{UINT,WPARAM,LPARAM,LRESULT};

const EVENTS_DISPATCH_ID: UINT_PTR = 1234321;

extern "system" fn process_event(object_handle: HWND, msg_id: UINT,
                                w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        match msg_id {
            WM_CREATE => { },
            WM_CLOSE   => { DestroyWindow(object_handle); },
            WM_DESTROY =>  { PostQuitMessage(0); },
            _ => {}
        }
      DefWindowProcW(object_handle, msg_id, w_param, l_param)
    }
}

extern "system" fn process_subevent(object_handle: HWND, msg_id: UINT,w_param: WPARAM, l_param: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    match msg_id {
        WM_CREATE => { println!("WM_CREATE {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_CLOSE   => { println!("WM_CLOSE {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_DESTROY =>  { println!("WM_DESTROY {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_COMMAND =>  { println!("WM_COMMAND {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_NOTIFY =>  { println!("WM_NOTIFY {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_LBUTTONDOWN =>  { println!("WM_LBUTTONDOWN {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_PAINT =>  { println!("WM_PAINT {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_TIMER =>  { println!("WM_TIMER {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        WM_MOUSEMOVE => { println!("WM_MOUSEMOVE {} {} {}", object_handle as usize, l_param as usize, w_param as usize); },
        _ => {}
    }
    unsafe {
      DefSubclassProc(object_handle, msg_id, w_param, l_param)
    }
}


fn unicode_str(str: &str) -> Vec<u16> {
  use std::ffi::OsStr;
  use std::iter::once;
  use std::os::windows::ffi::OsStrExt;
  
  OsStr::new(str).encode_wide().chain(once(0)).collect()
}


fn create_button(window_handle: HWND) -> HWND {
  let class_name = unicode_str("BUTTON");
  let title = unicode_str("Button");

  let x = 10;
  let y = 10;
  let width = 100;
  let height = 100;

  unsafe {

    let button_handle = CreateWindowExW(0,
                                        class_name.as_ptr(),
                                        title.as_ptr(),
                                        WS_CHILD | WS_VISIBLE | BS_NOTIFY | BS_TEXT,
                                        x, y, width, height,
                                        window_handle, null_mut(),
                                        null_mut(), null_mut());
    println!("Button Handle: {}", button_handle as usize);
    
    SetWindowSubclass(button_handle, Some(process_subevent), EVENTS_DISPATCH_ID, 0);
    
    button_handle
    
  }
}


fn create_textbox(window_handle: HWND) -> HWND {
  let class_name = unicode_str("EDIT");

  let x = 110;
  let y = 10;
  let width = 100;
  let height = 50;

  unsafe {

    let text_handle = CreateWindowExW(WS_EX_CLIENTEDGE,
                                        class_name.as_ptr(),
                                        null(),
                                        WS_CHILD | WS_VISIBLE | ES_LEFT,
                                        x, y, width, height,
                                        window_handle, null_mut(),
                                        null_mut(), null_mut());
    println!("Text Handle: {}", text_handle as usize);
    
    SetWindowSubclass(text_handle, Some(process_subevent), EVENTS_DISPATCH_ID, 0);
    
    text_handle
    
  }
}


fn create_window() -> HWND {
  let class_name = unicode_str("WINDOW");
  let title = unicode_str("test");

  let x = 0;
  let y = 0;
  let width = 240;
  let height = 200;

  unsafe {
//    let hmod = GetModuleHandleW(null_mut());

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
    println!("Window Handle: {}", window_handle as usize);
    SetWindowSubclass(window_handle, Some(process_subevent), EVENTS_DISPATCH_ID, 0);

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
  create_textbox(window_handle);
  handle_events();
}