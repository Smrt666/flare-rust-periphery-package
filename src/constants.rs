pub struct ProductField<'a> {
    // Contract
    pub name: &'a str,
    pub interface: &'a str,
    pub abi: &'a [u8],
}
