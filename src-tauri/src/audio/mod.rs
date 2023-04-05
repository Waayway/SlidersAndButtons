#![cfg(target_os="windows")]

use std::sync::{Mutex, Arc};
mod windows;

pub struct AudioState(pub Arc<Mutex<Audio>>);

pub struct Audio {
    interface: Box<dyn AudioInterface>
}

impl Audio {
    pub fn new() -> Self {
        let win_interface = windows::WinAudioInterface::new();
        Self {
            interface: Box::new(win_interface)
        }
    }
    pub fn get_all_sessions(&mut self) -> Vec<String> {
        // self.interface.refresh_all_sessions();
        self.interface.get_all_sessions()
    }
    pub fn set_volume(&mut self, vol: f32, session: String) {
        self.interface.set_volume(vol, session);
    }
    pub fn get_volume(&mut self, session: String) -> f32 {
        self.interface.get_volume(session)
    }
}


pub trait AudioInterface {
    fn new() -> Self where Self: Sized;
    fn get_all_sessions(&mut self) -> Vec<String>;
    fn set_volume(&mut self, vol: f32, session: String); // @param vol: f32, value between 0 and 1;
    fn get_volume(&mut self, session: String) -> f32;
    // fn refresh_all_sessions(&mut self);
}