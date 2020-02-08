mod bindings;

// use std::fs::File;
// use std::io::prelude::*;

/// Print the contents of the clipboard to filename
#[no_mangle]
pub extern "C" fn print_clipboard_file(_filename: *const u16, _len: usize) {
    let filename = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(_filename, _len) });
    println!("Filename: {}!", filename);

    let mut cb = bindings::Clipboard::new().unwrap();
    match cb.unicode_text() {
        Some(mut data_ptr) => {
            let _dataguard = data_ptr.lock().unwrap();
            println!("Contents of clipboard:\n{}", *_dataguard);
        }
        None => {
            println!("Contents of clipboard was not unicode text.");
        }
    }
}
