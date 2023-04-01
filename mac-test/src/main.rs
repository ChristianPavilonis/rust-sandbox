use objc::runtime::{Class, Object};
use objc::{sel, sel_impl};
use std::mem::transmute;
extern "C" {
    fn CGWindowListCopyWindowInfo(option: u32, relativeToWindow: u32) -> Object;
}
fn get_window_list() -> Object {
    let option = 2;
    // kCGWindowListExcludeDesktopElements
    let relative_to_window = 0;
    // kCGNullWindowID
    unsafe {
        CGWindowListCopyWindowInfo(option, relative_to_window)
    }
}
fn main() {
    let window_list = get_window_list();

    println!("{:?}", window_list);

//    let current_window_info = unsafe {
//        window_list
//            .enumerate()
//            // Enumerate the window list
//            .find(|(_, window)| {
//                // Filter out all windows except the foreground window
//                let layer = window.get("kCGWindowLayer") as u32;
//                let is_on_screen = window.get("kCGWindowIsOnscreen") as u32 != 0;
//                let is_foreground_window = layer == 0 && is_on_screen;
//                is_foreground_window
//            })
//    };
//
//    // The window ID of the currently focused window
//    let current_window_id = match current_window_info {
//        Some((_, window_info)) => window_info.get("kCGWindowNumber") as u32,
//        None => 0,
//    };
//    println!("The current window ID is {}", current_window_id);
}
