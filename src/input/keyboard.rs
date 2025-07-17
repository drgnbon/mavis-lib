use windows::Win32::UI::Input::KeyboardAndMouse::{
INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY,
VK_RETURN,
};



/// Synthesizes Unicode text input by simulating keyboard events for each character.
///
/// # Arguments
/// * `text` - The Unicode string to be typed (UTF-8 encoded)
///
/// # Behavior
/// - Generates key press/release events for each character
/// - Uses Unicode input mode (`KEYEVENTF_UNICODE`)
/// - Processes characters sequentially with no delays
/// - Supports all Unicode characters representable in UTF-16
///
/// # Safety
/// This function contains unsafe WinAPI calls. The caller must ensure:
/// - The thread has permission to synthesize input
/// - No other thread is manipulating keyboard state concurrently
/// - The target window is prepared to receive Unicode input
///
/// # Windows Notes
/// - Uses `SendInput` with `KEYEVENTF_UNICODE` flag
/// - Each character requires two events (down/up)
/// - Virtual key code is set to 0 (unused in Unicode mode)
/// - Scan code contains the UTF-16 code unit
/// - System must be in Unicode input mode
///
/// # Limitations
/// - Does not support surrogate pairs (only BMP characters)
/// - No control over input timing/delays
/// - May be blocked by UIPI (User Interface Privilege Isolation)
pub fn type_unicode_text(text: &str) {
    let mut inputs = Vec::with_capacity(text.len() * 2);

    for c in text.chars() {
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        });

        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        });
    }

    unsafe {
        SendInput(inputs.as_slice(), std::mem::size_of::<INPUT>() as i32);
    }
}

/// Simulates pressing and releasing the ENTER key.
///
/// # Behavior
/// - Generates two keyboard events: key down followed by key up
/// - Uses virtual key code for ENTER (`VK_RETURN`)
/// - No delay between press and release
/// - Uses system default keyboard layout
///
/// # Safety
/// This function contains unsafe WinAPI calls. The caller must ensure:
/// - The thread has permission to synthesize input
/// - No other thread is manipulating keyboard state concurrently
/// - The target window is in focus and ready to receive input
pub fn press_enter() {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_RETURN,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_RETURN,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Simulates typing a single Unicode character using keyboard input events.
///
/// # Arguments
/// * `c` - The character to type (must be a BMP character, surrogate pairs not supported)
///
/// # Behavior
/// - Generates both key press and key release events
/// - Uses Unicode input mode (`KEYEVENTF_UNICODE`)
/// - Zero delay between press and release
/// - No virtual key code used (pure Unicode input)
///
/// # Safety
/// This function contains unsafe WinAPI calls. The caller must ensure:
/// - The thread has input simulation privileges
/// - No other thread is manipulating keyboard state concurrently
/// - The target application is prepared to receive Unicode input
pub fn type_unicode_char(c: char) {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

