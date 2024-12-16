use serde::{Deserialize, Serialize};

/// 枚举类型CmdCodeMap用于定义各种命令的代码映射
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum CmdCodeMap {
    // 获取芯片版本等信息
    CmdGetInfo = 0x01,
    // 响应获取芯片版本信息
    RespGetInfo = 0x01 | 0x80,
    // 错误获取芯片版本信息
    ErrGetInfo = 0x01 | 0xC0,

    // 发送USB键盘普通数据
    CmdSendKbGeneralData = 0x02,
    // 响应发送USB键盘普通数据
    RespSendKbGeneralData = 0x02 | 0x80,
    // 错误发送USB键盘普通数据
    ErrSendKbGeneralData = 0x02 | 0xC0,

    // 发送USB键盘多媒体数据
    CmdSendKbMediaData = 0x03,
    // 响应发送USB键盘多媒体数据
    RespSendKbMediaData = 0x03 | 0x80,
    // 错误发送USB键盘多媒体数据
    ErrSendKbMediaData = 0x03 | 0xC0,

    // 发送绝对鼠标数据
    CmdSendMsAbsData = 0x04,
    // 响应发送绝对鼠标数据
    RespSendMsAbsData = 0x04 | 0x80,
    // 错误发送绝对鼠标数据
    ErrSendMsAbsData = 0x04 | 0xC0,

    // 发送相对鼠标数据
    CmdSendMsRelData = 0x05,
    // 响应发送相对鼠标数据
    RespSendMsRelData = 0x05 | 0x80,
    // 错误发送相对鼠标数据
    ErrSendMsRelData = 0x05 | 0xC0,

    // 发送自定义HID数据
    CmdSendMyHidData = 0x06,
    // 响应发送自定义HID数据
    RespSendMyHidData = 0x06 | 0x80,
    // 错误发送自定义HID数据
    ErrSendMyHidData = 0x06 | 0xC0,

    // 读取自定义HID数据
    CmdReadMyHidData = 0x87,

    // 获取参数配置
    CmdGetParaCfg = 0x08,
    // 响应获取参数配置
    RespGetParaCfg = 0x08 | 0x80,
    // 错误获取参数配置
    ErrGetParaCfg = 0x08 | 0xC0,

    // 设置参数配置
    CmdSetParaCfg = 0x09,
    // 响应设置参数配置
    RespSetParaCfg = 0x09 | 0x80,
    // 错误设置参数配置
    ErrSetParaCfg = 0x09 | 0xC0,

    // 获取USB字符串
    CmdGetUsbString = 0x0A,
    // 响应获取USB字符串
    RespGetUsbString = 0x0A | 0x80,
    // 错误获取USB字符串
    ErrGetUsbString = 0x0A | 0xC0,

    // 设置USB字符串
    CmdSetUsbString = 0x0B,
    // 响应设置USB字符串
    RespSetUsbString = 0x0B | 0x80,
    // 错误设置USB字符串
    ErrSetUsbString = 0x0B | 0xC0,

    // 恢复出厂默认设置
    CmdSetDefaultCfg = 0x0C,
    // 响应恢复出厂默认设置
    RespSetDefaultCfg = 0x0C | 0x80,
    // 错误恢复出厂默认设置
    ErrSetDefaultCfg = 0x0C | 0xC0,

    // 复位芯片
    CmdReset = 0x0F,
    // 响应复位芯片
    RespReset = 0x0F | 0x80,
    // 错误复位芯片
    ErrReset = 0x0F | 0xC0,
}

impl CmdCodeMap {
    pub fn get_code(&self) -> u8 {
        match self {
            CmdCodeMap::CmdGetInfo => 0x01,
            CmdCodeMap::CmdSendKbGeneralData => 0x02,
            CmdCodeMap::CmdSendKbMediaData => 0x03,
            CmdCodeMap::CmdSendMsAbsData => 0x04,
            CmdCodeMap::CmdSendMsRelData => 0x05,
            CmdCodeMap::CmdSendMyHidData => 0x06,
            CmdCodeMap::CmdReadMyHidData => 0x87,
            CmdCodeMap::CmdGetParaCfg => 0x08,
            CmdCodeMap::CmdSetParaCfg => 0x09,
            CmdCodeMap::CmdGetUsbString => 0x0A,
            CmdCodeMap::CmdSetUsbString => 0x0B,
            CmdCodeMap::CmdSetDefaultCfg => 0x0C,
            CmdCodeMap::CmdReset => 0x0F,

            CmdCodeMap::RespGetInfo => 0x01 | 0x80,
            CmdCodeMap::RespSendKbGeneralData => 0x02 | 0x80,
            CmdCodeMap::RespSendKbMediaData => 0x03 | 0x80,
            CmdCodeMap::RespSendMsAbsData => 0x04 | 0x80,
            CmdCodeMap::RespSendMsRelData => 0x05 | 0x80,
            CmdCodeMap::RespSendMyHidData => 0x06 | 0x80,
            CmdCodeMap::RespGetParaCfg => 0x08 | 0x80,
            CmdCodeMap::RespSetParaCfg => 0x09 | 0x80,
            CmdCodeMap::RespGetUsbString => 0x0A | 0x80,
            CmdCodeMap::RespSetUsbString => 0x0B | 0x80,
            CmdCodeMap::RespSetDefaultCfg => 0x0C | 0x80,
            CmdCodeMap::RespReset => 0x0F | 0x80,

            CmdCodeMap::ErrGetInfo => 0x01 | 0xC0,
            CmdCodeMap::ErrSendKbGeneralData => 0x02 | 0xC0,
            CmdCodeMap::ErrSendKbMediaData => 0x03 | 0xC0,
            CmdCodeMap::ErrSendMsAbsData => 0x04 | 0xC0,
            CmdCodeMap::ErrSendMsRelData => 0x05 | 0xC0,
            CmdCodeMap::ErrSendMyHidData => 0x06 | 0xC0,
            CmdCodeMap::ErrGetParaCfg => 0x08 | 0xC0,
            CmdCodeMap::ErrSetParaCfg => 0x09 | 0xC0,
            CmdCodeMap::ErrGetUsbString => 0x0A | 0xC0,
            CmdCodeMap::ErrSetUsbString => 0x0B | 0xC0,
            CmdCodeMap::ErrSetDefaultCfg => 0x0C | 0xC0,
            CmdCodeMap::ErrReset => 0x0F | 0xC0,
        }
    }

