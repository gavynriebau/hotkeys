fn main() {
    windows::build! {
        Windows::Win32::Foundation::HWND,
        Windows::Win32::UI::KeyboardAndMouseInput::{RegisterHotKey, HOT_KEY_MODIFIERS},
        Windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG},
    };
}
