#![windows_subsystem = "windows"]

use trayicon::{MenuBuilder, TrayIconBuilder};
use winapi::um::winuser;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};

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
    let event_loop = EventLoop::<Events>::with_user_event();
    let proxy = event_loop.create_proxy();

    let tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(include_bytes!("../icon.ico"))
        .tooltip("Tray Media Button")
        .on_click(Events::TogglePause)
        .menu(MenuBuilder::new().item("E&xit", Events::Exit))
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let _ = tray_icon;

        match event {
            Event::UserEvent(e) => match e {
                Events::Exit => *control_flow = ControlFlow::Exit,
                Events::TogglePause => {
                    send_key_event(0xB3, 0);
                }
            },
            _ => (),
        }
    });
}
