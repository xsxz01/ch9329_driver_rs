use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use serialport::SerialPort;

use crate::{constants::{CmdCodeMap, CmdExecuteStatusMap}, frame_format::Frame};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KbmsResponse {
    pub frame: Frame,
}

#[derive(Debug, Default)]
pub struct KbmsMgr {
    connected: bool,
    serial: Option<Box<dyn SerialPort>>,
    pub address: u8,
    pub enable_debug: bool,
}

impl KbmsMgr {
    pub fn new(address: u8) -> KbmsMgr {
        KbmsMgr {
            address,
            connected: false,
            serial: None,
            enable_debug: false,
        }
    }

    pub fn connect(&mut self) -> bool {
        if self.connected {
            return false;
        }
        self.serial = Some(
            serialport::new("/dev/ttyUSB0", 115_200)
                .timeout(Duration::from_millis(20))
                .open()
                .expect("Failed to open serial port"),
        );
        true
    }

    pub fn connect_with_settings(&mut self, port_name: &str, baud_rate: u32) -> bool {
        if self.connected {
            return false;
        }
        self.serial = Some(
            serialport::new(port_name, baud_rate)
                .timeout(Duration::from_millis(20))
                .open()
                .expect("Failed to open serial port"),
        );
        true
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.serial = None;
    }

    fn send_frame(&mut self, cmd: CmdCodeMap, data: Option<Vec<u8>>) -> Vec<u8> {
        // build a command
        let frame = Frame::new(self.address, cmd, data.unwrap_or_default());
        let frame_bytes = frame.build();
        println!("{:?}", frame_bytes);
        self.serial
            .as_mut()
            .unwrap()
            .write(&frame_bytes)
            .expect("Failed to write to serial port");
        let mut buffer = [0; 64];
        // delay 20ms
        std::thread::sleep(Duration::from_millis(20));
        // read serial data
        let bytes_read = self
            .serial
            .as_mut()
            .unwrap()
            .read(&mut buffer)
            .expect("Failed to read from serial port");
        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
        // copy read data to a new vector
        let data = buffer[..bytes_read].to_vec();
        data
    }

    fn send_without_data(&mut self, cmd: CmdCodeMap) {
        let frame = Frame::new(self.address, cmd, Vec::new());
        let frame_bytes = frame.build();
        println!("{:?}", frame_bytes);
        self.serial
            .as_mut()
            .unwrap()
            .write(&frame_bytes)
            .expect("Failed to write to serial port");
        // delay 20ms
        std::thread::sleep(Duration::from_millis(20));
    }

    pub fn get_info(&mut self) -> Result<HashMap<&str, u8>, String> {
        let data = self.send_frame(CmdCodeMap::CmdGetInfo, None);
        if self.enable_debug {
            dbg!(&data);
        }
        let frame = Frame::parse(&data).unwrap();
        if frame.cmd_code == CmdCodeMap::RespGetInfo {
            // parse data
            // data length must be 8 bytes
            assert_eq!(data.len(), 8);
            let ic_ver = data.get(1).unwrap();
            let usb_status = data.get(2).unwrap();
            // keyboard status 3 bits in a byte, bit 0 to 2 is representing the keyboard status, num lock, caps lock, scroll lock, etc is useless.
            let keyboard_status = data.get(3).unwrap();
            let num_lock = keyboard_status & 0b00000001;
            let caps_lock = keyboard_status & 0b00000010;
            let scroll_lock = keyboard_status & 0b00000100;
            // bind status in a map
            let mut return_map: HashMap<&str, u8> = HashMap::new();
            return_map.insert("ic_ver", *ic_ver);
            return_map.insert("usb_status", *usb_status);
            return_map.insert("num_lock", num_lock);
            return_map.insert("caps_lock", caps_lock);
            return_map.insert("scroll_lock", scroll_lock);
            Ok(return_map)

        }else if frame.cmd_code == CmdCodeMap::ErrGetInfo {
            self.handle_err(data)
        }else {
            Err(format!("Invalid response command code: {:?}", frame.cmd_code))
        }
    }

    pub fn send_kb_general_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSendKbGeneralData, Some(data));
    }

    pub fn send_kb_media_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSendKbMediaData, Some(data));
    }

    pub fn send_ms_abs_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSendMsAbsData, Some(data));
    }

    pub fn send_ms_rel_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSendMsRelData, Some(data));
    }

    pub fn send_my_hid_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSendMyHidData, Some(data));
    }

    pub fn read_my_hid_data(&mut self) {
        self.send_without_data(CmdCodeMap::CmdReadMyHidData)
    }

    pub fn get_para_cfg_data(&mut self) -> Vec<u8> {
        self.send_frame(CmdCodeMap::CmdGetParaCfg, None)
    }

    pub fn set_para_cfg_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSetParaCfg, Some(data));
    }

    pub fn get_usb_string_data(&mut self) -> Vec<u8> {
        self.send_frame(CmdCodeMap::CmdGetUsbString, None)
    }

    pub fn set_usb_string_data(&mut self, data: Vec<u8>) {
        self.send_frame(CmdCodeMap::CmdSetUsbString, Some(data));
    }

    pub fn set_default_cfg(&mut self) {
        self.send_frame(CmdCodeMap::CmdSetDefaultCfg, None);
    }

    pub fn reset(&mut self) {
        self.send_frame(CmdCodeMap::CmdReset, None);
    }

    fn handle_err(&mut self, data: Vec<u8>) -> Result<HashMap<&str, u8>, String> {
        // handle error
        let err_code = data.get(0);
        match err_code {
            Some(code) => {
                if self.enable_debug {
                    eprint!("Error code: {}, message ->", code);
                }
                match CmdExecuteStatusMap::from(*code) {
                    CmdExecuteStatusMap::DefCmdSuccess => {
                        if self.enable_debug {
                            eprintln!("Command executed successfully")
                        }
                        Err(format!("Error code: {}, message ->Command executed successfully", code.clone()))
                    },
                    CmdExecuteStatusMap::DefCmdErrTimeout => {
                        if self.enable_debug {
                            eprintln!("Command execution timeout")
                        }
                        Err(format!("Error code: {}, message ->Command execution timeout", code))
                    },
                    CmdExecuteStatusMap::DefCmdErrHead => {
                        if self.enable_debug {
                            eprintln!("Command head error")
                        }
                        Err(format!("Error code: {}, message ->Command head error", code))
                    },
                    CmdExecuteStatusMap::DefCmdErrCmd => {
                        if self.enable_debug {
                            eprintln!("Command error")
                        }
                        Err(format!("Error code: {}, message ->Command error", code))
                    },
                    CmdExecuteStatusMap::DefCmdErrSum => {
                        if self.enable_debug {
                            eprintln!("Command sum error")
                        }
                        Err(format!("Error code: {}, message ->Command sum error", code))
                    },
                    CmdExecuteStatusMap::DefCmdErrPara => {
                        if self.enable_debug {
                            eprintln!("Command parameter error")
                        }
                        Err(format!("Error code: {}, message ->Command parameter error", code))
                    },
                    CmdExecuteStatusMap::DefCmdErrOperate => {
                        if self.enable_debug {
                            eprintln!("Command operation error")
                        }
                        Err(format!("Error code: {}, message ->Command operation error", code))
                    },
                }
            }
            None => {
                panic!("Error code not found")
            }
        }
    }
}
