use std::{
    io::{self, BufRead, BufReader, Read},
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    thread,
    time::Duration,
    vec,
};

use serialport::SerialPort;
use simulate::Key;

use crate::serialization::{Data, GridItemData};

pub struct BackgroundState(pub Mutex<BackgroundProcess>);

pub struct BackgroundProcess {
    sender: Option<Sender<Data>>,
}
impl BackgroundProcess {
    pub fn new() -> Self {
        Self { sender: None }
    }
    pub fn spawn_thread(&mut self) {
        let (tx, rx) = mpsc::channel::<Data>();
        std::thread::spawn(move || {
            let mut data = Data {
                serial_port: "".to_string(),
                grid_items: vec![],
            };
            let mut serial_connection: Option<SerialConnection> = None;
            loop {
                let recv_data = rx.try_recv();
                if recv_data.is_ok() {
                    let mut temp_data = recv_data.unwrap();
                    if !serialport::available_ports()
                        .unwrap()
                        .iter()
                        .any(|i| i.port_name == temp_data.serial_port)
                    {
                        temp_data.serial_port = "".to_string();
                        data = temp_data;
                    } else {
                        data = temp_data;
                        if let Some(con) = serial_connection {
                            con.close_connection();
                        }
                        serial_connection =
                            Some(SerialConnection::open_connection(data.serial_port.clone()));
                    }
                }
                if data.serial_port.is_empty() {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                if let Some(x) = &mut serial_connection {
                    x.run_loop(data.clone());
                }
            }
        });
        self.sender = Some(tx);
    }
    pub fn update_config(&self, data: Data) {
        if let Some(tx) = &self.sender {
            tx.send(data).ok();
        }
    }
}

struct SerialConnection {
    port: Box<dyn SerialPort>,
}

impl SerialConnection {
    fn open_connection(serial_port: String) -> Self {
        let port = serialport::new(serial_port, 9600)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        Self { port: port }
    }

    fn close_connection(&self) {
        drop(&self.port);
    }
    fn run_loop(&mut self, config: Data) {
        // * Getting data from arduino
        let mut total_buf: String = String::new();
        let mut buffer: [u8; 1] = [0; 1];
        let mut should_read = true;
        while should_read {
            match self.port.read(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 1 {
                        should_read = buffer[0] != 0x0D;
                        if should_read && buffer[0] != 0x0A {
                            total_buf += &String::from_utf8(buffer.clone().to_vec())
                                .ok()
                                .unwrap_or("".to_string());
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        // * Parsing data from the arduino
        let collection = total_buf.split("|");
        for i in collection {
            let mut info = i.trim();
            if info.starts_with("Button #") {
                let data: i32 = info.replace("Button #", "").parse().unwrap_or(-1);
                if data != -1 {
                    let gridItemConfigs: Vec<GridItemData> =
                        config.grid_items.iter().map(|i| i.data.clone()).collect();

                    let result = gridItemConfigs
                        .iter()
                        .find(|&i| i.key.clone().unwrap_or("".to_string()) == data.to_string());

                    if result.is_some() {
                        let result = result.unwrap();
                        if result.keyCombo.is_some() {
                            println!("{:?}", result.keyCombo);
                            let keyCombo = result.keyCombo.as_ref().unwrap();
                            if keyCombo.modifiers.ctrl {
                                simulate::press(Key::Control).ok();
                            }
                            if keyCombo.modifiers.alt {
                                simulate::press(Key::Menu).ok();
                            }
                            if keyCombo.modifiers.shift {
                                simulate::press(Key::Shift).ok();
                            }
                            if keyCombo.modifiers.meta {
                                simulate::press(Key::LeftWindows).ok();
                            }
                            simulate::send(convertFromStringToKey(&keyCombo.key.as_ref().unwrap()))
                                .ok();
                            thread::sleep(Duration::from_millis(40));
                            if keyCombo.modifiers.ctrl {
                                simulate::release(Key::Control).ok();
                            }
                            if keyCombo.modifiers.alt {
                                simulate::release(Key::Menu).ok();
                            }
                            if keyCombo.modifiers.shift {
                                simulate::release(Key::Shift).ok();
                            }
                            if keyCombo.modifiers.meta {
                                simulate::release(Key::LeftWindows).ok();
                            }
                        }
                    }
                }
            } else if info.starts_with("Slider #") {
                let data = info.replace("Slider #", "");
                // println!("{}", data);
            }
        }
    }
}

fn convertFromStringToKey(key: &str) -> Key {
    match key.to_lowercase().as_str() {
        "escape" => Key::Escape,
        "mediaplaypause" => Key::MediaPlayPause,
        "mediaprevioustrack" => Key::MediaPreviousTrack,
        "mediastop" => Key::MediaStop,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "f13" => Key::F13,
        "f14" => Key::F14,
        "f15" => Key::F15,
        "f16" => Key::F16,
        "f17" => Key::F17,
        "f18" => Key::F18,
        "f19" => Key::F19,
        "f20" => Key::F20,
        "f21" => Key::F21,
        "f22" => Key::F22,
        "f23" => Key::F23,
        "f24" => Key::F24,
        "a" => Key::A,
        "b" => Key::B,
        "c" => Key::C,
        "d" => Key::D,
        "e" => Key::E,
        "f" => Key::F,
        "g" => Key::G,
        "h" => Key::H,
        "i" => Key::I,
        "j" => Key::J,
        "k" => Key::K,
        "l" => Key::L,
        "m" => Key::M,
        "n" => Key::N,
        "o" => Key::O,
        "p" => Key::P,
        "q" => Key::Q,
        "r" => Key::R,
        "s" => Key::S,
        "t" => Key::T,
        "u" => Key::U,
        "v" => Key::V,
        "w" => Key::W,
        "x" => Key::X,
        "y" => Key::Y,
        "z" => Key::Z,
        _ => Key::A,
    }
}
