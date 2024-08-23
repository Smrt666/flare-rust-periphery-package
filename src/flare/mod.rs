mod abis;

/// Get abi for a contract by name. You probably want to use [products] instead.
pub fn name_to_abi(name: String) -> Option<&'static [u8]> {
    Some(abis::NAME_TO_ABI_MAP.get(&name)?)
}

/// Get abi for an interface by name. You probably want to use [interface_abis] instead.
pub fn interface_to_abi(name: String) -> Option<&'static [u8]> {
    Some(abis::INTERFACE_TO_ABI_MAP.get(&name)?)
}

/// The recommended way of accessing abis if you cannot use [products].
pub use abis::interface_abis;
/// The recommended way of accessing abis and addresses for all contracts.
///
/// Each of the avaiable constants is of type [crate::constants::ProductField].
/// Using them you can access contract name, it's interface name, abi and use it to
/// get contract address.
pub use abis::products;
