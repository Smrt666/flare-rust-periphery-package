use hex_literal::hex;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;

use crate::flare;

pub const FLARE_CONTRACT_REGISTRY_ADDRESS: Address = Address {
    0: hex!("aD67FE66660Fb8dFE9d6b1b4240d8650e30F6019"),
};

pub async fn name_to_address(name: String, provider: &web3::Web3<Http>) -> Result<Address, String> {
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

pub async fn names_to_addresses(
    names: Vec<String>, // tuples, arrays?
    provider: &web3::Web3<Http>,
) -> Result<Vec<Address>, String> {
    let fcr_contract = Contract::from_json(
        provider.eth(),
        FLARE_CONTRACT_REGISTRY_ADDRESS,
        flare::products::FlareContractRegistry.abi,
    )
    .map_err(|e| e.to_string())?;
    fcr_contract
        .query(
            "getContractAddressesByName",
            names,
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
    pub async fn get_address(self, provider: &web3::Web3<Http>) -> Result<Address, String> {
        name_to_address(self.name.to_owned(), provider).await
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use web3::transports::Http;

    use super::*;

    // Tests only for coston2 - only syntax and logic tests
    #[tokio::test]
    async fn get_address() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let address: Address = name_to_address(String::from("FlareContractRegistry"), &web3s)
            .await
            .expect("Could not get address");
        assert_eq!(address, FLARE_CONTRACT_REGISTRY_ADDRESS);
    }

    #[tokio::test]
    async fn get_addresses() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let addresses: Vec<Address> = names_to_addresses(
            vec![
                String::from("FlareContractRegistry"),
                String::from("fbhsdlbf"),
            ],
            &web3s,
        )
        .await
        .expect("Could not get address");
        assert_eq!(addresses[0], FLARE_CONTRACT_REGISTRY_ADDRESS);
        assert_eq!(
            addresses[1],
            hex!("0000000000000000000000000000000000000000").into()
        );
        assert_eq!(addresses.len(), 2);
    }
}
