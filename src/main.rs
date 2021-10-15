// Uncomment this line to generate an .exe that doesn't spawn a console window when executed.
//#![windows_subsystem = "windows"]

windows::include_bindings!();

use Windows::Win32::Foundation::*;
use Windows::Win32::UI::KeyboardAndMouseInput::*;
use Windows::Win32::UI::WindowsAndMessaging::*;
use std::process::{Command, exit};

/// This code is an example of how to use Windows APIs, from rust, to register a hotkey listener
/// and spawn a calc.exe instance every time 'Alt+b' is pressed.
///
/// Reference:
/// * https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
/// * https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew

fn main() {
    println!("Starting...");

    let register_hotkey_result = unsafe {
        RegisterHotKey(
            HWND(0),                // NULL - messages will be posted to message queue for calling thread.
            1,                      // id for the hotkey
            MOD_ALT | MOD_NOREPEAT, // Modifiers
            0x42                    // 'b'
        )
    };

    if register_hotkey_result.as_bool() {
        println!("Successfully registered hotkey listener");
    } else {
        println!("Failed to register hotkey listener");
        exit(-1);
    }

    let mut msg: MSG = MSG::default();
    loop {
        let get_message_result = unsafe {
             GetMessageW(
                &mut msg,   // Store MSG structure info here
                HWND(0),    // NULL - retrieve message from current thread
                0,          // msgFilterMin
                0           // msgFilterMax
            )
        };

        if get_message_result.as_bool() {
            println!("Received new hotkey message: {:#?}", msg);

            Command::new("calc.exe")
                .spawn()
                .unwrap();
        } else {
            println!("Failed to receive new hotkey message");
            exit(-1);
        }
    }
}
