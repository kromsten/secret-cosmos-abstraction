use cosmwasm_std::{Binary, testing::mock_dependencies};
use super::{*, utils::{get_common_key, secret_key_from_bytes, public_key_from_bytes}};

const SIGNER : &str = "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6";
const SIGNED_MSG : &str = "SGVsbG8sIHdvcmxk";
const SIGNATURE : &str = "x9jjSFv8/n1F8gOSRjddakYDbvroQm8ZoDWht/Imc1t5xUW49+Xaq7gwcsE+LCpqYoTBxnaXLg/xgJjYymCWvw==";
const SIGNING_PUBKEY : &str = "A08EGB7ro1ORuFhjOnZcSgwYlpe0DSFjVNUIkNNQxwKQ";
const SIGN_HRP : &str = "cosmos";



#[test]
fn test_pubkey_to_account() {
    // correct
    assert_eq!(pubkey_to_address(
        Binary::from_base64(SIGNING_PUBKEY).unwrap().as_slice(),
        SIGN_HRP
    ).unwrap(), SIGNER);

    // works but incorrect
    assert_ne!(pubkey_to_address(
        Binary::from_base64(SIGNING_PUBKEY).unwrap().as_slice(),
        "secret"
    ).unwrap(), SIGNER);

    assert_ne!(pubkey_to_address(
        b"some other key",
        SIGN_HRP
    ).unwrap(), SIGNER);
}




#[test]
fn test_036_verification() {
    let deps = mock_dependencies();

    // all correct
    let cred = CosmosCredential {
        signature: Binary::from_base64(SIGNATURE).unwrap(),
        message: SIGNED_MSG,
        pubkey: Binary::from_base64(SIGNING_PUBKEY).unwrap(),
        hrp: Some(SIGN_HRP.to_string())
    };

    assert!(verify_arbitrary(&deps.api, &cred).is_ok());

    // wrong signature
    let cred2 = CosmosCredential {
        signature: Binary::from_base64("d3Jvbmc=").unwrap(),
        message: Binary::from_base64(SIGNED_MSG).unwrap(),
        pubkey: Binary::from_base64(SIGNING_PUBKEY).unwrap(),
        hrp: Some(SIGN_HRP.to_string())
    };
    assert!(verify_arbitrary(&deps.api, &cred2).is_err());


    // wrong message
    let cred3 = CosmosCredential {
        signature: Binary::from_base64(SIGNATURE).unwrap(),
        message: Binary::from_base64("d3Jvbmc=").unwrap(),
        pubkey: Binary::from_base64(SIGNING_PUBKEY).unwrap(),
        hrp: Some(SIGN_HRP.to_string())
    };
    assert!(verify_arbitrary(&deps.api, &cred3).is_err());

    // wrong pubkey
    let cred4 = CosmosCredential {
        signature: Binary::from_base64(SIGNATURE).unwrap(),
        message: Binary::from_base64(SIGNED_MSG).unwrap(),
        pubkey: Binary::from_base64("d3Jvbmc=").unwrap(),
        hrp: Some(SIGN_HRP.to_string())
    };
    assert!(verify_arbitrary(&deps.api, &cred4).is_err());

    // different hrp
    let cred5 = CosmosCredential {
        signature: Binary::from_base64(SIGNATURE).unwrap(),
        message: Binary::from_base64(SIGNED_MSG).unwrap(),
        pubkey: Binary::from_base64(SIGNING_PUBKEY).unwrap(),
        hrp: Some("secret".to_string())
    };
    assert!(verify_arbitrary(&deps.api, &cred5).is_err());
}



const CONTRACT_PRIVATE : &str = "OIxm5RnQkzCDMMoXv9LIkmVhrr2+AfFfneOsmXfSwQ0=";
const _CONTRACT_PUBLIC  : &str = "A5OJoIXQRceFbnqUe2rJ3s2MKAFEiPrKZ86eHPyZvSl5";
const CLIENT_PUBLIC    : &str = "AgGQoJ1UiOfUW1PKCAnoYS+JM9efvuIUTjjmFO7/Y+MZ";

const COMMON_KEY       : &str = "vkgtL7d53z12+Ies8iKnhG2HkvEBMCLmrQFwoLqccOo=";

const PLAINTEXT        : &str = "eyJ0ZXN0Ijp7fX0="; // {"test":{}}
const CIPHERTEXT       : &str = "b1TCHbfU3ndpEhRbYxVQNdMlJS5iRZ/QQW9O";
const NONCE            : &str = "8hxrJPTMRgQu45qL";


#[test]
fn can_derive_shared_key() {

    let key = get_common_key(
        public_key_from_bytes(
            &Binary::from_base64(CLIENT_PUBLIC).unwrap()
        ).unwrap(),
        secret_key_from_bytes(
            &Binary::from_base64(CONTRACT_PRIVATE).unwrap()
        ).unwrap()        
    );
    assert_eq!(key, Binary::from_base64(COMMON_KEY).unwrap());
}


#[test]
fn can_decrypt() {
    let key = get_common_key(
        public_key_from_bytes(
            &Binary::from_base64(CLIENT_PUBLIC).unwrap()
        ).unwrap(),
        secret_key_from_bytes(
            &Binary::from_base64(CONTRACT_PRIVATE).unwrap()
        ).unwrap()        
    );

    let decrypted = chacha20poly1305_decrypt(
        &Binary::from_base64(CIPHERTEXT).unwrap(),
        &key,
        &Binary::from_base64(NONCE).unwrap(),
    ).unwrap();

    assert_eq!(Binary(decrypted).to_base64(), PLAINTEXT);
}