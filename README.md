# Cosmos Wallet Abstraction on Secret Network

---

## Installation


1 -------

Setup  local blockhain nodes + relayer (Hermes)
```
docker-compose up --build -d
```
Wait before hermes create a connection and a channel between two chains before running tests



2 -------

Compile contracts using
```
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
```
and move the final wasm into `artifacts` folder

You might want to use a docker container for optimizing or take a look into Makefile for:
```
make updest
``` 

3 -------

You can already run commands using
```npm test```
If you are missing the configuration files for IBC and deployed contract the test-suite will create them automatically

At the moment there is non-fixed racing condition issue due to which test sometimes crashes on account sequence errors. To avoid them you can run one test-suite only for the first time. E.g.
```npm test setup```

IBC tests might take a while due to waiting for acknowlegements


## Overview

The SDK is designed to abstract the complexities involved in interacting with the Secret Network for applications that deal with Cosmos wallets. It introduces a secure method for generating confidentials messages and reliable athenticating user at the same time using `chacha20poly1305_decrypt` algorithm

The SDK can be used be for developing major gateways that forward incoming messages forward to the network but it primives released here also make it quite straightforard to add built-in support of confidental messages directly in other contracts


## Extending Examples

This SDK extensively uses Rust template engines and trait to allow users to customise existing data structures and reuse them without need to fork the whole contract and define similar data-structures

For example
```Rust
pub enum GatewayExecuteMsg<E = Option<Empty>> 
    where E: JsonSchema
{
    ResetEncryptionKey  { },


    Extension {
        msg : E
    }


    /// encrypted variant of this enum except for this variant itself 
    Encrypted {
        payload             :   Binary,
        payload_signature   :   Binary,
        payload_hash        :   Binary,
        user_key            :   Binary,
        nonce               :   Binary,
    },

}
```
allows you to pass a custom type for `Extension` so that you don't need to re-define  your custom encrypted methods many times


```Rust
pub fn handle_encrypted_wrapper<E>(
    api     : &dyn Api,
    storage : &mut dyn Storage,
    info    : MessageInfo,
    msg     : E
) -> Result<(E, MessageInfo), StdError> 
    where E: WithEncryption + DeserializeOwned;
```
Take any Enum with a possible encrypted variant, attempts to decrypt it and then verify the authencity of the message and try to deserialize it into of the other inner variants of the message 


In practice you just need do a simple call like this: 
```Rust

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    let (
        msg, 
        info
    ) = sdk::common::handle_encrypted_wrapper(
        deps.api, deps.storage, info, msg
    )?;

    ...
```
and you get a decrypted version of `ExecuteMsg` with authenticated  `MessageInfo` which is set to user signing the message instead of a relayer or a dummy value set for IBC-hooks 


To be able to use the method you simply must implement `WithEncryption` trait for your message. It simply tells whether the message is encrypted and extract the essential parameters. 


Since it's already implemented for `GatewayExecuteMsg` when you extend it using template engine you don't need to do anything and get it out of the box


```Rust
impl<E> WithEncryption for crate::gateway::GatewayExecuteMsg<E> 
    where E: Clone + JsonSchema + Serialize
{
    fn encrypted(&self)     -> EncryptedParams {
        match self.clone() {
            crate::gateway::GatewayExecuteMsg::Encrypted {
                payload,
                payload_signature,
                payload_hash,
                user_key,
                nonce,
            } => EncryptedParams {
                payload,
                payload_signature,
                payload_hash,
                user_key,
                nonce
            },
            _ => panic!("This message is not encrypted")

        }
    }

    fn is_encrypted(&self)  -> bool {
        if let crate::gateway::GatewayExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}

```

In case there is need for complete customisation it's allways possible to take invidual components. They've beem designed to make as unrestrictive as possible. For example the following function accept a reference to
`cosmwasm_std::Binary`,
`std::vec::Vec`.
`[u8]`
and so on. 

```Rust
pub fn chacha20poly1305_decrypt(
    ciphertext    :     &impl Deref<Target = [u8]>,
    key           :     &impl Deref<Target = [u8]>,
    nonce         :     &impl Deref<Target = [u8]>,
) -> StdResult<Vec<u8>> {

    let ciper = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| StdError::generic_err(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);

    let plaintext = ciper.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| StdError::generic_err(e.to_string()))?;

    Ok(plaintext)
}
```



## Frontend

Documentation is Work in Progress. Refer to test files and `src` folder in the root of repository for usage examples