use std::marker::PhantomData;
use winapi::um::*;

pub struct Clipboard;

pub struct ClipData<'a> {
    handle: winapi::shared::minwindef::HGLOBAL,
    _marker: PhantomData<&'a mut Clipboard>,
}

pub struct ClipDataGuard<'a> {
    handle: winapi::shared::minwindef::HGLOBAL,
    data: String,
    _marker: PhantomData<&'a mut ClipData<'a>>,
}
impl<'a> std::ops::Deref for ClipDataGuard<'a> {
    type Target = String;

    fn deref(&self) -> &String {
        &self.data
    }
}

impl<'a> Drop for ClipDataGuard<'a> {
    fn drop(&mut self) {
        unsafe { winbase::GlobalUnlock(self.handle) };
    }
}
extern "C" {
    fn wcslen(ws: *const u16) -> usize;
}

impl<'a> ClipData<'a> {
    pub fn lock(&'a mut self) -> Option<ClipDataGuard<'a>> {
        let result = unsafe { winbase::GlobalLock(self.handle) as *const u16 };
        if result == std::ptr::null_mut() {
            None
        } else {
            Some(ClipDataGuard {
                handle: self.handle,
                data: String::from_utf16_lossy(unsafe {
                    std::slice::from_raw_parts(result, wcslen(result))
                }),
                _marker: PhantomData,
            })
        }
    }
}

impl Clipboard {
    pub fn new() -> Result<Clipboard, u32> {
        let result = unsafe { winuser::OpenClipboard(std::ptr::null_mut()) };
        if result == 0 {
            Err(unsafe { errhandlingapi::GetLastError() })
        } else {
            Ok(Clipboard {})
        }
    }
    pub fn unicode_text<'a>(&'a mut self) -> Option<ClipData<'a>> {
        let hndl = unsafe { winuser::GetClipboardData(winuser::CF_UNICODETEXT) };
        if hndl == std::ptr::null_mut() {
            None
        } else {
            Some(ClipData {
                handle: hndl,
                _marker: PhantomData,
            })
        }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        unsafe { winuser::CloseClipboard() };
    }
}
