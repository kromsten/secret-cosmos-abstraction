# Cosmos Wallet Abstraction on Secret Network

---

## Introduction

This document provides a detailed look into SDK, which facilitates the creation and management of proxy accounts to operate on Secret Network with remote Cosmos addresses. It includes utilities for account management, transaction signing, and integration with custom contracts

## Overview

The SDK is designed to abstract the complexities involved in interacting with the Secret Network for applications that deal with Cosmos wallets. It introduces a secure method for authenticating users and managing session keys, allowing decentralized applications (dApps) to authenticate users via signatures using the arbitrary signatures [036](https://docs.cosmos.network/main/build/architecture/adr-036-arbitrary-signature) specification

Session keys, once generated, can be used to bypass the need to generate a new signature and do its cryptographic verification every single time. Session keys can follow similar format of viewing keys and can be produced using existing utility functions from the secret-toolkit. It's recommend to allow session keys to be used as viewing keys for queries, as it simplifies the process of managing keys and permissions. Regular viewing keys created from signed permit can be also should also be supported for compatibility with existing ecosystem.

For security and modularity, it's recommended to separate authentication logic into its own contract, using it as a proxy for user actions. This not only enhances security but also simplifies contract design by decoupling authentication from business logic.

The SDK includes message for interacting with an example proxy account contract and with a factory contract used for creating new accounts


## Example Flow


### 1. Generating a Signature

To proof ownership of a Cosmos address, the user must sign a message using their private key. The message should be formatted according to the arbitrary signature [036](https://docs.cosmos.network/main/build/architecture/adr-036-arbitrary-signature) specification.

The content of the message itself can be arbitrary, but it is good idea to include a timestamps and chain-id to prevent replay attacks. 
A big message or data structure should cab hashed using SHA256 before signing. E.g.

```typescript
type DataToSign = {
    msgs      :  CosmosMsg[],
    chainId   :  string,
    timestamp :  number,
}


const dataToSign = /* Populated Data */

const message = sha256(JSON.stringify(dataToSign))
```


Example of generating an arbitary signature [with Keplr](https://docs.keplr.app/api/#request-signature-for-arbitrary-message):

```typescript
const signature = await signArbitrary(chainId, signer, message)
```

To verify the signature on contract side we must `dataToSign`, `signature` and `pubkey` of the signer. Additionally to generate the correct prefix for the address we ether must pass `hrp` of the chain or the `signer` address itself as additional parameter.


### 2. Authenticating / Creating Proxy Account

Using the signature and the rest of the authenticating primitives, the user can authenticate themselves on a contract of a dapp or to create a separate proxy account contract only controlled by the provided credentials.

To avoid generating a new signature every time, the contract can generate a session key that can be used to authenticate the user in future interactions and also to view the private state of the contract. A contract can return the newly generated session key in encrypted logs (ether with help of VM or using public key of the user) 

Calling the contract with a session key can look like this:

```typescript
const execute_msg =  {
    with_session_key: {
        msg: { wasm_msg: { execute: {...} }}
        session_key: "session_key",
    }
}
```

#### First Interaction
Operations on Secret Network require gas paid from a secret address we need a relayer to authenticate / create a proxy on behalf of a Cosmos user. 

Since the calling address is not used for authentication, the action can be subsidized by a dapp or a public community relayer

Another option is to use an IBC relayers e.g. Polyton or simply using IBC-Hooks. Keep in mind that payload including signature are first submiited to a remote chain which might not be encrypted

For subsequent interactions a client application can generate a local (e.g. browser local storage) wallet used for interacting with Secret Network behind the scenes. An proxy account contract might issue a feegrant so that the wallet can use the contract's funds for gas payments. 

### 3. Funding and Forwarding

Fees for executing transactions on Secret Network are paid in SCRT. In case of a local in-browser wallet must have some itself or alternatively a proxy account contract that issues a feegrant must have them

Getting SCRT to the wallet can be done in seemless manner in various ways. 

With IBC-Hooks mentioned earlier a user can send ibc/denom version of SCRT tokens on a remote chain and send them together with payload as a part of normal ics20 message

Another viable options is to send a different denom of a remote chain in similar manner and swap it for SCRT behind the scenes e.g. using Shade Protocol, Sienna, etc

Funding can be done atomically with a multi-message transaction (e.g. first message for swapping and the second for creating a funded proxy account over IBC-Hook).

Given there is a relayer for a first operation a funding is not required for authentication or creating an account per se. It can be done later at any point in time.

### 4. Executing Actions

All steps described actions above might be integrating into any smart contract and tailored to the specific needs of the dapp.

The most simple approach covering most of the use cases is to use a separate proxy account contract and use it as an entrypoint for interacting with any other contract / dapp on Secret Network.

A message for interacting with a remote contract through a proxy account can look like this:

```typescript
const execute_msg =  {
    // or `with_auth_data`
    with_session_key: {
        msg: { 
            cosmos_msg: { 
                wasm: {
                    execute: {
                        contract_addr: "remote contract address",
                        code_hash: "remote contract code hash",
                        msg: "binary message of action on remote contract",
                        funds: [{ coin1 }, { coin2 }]
                    }
                }
        }}
        session_key: "session_key",
    }
}
```


## Data Structures

### SessionConfig

Configuration for session management, determining how session keys are generated and managed.

```rust
struct SessionConfig {
    /// Whether to generate a session key for . 
    /// The rest of the properties are ignored is set to false
    /// Defaults to true.
    generate_key: Option<bool>,
    /// Whether to allow newly generated session key to also act as a viewing key for queries.
    can_view: Option<bool>,
    /// Sets an optional expiration for the session key.
    expires: Option<Expiration>,
}
```

### AbstractionParams

Parameters to abstract interaction within the Secret Network, including configurations for transaction signing and authorization.

```rust
struct AbstractionParams {
    /// Address to give a feegrant to use contracts's funds for gas payments.
    feegrant_signer: Option<String>,
    /// Whether to generate a new wallet seed phrase and automaticly give it a feegrant. Defaults to true.
    generate_signer: Option<bool>,
    /// Configuration for session key generation.
    session_key_config: Option<SessionConfig>,
}
```

### CosmosAuthData

Data used for user authentication and transaction authorization.

```rust
struct CosmosAuthData {
    /// Public key corresponding to the user's secret key used for signing.
    pubkey: Binary,
    /// Signed SHA256 digest of a message formatted according to 036.
    signature: Binary,
    /// Original message before wrapping and signing.
    message: String,
    /// Bech32 address prefix.
    hrp: Option<String>
}
```

### FundForwarding

Example with Squid Router inspired calls
```rust
struct Call<A = CallAction> {
    /// cosmos message to execute
    msg: CosmosMsg,
    /// actions to perfirm bebore the cosmos message
    actions: Vec<A>,
}
```


```rust
struct FundForwarding {
    /// list of calls to execute 
    calls: Vec<Call>,
}
```


### CosmosProxy

A proxy configuration for handling authentication and subsequent actions.

```rust
struct CosmosProxy<F = FundForwarding> {
    /// Parameters for abstract interaction settings.
    abstraction_params: AbstractionParams,
    /// Authentication and authorization data.
    auth_data: CosmosAuthData,
    /// Fund forwarding configuration.
    fund_forwarding: Option<F>,
    /// Optional payload to execute immediately after authentication.
    payload: Option<Binary>
}
```

### Utility Function

```rust
/// Verifies a signature of a message signed according to 036 specs.
/// 
/// Parameters:
/// - `api`: A reference to an object that implements the `Api` trait.
/// - `auth`: Authentication data containing the user's public key, message, and signature.
/// 
/// Returns:
/// - A standard result indicating success or failure.
fn verify_arbitrary(api: &dyn Api, auth: CosmosAuthData) -> StdResult<()> 
```


## Messages and Queries

The SDK include message to interact with a simple proxy account contract and a factory contract that can create new accounts. 


### Factory Messages

Handles the creation of accounts and management of encryption keys.

```rust
enum FactoryExecuteMsg {
    /// Creates a separate proxy account with the specified settings.
    CreateAccount(CreateAccountMsg),
    /// Resets the encryption key of the factory contract (admin only).
    ResetEncryptionKey {},
    /// Encrypts the message, excluding this variant itself.
    Encrypted(Binary)
}

enum FactoryQueryMsg {
    /// Queries the current encryption key of the factory.
    EncryptionKey {}
}
```

### Account Messages

Manages authentication, execution of delegated actions, and key management.

```rust
enum AccountExecuteMsg<T = Empty> {

    /// Authenticates using provided auth data and executes a given message.
    WithAuthData {
        msg          :   AuthorisedExecuteMsg<T>,
        auth_data    :   CosmosAuthData,
        padding      :   Option<String>,
        gas_target   :   Option<String>,
    },
     /// authenticate and execute a payload with a session key
    WithSessionKey {
        msg          :   AuthorisedExecuteMsg<T>,
        session_key  :   String,
        padding      :   Option<String>,
        gas_target   :   Option<Uint64>,

    },
    /// encrypt version of any other variant of this enum
    Encrypted(Binary)
}
```




```rust
enum AuthorisedExecuteMsg<T = Empty> {
    /// executing arbitrary actions
    Execute { 
        msgs: Vec<CosmosMsg<T>>,
    },
    /// syntax sugar for giving a fee grant a new address
    FeeGrant {
        grantee: String,
        // todo: specify prost and user-friendly rust types
        allowance: Binary
    },

    /// reset internal feegrant wallet if exists 
    ResetFeeGrantWallet {
        remove: Option<bool>
    },

    /// reset the encryption key of the account contract
    ResetEncryptionKey {},

    /// creating a viewing key from account to another contract
    CreateProxyViewingKey {
        contract: Contract,
        entropy: String,
    },

    /// setting a viewing key from account to another contract
    SetProxyViewingKey {
        contract: Contract,
        key: String,
    },

    /// create a viewing key to this contract
    CreateViewingKey {
        entropy: String,
    },

    /// set a viewing key to this contract
    SetViewingKey {
        key: String,
    },

    /// snip50 compliant message
    Evaporate {
        gas_target : Uint64,
    },
}
```



```rust
enum AccountQueryMsg {

    EncryptionKey {},

    WithKey {
        query: AuthorisedQueryMsg,
        key: String,
    },

    WithPermit {
        query: AuthorisedQueryMsg,
        permit: Permit,
    },

}


```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum AuthorisedQueryMsg {

    /// get a seed phrase of a wallet with feegrant to use this contract 
    #[returns(Option<String>)]
    FeeGrantWallet {},

}
```