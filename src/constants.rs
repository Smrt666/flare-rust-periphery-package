use std::fmt;

pub struct ProductField<'a> {
    // Contract
    pub name: &'a str,
    pub interface: &'a str,
    pub abi: &'a [u8],
}

#[derive(Debug, Clone)]
pub struct UnsupportedChainError;

impl fmt::Display for UnsupportedChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "supported chains are coston, coston2, songbird and flare"
        )
    }
}
