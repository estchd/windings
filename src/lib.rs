#![allow(non_snake_case,dead_code)]
#[macro_use]
mod type_wrappers;

use std::panic::catch_unwind;

#[cfg(feature = "dxgi")]
pub mod dxgi;

#[cfg(feature = "unknown")]
pub mod unknown;


#[cfg(test)]
mod tests {
    use crate::{create_window};

    #[test]
    fn it_works() {
        create_window(|| {println!("Hello There");})
    }
}

use winapi::shared::windef::{HWND, HCURSOR, HICON, HBRUSH};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, BOOL, HINSTANCE};
use winapi::um::winuser::{LPMSG, MSG};
use std::ffi::{CString};
use std::mem::MaybeUninit;
use crate::type_wrappers::window_class::{ClassStyle, WindowClassExA};

struct Window {
    wnd_proc: fn(window: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM),

}

type SafeCallback = fn(HWND, UINT, WPARAM, LPARAM);

unsafe extern "system" fn unsafe_wnd_proc(window: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) {
    if let Err(e) = catch_unwind(|| {
    }) {
        // Code here must be panic-free.
        // Sane things to do:
        // log failure and/or kill the program
        eprintln!("{:?}", e);
    }
}

fn create_window(window_procedure: fn())
{
    let proc_ptr = window_procedure as isize;
    println!("{}", proc_ptr);
    let proc: fn();
    unsafe {
        proc = std::mem::transmute::<isize,fn()>(proc_ptr);
    }
    proc();
}

fn SafeCallback_to_isize(ptr: *mut isize) -> isize {
    return ptr as isize;
}

fn isize_to_ptr(val: isize) -> * mut isize {
    return val as *mut isize
}