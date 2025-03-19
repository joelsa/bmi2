pub struct Registers;
impl Registers {
    pub const CHIP_ID: u8 = 0x00;
    pub const ERR_REG: u8 = 0x02;
    pub const STATUS: u8 = 0x03;
    pub const AUX_DATA_0: u8 = 0x04;
    pub const ACC_DATA_0: u8 = 0x0C;
    pub const GYR_DATA_0: u8 = 0x12;
    pub const SENSORTIME_0: u8 = 0x18;
    pub const EVENT: u8 = 0x1B;
    pub const INT_STATUS_0: u8 = 0x1C;
    pub const INT_STATUS_1: u8 = 0x1D;
    pub const SC_OUT_0: u8 = 0x1E;
    pub const WR_GEST_ACT: u8 = 0x20;
    pub const INTERNAL_STATUS: u8 = 0x21;
    pub const TEMPERATURE_0: u8 = 0x22;
    pub const FIFO_LENGTH_0: u8 = 0x24;
    pub const FIFO_DATA: u8 = 0x26;
    pub const FEAT_PAGE: u8 = 0x2F;
    pub const FEATURES: u8 = 0x30;
    pub const ACC_CONF: u8 = 0x40;
    pub const ACC_RANGE: u8 = 0x41;
    pub const GYR_CONF: u8 = 0x42;
    pub const GYR_RANGE: u8 = 0x43;
    pub const AUX_CONF: u8 = 0x44;
    pub const FIFO_DOWNS: u8 = 0x45;
    pub const FIFO_WTM_0: u8 = 0x46;
    pub const FIFO_CONFIG_0: u8 = 0x48;
    pub const SATURATION: u8 = 0x4A;
    pub const AUX_DEV_ID: u8 = 0x4B;
    pub const AUX_IF_CONF: u8 = 0x4C;
    pub const AUX_RD_ADDR: u8 = 0x4D;
    pub const AUX_WR_ADDR: u8 = 0x4E;
    pub const AUX_WR_DATA: u8 = 0x4F;
    pub const ERR_REG_MSK: u8 = 0x52;
    pub const INT1_IO_CTRL: u8 = 0x53;
    pub const INT2_IO_CTRL: u8 = 0x54;
    pub const INT_LATCH: u8 = 0x55;
    pub const INT1_MAP_FEAT: u8 = 0x56;
    pub const INT2_MAP_FEAT: u8 = 0x57;
    pub const INT_MAP_DATA: u8 = 0x58;
    pub const INIT_CTRL: u8 = 0x59;
    pub const INIT_ADDR_0: u8 = 0x5B;
    pub const INIT_DATA: u8 = 0x5E;
    pub const INTERNAL_ERROR: u8 = 0x5F;
    pub const AUX_IF_TRIM: u8 = 0x68;
    pub const GYR_CRT_CONF: u8 = 0x69;
    pub const NVM_CONF: u8 = 0x6A;
    pub const IF_CONF: u8 = 0x6B;
    pub const DRV: u8 = 0x6C;
    pub const ACC_SELF_TEST: u8 = 0x6D;
    pub const GYR_SELF_TEST: u8 = 0x6E;
    pub const NV_CONF: u8 = 0x70;
    pub const OFFSET_0: u8 = 0x71;
    pub const OFFSET_3: u8 = 0x74;
    pub const PWR_CONF: u8 = 0x7C;
    pub const PWR_CTRL: u8 = 0x7D;
    pub const CMD: u8 = 0x7E;
}
