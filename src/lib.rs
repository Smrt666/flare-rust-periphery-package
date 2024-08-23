pub mod constants;

pub mod coston;
pub mod coston2;
pub mod flare;
pub mod songbird;

pub use constants::{name_to_address, names_to_addresses};

pub enum Chain {
    Coston,
    Coston2,
    Flare,
    Songbird,
}

pub fn name_to_abi(name: String, chain: Chain) -> Option<&'static [u8]> {
    match chain {
        Chain::Coston2 => coston2::name_to_abi(name),
        Chain::Coston => coston::name_to_abi(name),
        Chain::Flare => flare::name_to_abi(name),
        Chain::Songbird => songbird::name_to_abi(name),
    }
}

pub fn interface_to_abi(name: String, chain: Chain) -> Option<&'static [u8]> {
    match chain {
        Chain::Coston2 => coston2::interface_to_abi(name),
        Chain::Coston => coston::interface_to_abi(name),
        Chain::Flare => flare::interface_to_abi(name),
        Chain::Songbird => songbird::interface_to_abi(name),
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use web3::contract::{Contract, Options};
    use web3::{ethabi::FixedBytes, transports::Http};

    use super::*;

    // Tests only for coston2 - only syntax and logic tests
    #[tokio::test]
    async fn get_abis_interface_dot() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let address = hex!("332aa9e37d64caaf70ae5dbe8efc2fc7611934ae").into();

        let contract = Contract::from_json(
            web3s.eth(),
            address,
            coston2::interface_abis::IAddressBinder,
        )
        .expect("Failed to create a contract instance.");
        let result = contract.query(
            "cAddressToPAddress",
            address,
            None,
            Options::default(),
            None,
        );
        let _query_result: FixedBytes = result.await.expect("Failed to retrieve results");
    }

    #[tokio::test]
    async fn get_abis_product_dot() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let address = hex!("332aa9e37d64caaf70ae5dbe8efc2fc7611934ae").into();

        let contract =
            Contract::from_json(web3s.eth(), address, coston2::products::AddressBinder.abi)
                .expect("Failed to create a contract instance.");
        let result = contract.query(
            "cAddressToPAddress",
            address,
            None,
            Options::default(),
            None,
        );
        let _query_result: FixedBytes = result.await.expect("Failed to retrieve results");
    }

    #[tokio::test]
    async fn get_abis_interface_name() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let address = hex!("332aa9e37d64caaf70ae5dbe8efc2fc7611934ae").into();

        let contract = Contract::from_json(
            web3s.eth(),
            address,
            coston2::interface_to_abi(String::from("IAddressBinder"))
                .expect("Could not find interface name"),
        )
        .expect("Failed to create a contract instance.");
        let result = contract.query(
            "cAddressToPAddress",
            address,
            None,
            Options::default(),
            None,
        );
        let _query_result: FixedBytes = result.await.expect("Failed to retrieve results");
    }

    #[tokio::test]
    async fn get_abis_contract_name() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);
        let address = hex!("332aa9e37d64caaf70ae5dbe8efc2fc7611934ae").into();

        let contract = Contract::from_json(
            web3s.eth(),
            address,
            coston2::name_to_abi(String::from("AddressBinder"))
                .expect("Could not find contract name"),
        )
        .expect("Failed to create a contract instance.");
        let result = contract.query(
            "cAddressToPAddress",
            address,
            None,
            Options::default(),
            None,
        );
        let _query_result: FixedBytes = result.await.expect("Failed to retrieve results");
    }

    #[test]
    fn get_product_properties() {
        let name = coston2::products::AddressBinder.name;
        assert_eq!(name, "AddressBinder");
        let interface = coston2::products::AddressBinder.interface;
        assert_eq!(interface, "IAddressBinder");
    }

    #[test]
    fn toplevel_name_to_abi() {
        name_to_abi(String::from("FlareContractRegistry"), Chain::Coston2).expect("Abi not found");
        name_to_abi(String::from("FlareContractRegistry"), Chain::Coston).expect("Abi not found");
        name_to_abi(String::from("FlareContractRegistry"), Chain::Flare).expect("Abi not found");
        name_to_abi(String::from("FlareContractRegistry"), Chain::Songbird).expect("Abi not found");
        match name_to_abi(String::from("kjdhsfjlhf"), Chain::Coston2) {
            Some(_) => panic!("Abi should not be found"),
            None => (),
        };
    }

    #[tokio::test]
    async fn test_get_address() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);

        // by products objects
        let addr = coston2::products::FlareContractRegistry
            .get_address(&web3s)
            .await
            .expect("Something went wrong");
        assert_eq!(addr, constants::FLARE_CONTRACT_REGISTRY_ADDRESS);
        let _ = coston2::products::FtsoFeedDecimals
            .get_address(&web3s)
            .await
            .expect("Something went wrong");

        // by name
        let addr = name_to_address(String::from("FlareContractRegistry"), &web3s)
            .await
            .expect("Something went wrong");
        assert_eq!(addr, constants::FLARE_CONTRACT_REGISTRY_ADDRESS);
    }

    #[tokio::test]
    async fn test_get_addresses() {
        let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
            .expect("Failed to create websocket.");
        let web3s = web3::Web3::new(websocket);

        let addrs = names_to_addresses(vec![String::from("FlareContractRegistry")], &web3s)
            .await
            .expect("Something went wrong");
        assert_eq!(addrs, vec![constants::FLARE_CONTRACT_REGISTRY_ADDRESS]);

        let addrs = names_to_addresses(
            vec![
                String::from("FlareContractRegistry"),
                String::from("sdkafnkljdfhl"),
            ],
            &web3s,
        )
        .await
        .expect("Something went wrong");
        assert_eq!(addrs.len(), 2);
    }
}
