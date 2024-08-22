mod abis;

pub fn name_to_abi(name: String) -> Option<&'static [u8]> {
    Some(abis::NAME_TO_ABI_MAP.get(&name)?)
}

pub fn interface_to_abi(name: String) -> Option<&'static [u8]> {
    Some(abis::INTERFACE_TO_ABI_MAP.get(&name)?)
}

pub use abis::{InterfaceAbis, Products};
