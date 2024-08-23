//! Flare periphery package for rust.
//! It makes accessing artifact abis and their deployed addresses simpler
//! for official Flare contracts on chains Flare, Coston2, Songbird and Coston.
//!
//! # Examples
//! Get simple information about a contract:
//! ```
//! use flare_rust_periphery_package::coston2;
//!
//! fn main() {
//!     println!("Contract name: {}", coston2::products::FastUpdater.name);
//!     println!(
//!         "Contract name: {}",
//!         coston2::products::FastUpdater.interface
//!     );
//!     println!(
//!         "Contract abi: {}...",
//!         std::str::from_utf8(&coston2::products::FastUpdater.abi[0..50]).unwrap()
//!     );
//! }
//!
//! ```
//!
//! Get contract's address and create a web3 contract instance with a real address on coston2:
//! ```
//! use web3::contract::{Contract, Options};
//! use web3::transports::Http;
//! use web3::types::U256;
//!
//! use flare_rust_periphery_package::coston2;
//!
//! #[tokio::main]
//! async fn main() {
//!     let websocket = Http::new("https://coston2-api.flare.network/ext/C/rpc")
//!         .expect("Failed to create websocket.");
//!     let web3s = web3::Web3::new(websocket);
//!     let address = coston2::products::FastUpdater
//!         .get_address(&web3s)
//!         .await
//!         .expect("Could not get address.");
//!     println!("FastUpdater address: {}", address);
//!
//!     let contract = Contract::from_json(web3s.eth(), address, coston2::products::FastUpdater.abi)
//!         .expect("Failed to create a contract instance.");
//!     println!("Successfully built contract instance.");
//!
//!     let result: U256 = contract
//!         .query("currentRewardEpochId", (), None, Options::default(), None)
//!         .await
//!         .expect("Something went wrong");
//!     println!("Current reward epoch id: {}", result);
//! }
//! ```

/// Functionallity for an individual chain. They do not support exactly the same contracts
/// but are generally simillar.
pub mod coston;
pub mod coston2;
pub mod flare;
pub mod songbird;

/// Constants and types
pub mod constants;

use web3::types::Address;

/// Gets a contract address from FlareContractRegistry on chain.
///
/// For contracts supported by this package you should use `products.ContractName.get_address`
/// in appropriate chain.
pub async fn name_to_address(
    name: String,
    provider: &web3::Web3<web3::transports::Http>,
) -> Result<Address, String> {
    constants::name_to_address(name, provider).await
}

/// Gets contract addresses from FlareContractRegistry on chain. Queries only once for all contracts together.
pub async fn names_to_addresses(
    names: Vec<String>,
    provider: &web3::Web3<web3::transports::Http>,
) -> Result<Vec<Address>, String> {
    constants::names_to_addresses(names, provider).await
}

/// Used to specify chain when calling `name_to_abi` or `interface_to_abi`.
pub enum Chain {
    Coston,
    Coston2,
    Flare,
    Songbird,
}

/// Can be used to access abis by contract name. You probably want to use `products`
///  in [flare], [songbird], [coston] or [coston2] instead.
pub fn name_to_abi(name: String, chain: Chain) -> Option<&'static [u8]> {
    match chain {
        Chain::Coston2 => coston2::name_to_abi(name),
        Chain::Coston => coston::name_to_abi(name),
        Chain::Flare => flare::name_to_abi(name),
        Chain::Songbird => songbird::name_to_abi(name),
    }
}

/// Can be used to access abis by interface name. You probably want to use `products`
///  or `interface_abis` in [flare], [songbird], [coston] or [coston2] instead.
pub fn interface_to_abi(name: String, chain: Chain) -> Option<&'static [u8]> {
    match chain {
        Chain::Coston2 => coston2::interface_to_abi(name),
        Chain::Coston => coston::interface_to_abi(name),
        Chain::Flare => flare::interface_to_abi(name),
        Chain::Songbird => songbird::interface_to_abi(name),
    }
}
