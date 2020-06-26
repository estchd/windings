#![allow(non_snake_case, dead_code)]

use bitflags::bitflags;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, HINSTANCE, LRESULT};
use winapi::shared::windef::{HCURSOR, HWND, HICON, HBRUSH};
use winapi::um::winuser::{WNDCLASSEXA, WNDCLASSEXW};
use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;
use crate::type_wrappers::error_handling_api::GetLastError;

bitflags! {
    pub struct ClassStyle : winapi::shared::minwindef::UINT {
        const BYTEALIGNCLIENT = winapi::um::winuser::CS_BYTEALIGNCLIENT;
        const BYTEALIGNWINDOW = winapi::um::winuser::CS_BYTEALIGNWINDOW;
        const CLASSDC = winapi::um::winuser::CS_CLASSDC;
        const DBLCLKS = winapi::um::winuser::CS_DBLCLKS;
        const DROPSHADOW = winapi::um::winuser::CS_DROPSHADOW;
        const GLOBALCLASS = winapi::um::winuser::CS_GLOBALCLASS;
        const HREDRAW = winapi::um::winuser::CS_HREDRAW;
        const NOCLOSE = winapi::um::winuser::CS_NOCLOSE;
        const OWNDC = winapi::um::winuser::CS_OWNDC;
        const PARENTDC = winapi::um::winuser::CS_PARENTDC;
        const SAVEBITS = winapi::um::winuser::CS_SAVEBITS;
        const VREDRAW = winapi::um::winuser::CS_VREDRAW;
    }
}

type UnsafeCallback = unsafe extern "system" fn(HWND, u32, usize, isize) -> isize;

#[derive(Clone)]
pub struct WindowClassExA {
    pub style: ClassStyle,
    pub wnd_proc: Option<UnsafeCallback>,
    pub class_extra: i32,
    pub window_extra: i32,
    pub instance: HINSTANCE,
    pub icon: Option<HICON>,
    pub cursor: Option<HCURSOR>,
    pub background_brush: Option<HBRUSH>,
    pub menu_name: CString,
    pub class_name: CString,
    pub small_icon: Option<HICON>,
}

impl Into<WNDCLASSEXA> for WindowClassExA {
    #[inline]
    fn into(self) -> WNDCLASSEXA {
        return WNDCLASSEXA {
            cbSize: mem::size_of::<WNDCLASSEXA>() as u32,
            style: self.style.bits,
            lpfnWndProc: self.wnd_proc,
            cbClsExtra: self.class_extra,
            cbWndExtra: self.window_extra,
            hInstance: self.instance,
            hIcon: self.icon.unwrap_or(null_mut()),
            hCursor: self.cursor.unwrap_or(null_mut()),
            hbrBackground: self.background_brush.unwrap_or(null_mut()),
            lpszMenuName: self.menu_name.as_ptr(),
            lpszClassName: self.class_name.as_ptr(),
            hIconSm: self.small_icon.unwrap_or(null_mut())
        }
    }
}

#[inline]
pub fn RegisterClassExA(class: WindowClassExA) -> Result<u16,u32>{
    let winapi_window_class: WNDCLASSEXA = class.into();
    let class_ptr = &winapi_window_class as *const WNDCLASSEXA;
    let value: u16;
    unsafe {
        value = winapi::um::winuser::RegisterClassExA(class_ptr);
    }
    return match value {
        0 => Err(GetLastError()),
        _ => Ok(value)
    };
}