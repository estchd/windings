#![allow(non_snake_case,non_camel_case_types)]
use std::mem::MaybeUninit;
use winapi::um::winuser::{LPMSG, MSG};
use winapi::shared::minwindef::{BOOL, LRESULT, DWORD, LPVOID, HINSTANCE};
use winapi::shared::windef::{HWND, HMENU};
use bitflags::bitflags;
use winapi::ctypes::c_int;
use std::ffi::CString;
use std::ptr::null_mut;
use crate::type_wrappers::conversion::convert_bool;

CONST_TO_ENUM!(const_enum N_CMD_SHOW,  winapi::ctypes::c_int {
    FORCEMINIMIZE = winapi::um::winuser::SW_FORCEMINIMIZE,
    HIDE = winapi::um::winuser::SW_HIDE,
    MAXIMIZE = winapi::um::winuser::SW_MAXIMIZE,
    MINIMIZE = winapi::um::winuser::SW_MINIMIZE,
    RESTORE = winapi::um::winuser::SW_RESTORE,
    SHOW = winapi::um::winuser::SW_SHOW,
    SHOWDEFAULT = winapi::um::winuser::SW_SHOWDEFAULT,
    SHOWMAXIMIZED = winapi::um::winuser::SW_SHOWMAXIMIZED,
    SHOWMINIMIZED = winapi::um::winuser::SW_SHOWMINIMIZED,
    SHOWMINNOACTIVE = winapi::um::winuser::SW_SHOWMINNOACTIVE,
    SHOWNA = winapi::um::winuser::SW_SHOWNA,
    SHOWNOACTIVATE = winapi::um::winuser::SW_SHOWNOACTIVATE,
    SHOWNORMAL = winapi::um::winuser::SW_SHOWNORMAL,
});

bitflags! {
    pub struct WindowExtendedStyle : DWORD {
        const ACCEPTFILES = winapi::um::winuser::WS_EX_ACCEPTFILES;
        const APPWINDOW = winapi::um::winuser::WS_EX_APPWINDOW;
        const CLIENTEDGE = winapi::um::winuser::WS_EX_CLIENTEDGE;
        const COMPOSITED = winapi::um::winuser::WS_EX_COMPOSITED;
        const CONTEXTHELP = winapi::um::winuser::WS_EX_CONTEXTHELP;
        const CONTROLPARENT = winapi::um::winuser::WS_EX_CONTROLPARENT;
        const DLGMODALFRAME = winapi::um::winuser::WS_EX_DLGMODALFRAME;
        const LAYERED = winapi::um::winuser::WS_EX_LAYERED;
        const LAYOUTRTL = winapi::um::winuser::WS_EX_LAYOUTRTL;
        const LEFT = winapi::um::winuser::WS_EX_LEFT;
        const LEFTSCROLLBAR = winapi::um::winuser::WS_EX_LEFTSCROLLBAR;
        const LTRREADING = winapi::um::winuser::WS_EX_LTRREADING;
        const MDICHILD = winapi::um::winuser::WS_EX_MDICHILD;
        const NOACTIVATE = winapi::um::winuser::WS_EX_NOACTIVATE;
        const NOINHERITLAYOUT = winapi::um::winuser::WS_EX_NOINHERITLAYOUT;
        const NOPARENTNOTIFY = winapi::um::winuser::WS_EX_NOPARENTNOTIFY;
        const NOREDIRECTIONBITMAP = winapi::um::winuser::WS_EX_NOREDIRECTIONBITMAP;
        const OVERLAPPEDWINDOW = winapi::um::winuser::WS_EX_OVERLAPPEDWINDOW;
        const PALETTEWINDOW = winapi::um::winuser::WS_EX_PALETTEWINDOW;
        const RIGHT = winapi::um::winuser::WS_EX_RIGHT;
        const RIGHTSCROLLBAR = winapi::um::winuser::WS_EX_RIGHTSCROLLBAR;
        const RTLREADING = winapi::um::winuser::WS_EX_RTLREADING;
        const STATICEDGE = winapi::um::winuser::WS_EX_STATICEDGE;
        const TOOLWINDOW = winapi::um::winuser::WS_EX_TOOLWINDOW;
        const TOPMOST = winapi::um::winuser::WS_EX_TOPMOST;
        const TRANSPARENT = winapi::um::winuser::WS_EX_TRANSPARENT;
        const WINDOWEDGE = winapi::um::winuser::WS_EX_WINDOWEDGE;
    }
}