    pub fn from(code: u8) -> Result<CmdCodeMap, String> {
        match code {
            0x01 => Ok(CmdCodeMap::CmdGetInfo),
            0x81 => Ok(CmdCodeMap::RespGetInfo),
            0xC1 => Ok(CmdCodeMap::ErrGetInfo),

            0x02 => Ok(CmdCodeMap::CmdSendKbGeneralData),
            0x82 => Ok(CmdCodeMap::RespSendKbGeneralData),
            0xC2 => Ok(CmdCodeMap::ErrSendKbGeneralData),

            0x03 => Ok(CmdCodeMap::CmdSendKbMediaData),
            0x83 => Ok(CmdCodeMap::RespSendKbMediaData),
            0xC3 => Ok(CmdCodeMap::ErrSendKbMediaData),

            0x04 => Ok(CmdCodeMap::CmdSendMsAbsData),
            0x84 => Ok(CmdCodeMap::RespSendMsAbsData),
            0xC4 => Ok(CmdCodeMap::ErrSendMsAbsData),

            0x05 => Ok(CmdCodeMap::CmdSendMsRelData),
            0x85 => Ok(CmdCodeMap::RespSendMsRelData),
            0xC5 => Ok(CmdCodeMap::ErrSendMsRelData),

            0x06 => Ok(CmdCodeMap::CmdSendMyHidData),
            0x86 => Ok(CmdCodeMap::RespSendMyHidData),
            0xC6 => Ok(CmdCodeMap::ErrSendMyHidData),

            0x87 => Ok(CmdCodeMap::CmdReadMyHidData),

            0x08 => Ok(CmdCodeMap::CmdGetParaCfg),
            0x88 => Ok(CmdCodeMap::RespGetParaCfg),
            0xC8 => Ok(CmdCodeMap::ErrGetParaCfg),

            0x09 => Ok(CmdCodeMap::CmdSetParaCfg),
            0x89 => Ok(CmdCodeMap::RespSetParaCfg),
            0xC9 => Ok(CmdCodeMap::ErrSetParaCfg),

            0x0A => Ok(CmdCodeMap::CmdGetUsbString),
            0x8A => Ok(CmdCodeMap::RespGetUsbString),
            0xCA => Ok(CmdCodeMap::ErrGetUsbString),

            0x0B => Ok(CmdCodeMap::CmdSetUsbString),
            0x8B => Ok(CmdCodeMap::RespSetUsbString),
            0xCB => Ok(CmdCodeMap::ErrSetUsbString),

            0x0C => Ok(CmdCodeMap::CmdSetDefaultCfg),
            0x8C => Ok(CmdCodeMap::RespSetDefaultCfg),
            0xCC => Ok(CmdCodeMap::ErrSetDefaultCfg),

            0x0F => Ok(CmdCodeMap::RespReset),
            0x8F => Ok(CmdCodeMap::CmdReset),
            0xCF => Ok(CmdCodeMap::ErrReset),
            _ => Err(String::from("Invalid command code")),
        }
    }
}
/// 枚举类型CmdExecuteStatusMap用于定义命令执行状态的代码映射
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CmdExecuteStatusMap {
    // 命令执行成功
    DefCmdSuccess = 0x00,
    // 串口接受一个字节超时
    DefCmdErrTimeout = 0xE1,
    // 串口接收包头字节出错
    DefCmdErrHead = 0xE2,
    // 串口接受命令码错误
    DefCmdErrCmd = 0xE3,
    // 累加和检验值不通过
    DefCmdErrSum = 0xE4,
    // 参数错误
    DefCmdErrPara = 0xE5,
    // 帧正常，执行命令执行失败
    DefCmdErrOperate = 0xE6,
}

impl CmdExecuteStatusMap {
    pub fn get_code(&self) -> u8 {
        match self {
            CmdExecuteStatusMap::DefCmdSuccess => 0x00,
            CmdExecuteStatusMap::DefCmdErrTimeout => 0xE1,
            CmdExecuteStatusMap::DefCmdErrHead => 0xE2,
            CmdExecuteStatusMap::DefCmdErrCmd => 0xE3,
            CmdExecuteStatusMap::DefCmdErrSum => 0xE4,
            CmdExecuteStatusMap::DefCmdErrPara => 0xE5,
            CmdExecuteStatusMap::DefCmdErrOperate => 0xE6,
        }
    }
}

impl From<u8> for CmdExecuteStatusMap {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CmdExecuteStatusMap::DefCmdSuccess,
            0xE1 => CmdExecuteStatusMap::DefCmdErrTimeout,
            0xE2 => CmdExecuteStatusMap::DefCmdErrHead,
            0xE3 => CmdExecuteStatusMap::DefCmdErrCmd,
            0xE4 => CmdExecuteStatusMap::DefCmdErrSum,
            0xE5 => CmdExecuteStatusMap::DefCmdErrPara,
            0xE6 => CmdExecuteStatusMap::DefCmdErrOperate,
            _ => panic!("Invalid value"),
        }
    }
}
