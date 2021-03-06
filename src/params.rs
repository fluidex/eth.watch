pub const ADDRESS_LEN: usize = 20;

/// Priority op should be executed for this number of eth blocks.
pub const PRIORITY_EXPIRATION: u64 = 35000;

// TODO: check these
pub const ACCOUNT_ID_BIT_WIDTH: usize = 16;
pub const TOKEN_BIT_WIDTH: usize = 16;
pub const TX_TYPE_BIT_WIDTH: usize = 8;
pub const BALANCE_BIT_WIDTH: usize = 256; // TODO: need to be consistent with .sol. zkSync use uint128.

// Fr element encoding
pub const FR_BIT_WIDTH: usize = 254;

pub const FR_ADDRESS_LEN: usize = 20;
pub const BJJ_ADDRESS_LEN: usize = 32;
