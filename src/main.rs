#![windows_subsystem = "windows"]

use std::mem::MaybeUninit;
use trayicon::{MenuBuilder, TrayIconBuilder};
use winapi::um::winuser;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Events {
    Exit,
    TogglePause,
}

fn send_key_event(vk: u8, flags: u32) {
    unsafe {
        winuser::keybd_event(vk, 0, flags, 0);
        winuser::keybd_event(vk, 0, flags | winuser::KEYEVENTF_KEYUP, 0);
    }
}

fn main() {
    let (s, r) = std::sync::mpsc::channel::<Events>();

    TrayIconBuilder::new()
        .sender(s)
        .icon_from_buffer(include_bytes!("../icon.ico"))
        .tooltip("Tray Media Button")
        .on_click(Events::TogglePause)
        .menu(MenuBuilder::new().item("E&xit", Events::Exit))
        .build()
        .unwrap();

    std::thread::spawn(move || {
        r.iter().for_each(|m| match m {
            Events::TogglePause => {
                send_key_event(0xB3, 0);
            }
            Events::Exit => {
                std::process::exit(0x0000);
            }
        })
    });

    loop {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0);
            if bret > 0 {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                break;
            }
        }
    }
}
