use crate::address::Address;

use super::{Nonce, LayerLimits};



pub struct TxHeader {
    pub principal: Address,
    pub template: Address,
    pub method: u8,
    pub nonce: Nonce,
    pub layer_limits: LayerLimits,
    pub max_gas: u64,
    pub gas_price: u64,
    pub max_spend: u64,
}