bitflags! {
    pub struct WindowStyle : DWORD {
        const BORDER = winapi::um::winuser::WS_BORDER;
        const CAPTION = winapi::um::winuser::WS_CAPTION;
        const CHILD = winapi::um::winuser::WS_CHILD;
        const CHILDWINDOW = winapi::um::winuser::WS_CHILDWINDOW;
        const CLIPCHILDREN = winapi::um::winuser::WS_CLIPCHILDREN;
        const CLIPSIBLINGS = winapi::um::winuser::WS_CLIPSIBLINGS;
        const DISABLED = winapi::um::winuser::WS_DISABLED;
        const DLGFRAME = winapi::um::winuser::WS_DLGFRAME;
        const GROUP = winapi::um::winuser::WS_GROUP;
        const HSCROLL = winapi::um::winuser::WS_HSCROLL;
        const ICONIC = winapi::um::winuser::WS_ICONIC;
        const MAXIMIZE = winapi::um::winuser::WS_MAXIMIZE;
        const MAXIMIZEBOX = winapi::um::winuser::WS_MAXIMIZEBOX;
        const MINIMIZE = winapi::um::winuser::WS_MINIMIZE;
        const MINIMIZEBOX = winapi::um::winuser::WS_MINIMIZEBOX;
        const OVERLAPPED = winapi::um::winuser::WS_OVERLAPPED;
        const OVERLAPPEDWINDOW = winapi::um::winuser::WS_OVERLAPPEDWINDOW;
        const POPUP = winapi::um::winuser::WS_POPUP;
        const POPUPWINDOW = winapi::um::winuser::WS_POPUPWINDOW;
        const SIZEBOX = winapi::um::winuser::WS_SIZEBOX;
        const SYSMENU = winapi::um::winuser::WS_SYSMENU;
        const TABSTOP = winapi::um::winuser::WS_TABSTOP;
        const THICKFRAME = winapi::um::winuser::WS_THICKFRAME;
        const TILED = winapi::um::winuser::WS_TILED;
        const TILEDWINDOW = winapi::um::winuser::WS_TILEDWINDOW;
        const VISIBLE = winapi::um::winuser::WS_VISIBLE;
        const VSCROLL = winapi::um::winuser::WS_VSCROLL;
    }
}

#[inline]
pub fn GetMessageA(h_wnd: HWND, msg_filter_min: u32, msg_filter_max: u32) -> (bool, MSG) {
    let mut uninit_msg: MaybeUninit<MSG> = MaybeUninit::<MSG>::uninit();
    let lp_msg: LPMSG = uninit_msg.as_mut_ptr() as LPMSG;
    let value: BOOL;
    let msg: MSG;
    unsafe {
        value = winapi::um::winuser::GetMessageA(lp_msg, h_wnd, msg_filter_min, msg_filter_max);
        msg = uninit_msg.assume_init();
    }
    return (convert_bool(value), msg);
}

#[inline]
pub fn GetMessageW(h_wnd: HWND, msg_filter_min: u32, msg_filter_max: u32) -> (bool, MSG)
{
    let mut uninit_msg: MaybeUninit<MSG> = MaybeUninit::<MSG>::uninit();
    let lp_msg: LPMSG = uninit_msg.as_mut_ptr() as LPMSG;
    let value: BOOL;
    let msg: MSG;
    unsafe {
        value = winapi::um::winuser::GetMessageW(lp_msg, h_wnd, msg_filter_min, msg_filter_max);
        msg = uninit_msg.assume_init();
    }
    return (convert_bool(value), msg);
}

#[inline]
pub fn TranslateMessage(msg: &MSG) -> bool {
    let msg_ptr: *const MSG = msg as *const MSG;
    let value: BOOL;
    unsafe {
        value = winapi::um::winuser::TranslateMessage(msg_ptr);
    }
    return convert_bool(value);
}

#[inline]
pub fn DispatchMessageA(msg: &MSG) -> isize {
    let msg_ptr: *const MSG = msg as *const MSG;
    let value: LRESULT;
    unsafe {
        value = winapi::um::winuser::DispatchMessageA(msg_ptr);
    }
    return value;
}

#[inline]
pub fn DispatchMessageW(msg: &MSG) -> isize {
    let msg_ptr: *const MSG = msg as *const MSG;
    let value: LRESULT;
    unsafe {
        value = winapi::um::winuser::DispatchMessageW(msg_ptr);
    }
    return value;
}

#[inline]
pub fn ShowWindow(h_wnd: HWND, n_cmd_show: N_CMD_SHOW) -> bool {
    let n_cmd_show: c_int = n_cmd_show.into();
    let value: BOOL;
    unsafe {
        value = winapi::um::winuser::ShowWindow(h_wnd,n_cmd_show);
    }
    return convert_bool(value);
}

#[inline]
pub fn ShowWindowAsync(h_wnd: HWND, n_cmd_show: N_CMD_SHOW) -> bool {
    let n_cmd_show: c_int = n_cmd_show.into();
    let value: BOOL;
    unsafe {
        value = winapi::um::winuser::ShowWindowAsync(h_wnd,n_cmd_show);
    }
    return convert_bool(value);
}

pub fn CreateWindowExA(
    w_ex_style: WindowExtendedStyle,
    lp_class_name: CString,
    lp_window_name: CString,
    dw_style: WindowStyle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    wnd_parent: Option<HWND>,
    menu: Option<HMENU>,
    instance: HINSTANCE,
    lp_param: Option<LPVOID>
) -> HWND
{
    let class_name_ptr: *const i8 = lp_class_name.as_ptr();
    let window_name_ptr: *const i8 = lp_window_name.as_ptr();
    let ex_style_bits = w_ex_style.bits;
    let window_style_bits = dw_style.bits;

    let wnd_parent: HWND = wnd_parent.unwrap_or(null_mut());
    let menu: HMENU = menu.unwrap_or(null_mut());
    let param: LPVOID = lp_param.unwrap_or(null_mut());

    let window: HWND;
    unsafe {
        window = winapi::um::winuser::CreateWindowExA(
            ex_style_bits,
            class_name_ptr,
            window_name_ptr,
            window_style_bits,
            x,
            y,
            width,
            height,
            wnd_parent,
            menu,
            instance,
            param
        );
    }
    return window;
}

