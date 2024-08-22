use hex_literal::hex;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;

use crate::flare;

pub const FLARE_CONTRACT_REGISTRY_ADDRESS: Address = Address {
    0: hex!("aD67FE66660Fb8dFE9d6b1b4240d8650e30F6019"),
};

pub async fn get_address(name: String, provider: web3::Web3<Http>) -> Result<Address, String> {
    let fcr_contract = Contract::from_json(
        provider.eth(),
        FLARE_CONTRACT_REGISTRY_ADDRESS,
        flare::products::FlareContractRegistry.abi,
    )
    .map_err(|e| e.to_string())?;
    fcr_contract
        .query(
            "getContractAddressByName",
            name,
            None,
            Options::default(),
            None,
        )
        .await
        .map_err(|e| e.to_string())
}

pub struct ProductField<'a> {
    // Contract
    pub name: &'a str,
    pub interface: &'a str,
    pub abi: &'a [u8],
}

impl<'a> ProductField<'a> {
    pub async fn get_address(self, provider: web3::Web3<Http>) -> Result<Address, String> {
        get_address(self.name.to_owned(), provider).await
    }
}
