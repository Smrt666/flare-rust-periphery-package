use std::collections::HashMap;
use std::sync::LazyLock;

use crate::constants::ProductField;

pub struct InterfaceAbis {}

#[allow(non_upper_case_globals)]
impl InterfaceAbis {
    pub const IAddressBinder: &'static[u8] = "[{\"anonymous\": false, \"inputs\": [{\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"publicKey\", \"type\": \"bytes\"}, {\"indexed\": false, \"internalType\": \"bytes20\", \"name\": \"pAddress\", \"type\": \"bytes20\"}, {\"indexed\": false, \"internalType\": \"address\", \"name\": \"cAddress\", \"type\": \"address\"}], \"name\": \"AddressesRegistered\", \"type\": \"event\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"_cAddress\", \"type\": \"address\"}], \"name\": \"cAddressToPAddress\", \"outputs\": [{\"internalType\": \"bytes20\", \"name\": \"_pAddress\", \"type\": \"bytes20\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes20\", \"name\": \"_pAddress\", \"type\": \"bytes20\"}], \"name\": \"pAddressToCAddress\", \"outputs\": [{\"internalType\": \"address\", \"name\": \"_cAddress\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes\", \"name\": \"_publicKey\", \"type\": \"bytes\"}, {\"internalType\": \"bytes20\", \"name\": \"_pAddress\", \"type\": \"bytes20\"}, {\"internalType\": \"address\", \"name\": \"_cAddress\", \"type\": \"address\"}], \"name\": \"registerAddresses\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes\", \"name\": \"_publicKey\", \"type\": \"bytes\"}], \"name\": \"registerPublicKey\", \"outputs\": [{\"internalType\": \"bytes20\", \"name\": \"_pAddress\", \"type\": \"bytes20\"}, {\"internalType\": \"address\", \"name\": \"_cAddress\", \"type\": \"address\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}]".as_bytes();
}

pub struct Products {}

#[allow(non_upper_case_globals)]
impl Products {
    pub const AddressBinder: ProductField<'static> = ProductField {
        name: "AddressBinder",
        interface: "IAddressBinder",
        abi: InterfaceAbis::IAddressBinder,
    };
}

pub const NAME_TO_ABI_MAP: LazyLock<HashMap<String, &[u8]>> = LazyLock::new(|| {
    [(String::from("AddressBinder"), InterfaceAbis::IAddressBinder)]
        .iter()
        .cloned()
        .collect()
});

pub const INTERFACE_TO_ABI_MAP: LazyLock<HashMap<String, &[u8]>> = LazyLock::new(|| {
    [(
        String::from("IAddressBinder"),
        InterfaceAbis::IAddressBinder,
    )]
    .iter()
    .cloned()
    .collect()
});
