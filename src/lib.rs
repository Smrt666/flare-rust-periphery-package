mod constants;
use constants::UnsupportedChainError;

pub mod coston2;

pub fn name_to_abi(
    name: String,
    chain: &str,
) -> Result<Option<&'static [u8]>, UnsupportedChainError> {
    match chain {
        "coston2" => Ok(coston2::name_to_abi(name)),
        _ => Err(UnsupportedChainError),
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

        let contract =
            Contract::from_json(web3s.eth(), address, coston2::InterfaceAbis::IAddressBinder)
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
            Contract::from_json(web3s.eth(), address, coston2::Products::AddressBinder.abi)
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
        let name = coston2::Products::AddressBinder.name;
        assert_eq!(name, "AddressBinder");
        let interface = coston2::Products::AddressBinder.interface;
        assert_eq!(interface, "IAddressBinder");
    }
}
