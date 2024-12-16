use serde::{Deserialize, Serialize};

use crate::constants::CmdCodeMap;

// 帧头 2 字节 固定 0x57 0xAB
const FRAME_HEAD: [u8; 2] = [0x57, 0xAB];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    pub address: u8,
    pub cmd_code: CmdCodeMap,
    pub data: Vec<u8>,
}

impl Frame {
    // 构造函数
    pub fn new(address: u8, cmd_code: CmdCodeMap, data: Vec<u8>) -> Frame {
        Frame {
            address,
            cmd_code,
            data,
        }
    }

    // 构建帧
    pub fn build(&self) -> Vec<u8> {
        let mut frame_data: Vec<u8> = Vec::new();
        frame_data.extend_from_slice(&FRAME_HEAD);
        frame_data.push(self.address);
        frame_data.push(self.cmd_code.get_code());
        frame_data.push(self.data.len() as u8);
        frame_data.extend_from_slice(&self.data);
        frame_data
    }

    pub fn parse(data: &[u8]) -> Result<Frame, String> {
        if data.len() < 6 {
            return Err("Invalid frame length".to_string());
        }
        let address = data[2];
        let cmd_code = CmdCodeMap::from(data[3]).unwrap();
        let data_len = data[4] as usize;
        let data = data[5..(5 + data_len)].to_vec();
        Ok(Frame {
            address,
            cmd_code,
            data,
        })
    }
}
