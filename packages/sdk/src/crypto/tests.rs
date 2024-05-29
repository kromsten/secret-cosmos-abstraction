use cosmwasm_std::{Binary, testing::mock_dependencies};
use super::*;

const SIGNER : &str = "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6";
const MSG : &str = "SGVsbG8sIHdvcmxk";
const SIGNATURE : &str = "x9jjSFv8/n1F8gOSRjddakYDbvroQm8ZoDWht/Imc1t5xUW49+Xaq7gwcsE+LCpqYoTBxnaXLg/xgJjYymCWvw==";
const PUBKEY : &str = "A08EGB7ro1ORuFhjOnZcSgwYlpe0DSFjVNUIkNNQxwKQ";
const HRP : &str = "cosmos";

#[test]
fn test_pubkey_to_account() {
    // correct
    assert_eq!(pubkey_to_account(
        Binary::from_base64(PUBKEY).unwrap().as_slice(),
        HRP
    ).unwrap(), SIGNER);

    // works but incorrect
    assert_ne!(pubkey_to_account(
        Binary::from_base64(PUBKEY).unwrap().as_slice(),
        "secret"
    ).unwrap(), SIGNER);

    assert_ne!(pubkey_to_account(
        b"some other key",
        HRP
    ).unwrap(), SIGNER);
}



#[test]
fn test_036_verification() {
    let deps = mock_dependencies();

    let auth = CosmosAuthData {
        signature: Binary::from_base64(SIGNATURE).unwrap(),
        message: Binary::from_base64(MSG).unwrap(),
        pubkey: Binary::from_base64(PUBKEY).unwrap(),
        hrp: Some(HRP.to_string())
    };

    assert!(verify_arbitrary(&deps.api, auth).is_ok());
}


