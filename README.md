# Cosmos Wallet Abstraction on Secret Network

---

## Introduction

This document provides a detailed look into SDK, which facilitates the creation and management of proxy accounts to operate on Secret Network with remote Cosmos addresses. It includes utilities for account management, transaction signing, and integration with custom contracts

## Overview

The SDK is designed to abstract the complexities involved in interacting with the Secret Network for applications that deal with Cosmos wallets. It introduces a secure method for authenticating users and managing session keys, allowing decentralized applications (dApps) to authenticate users via signatures using the arbitrary signatures [036](https://docs.cosmos.network/main/build/architecture/adr-036-arbitrary-signature) specification

Session keys, once generated, can be used to bypass the need to generate a new signature and do its cryptographic verification every single time. Session keys can follow similar format of viewing keys and can be produced using existing utility functions from the secret-toolkit. It's recommend to allow session keys to be used as viewing keys for queries, as it simplifies the process of managing keys and permissions. Regular viewing keys created from signed permit can be also should also be supported for compatibility with existing ecosystem.

For security and modularity, it's recommended to separate authentication logic into its own contract, using it as a proxy for user actions. This not only enhances security but also simplifies contract design by decoupling authentication from business logic.

The SDK includes message for interacting with an example proxy account contract and with a factory contract used for creating new accounts


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


```rust
struct FundForwarding {
    // ToBeDefined
}
```


### CosmosProxy

A proxy configuration for handling authentication and subsequent actions.

```rust
struct CosmosProxy {
    /// Parameters for abstract interaction settings.
    abstraction_params: AbstractionParams,
    /// Authentication and authorization data.
    auth_data: CosmosAuthData,
    /// Fund forwarding configuration.
    fund_forwarding: Option<FundForwarding>,
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
    /// Authenticate and optionaly generate abstraction primitivas with optional action payload.
    Authenticate(CosmosProxy),
    /// Authenticates using provided auth data and executes a given message.
    WithAuthData {
        msg: AuthorisedExecuteMsg<T>,
        auth_data: CosmosAuthData,
        padding: Option<String>,
        gas_target: Option<String>,
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