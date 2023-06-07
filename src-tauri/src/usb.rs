use std::{time::Duration, io::{BufReader, BufRead, Write, ErrorKind}};

use serialport::{available_ports, SerialPortType::UsbPort};

const BAUD_RATE: u32 = 9600;
const VID: u16 = 0x2341;
const PID: u16 = 0x0042;

pub fn get_graph() -> Vec<String> {
    let mut s_port = None;
    for port in available_ports().unwrap() {
        match port.port_type {
            UsbPort(p_info) => {
                if p_info.vid == VID && p_info.pid == PID {
                    s_port = Some(port.port_name);
                }
            },
            _ => {},
        }
    }

    let mut serial_port = serialport::new(s_port.expect("Arduino not found"), BAUD_RATE)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open serial port");

    let mut reader = BufReader::new(serial_port.try_clone().unwrap());
    let mut ret: Vec<String> = Vec::new();

    serial_port.write(&[50]).expect("Write Failed");

    loop {
        let mut my_str = String::new();
        match reader.read_line(&mut my_str) {
            Ok(_) => { ret.push(my_str); },
            Err(e) => {
                if e.kind() == ErrorKind::TimedOut {
                    break;
                }
            },
        }
    }

    ret
    
}
