use flare_rust_periphery_package::{coston, interface_to_abi, name_to_abi, Chain::Coston};

#[test]
fn abis_exist_for_all_contracts() {
    let _ = coston::interface_abis::IFtso;
}

#[test]
fn interface_name_to_abi() {
    let _ = interface_to_abi(String::from("IFtso"), Coston).expect("Not found");
}

#[test]
fn abis_for_contracts() {
    let _ = coston::products::ClaimSetupManager;
}

#[test]
fn contract_name_to_abi() {
    let _ = name_to_abi(String::from("ClaimSetupManager"), Coston);
}

#[test]
fn toplevel_match() {
    assert_eq!(
        interface_to_abi(String::from("IClaimSetupManager"), Coston),
        name_to_abi(String::from("ClaimSetupManager"), Coston)
    );
}
