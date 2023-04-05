use windows_volume_control::AudioController;

use super::AudioInterface;

pub struct WinAudioInterface {
    controller: AudioController,
}

impl AudioInterface for WinAudioInterface {
    fn new() -> Self {
        unsafe {
            let mut controller = AudioController::init(Some(windows_volume_control::CoinitMode::ApartmentThreaded));
            controller.GetSessions();
            controller.GetDefaultAudioEnpointVolumeControl();
            controller.GetAllProcessSessions();
            Self { 
                controller: controller
            }
        }
    }
    fn get_all_sessions(&mut self) -> Vec<String> {
        unsafe {
            self.controller.get_all_session_names()
        }
    }
    fn get_volume(&mut self, session: String) -> f32 {
        unsafe {
            if let Some(session) = self.controller.get_session_by_name(session) {
                session.getVolume()
            } else {
                0.0
            }
        }
    }
    fn set_volume(&mut self, vol: f32, session: String) {
        unsafe {
            if let Some(session) = self.controller.get_session_by_name(session) {
                session.setVolume(vol);
            }
        }
    }
}
