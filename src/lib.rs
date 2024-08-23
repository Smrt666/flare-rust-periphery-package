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
