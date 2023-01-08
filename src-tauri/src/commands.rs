use crate::serialization::Data;

#[tauri::command]
pub fn get_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut converted_ports: Vec<String> = vec![];
    for p in ports {
        converted_ports.push(p.port_name);
    }
    return converted_ports;
}

#[tauri::command]
async fn save_serial_port(serial_port: String) {
  let mut data = Data::load_data();
  data.serial_port = serial_port;
  data.save_data();
}
