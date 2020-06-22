pub fn GetLastError() -> u32 {
    let value: u32;
    unsafe {
        value = winapi::um::errhandlingapi::GetLastError();
    }
    return value;
}