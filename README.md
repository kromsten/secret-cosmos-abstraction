# Cosmos Wallet Abstraction on Secret Network

### Prerequisites

`docker-compose` for setting up local blockchain nodes and relayer

`Rust` and `cargo` for compiling contracts

`Node.js` runtume for running tests


## Installation



### Running nodes
To setup  local blockhain nodes + relayer (Hermes)
```
docker-compose up --build -d
```
The command will start 2 local secret network nodes and a relayer but one can treat one of them as a regular "consumer" network

Wait before hermes create a connection and a channel between two chains before running tests

##### Note: Local Secret node only work if your machince supports SGX



### Preparing contracts
Compile contracts using
```
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
```
and move the final wasm into `artifacts` folder

You might want to use a docker container for optimizing the builds

There is a Makefile script that compiles the contract and moves it to the artifacts folder automatically
```
make updest
``` 
### Configuration

The test-suite rely on configuration files for automatically deploying the contracts, recognsing the IBC connection with other primitives and persisting them between the test runs

Running the test will automatically create them in "config" folder. If you need to update the contract code or instantiate a new version of the contract for the test you can delete one of the respective configuration files or alter the content by removing respective fields

Refer to `src/config.ts` file and respective types it imports from `src/types.ts` for the structure of the configuration files

### Running tests
You can already run commands using
```npm test```
but if you are missing the configuration files for IBC and deployed contract the test-suite will create them automatically

At the moment there is non-fixed racing condition issue due to which test sometimes crashes on account sequence errors. To avoid them you can run one test-suite only for the first time. E.g.
```npm test setup```

IBC tests might take a while due to waiting for acknowlegements


## Overview

The SDK is designed to abstract the complexities involved in interacting with the Secret Network for applications that deal with Cosmos wallets. It introduces a secure method for generating confidentials messages and reliably athenticating users at the same time thanks to `chacha20poly1305` algorithm

The SDK can be used be for developing major gateways that forward incoming messages forward to the network but the primives released here also make it quite straightforard to add built-in support for confidental messages directly in other contracts


## Rust SDK & Contracts

This SDK extensively uses Rust template engines and trait to allow users to customise existing data structures and reuse them without need and re-defining everything for introducing small changes.

### Data Structures
The essential parameters required for `chacha20poly1305`flow are defined in the following data structure

```Rust

/// A data structure that is safe to be visible by all network participants and can be transmited over non-secure channels
struct EncryptedParams {
    /// Encrypted payload containging hidden message
    pub payload            :   Binary,
    /// Sha256 hash of the payload
    pub payload_hash       :   Binary,
    /// Signed base64 digest of the payload_hash being wrapped
    /// in an cosmos arbitrary (036) object and rehashed again with sha256
    pub payload_signature  :   Binary,
    /// Public key of wallet used for deriving a shared key for chacha20_poly1305
    /// Not necessary the same as user's public key 
    pub user_key           :   Binary,
    /// One-time nonce used for chacha20_poly1305 encryption
    pub nonce              :   Binary,
}

/// Data meant to be encrypted and stored in the payload field of [EncryptedParams]
#[cw_serde]
pub struct EncryptedPayload {
    /// bech32 prefix address of a wallet used for signing hash of the payload 
    pub user_address   :  String,
    /// Public key of a wallet used for signing hash of the payload 
    pub user_pubkey   :   Binary,
    /// Human readable prefix for the bech32 address on the remote cosmos chain
    pub hrp           :   String,
    /// Plaintext message e.g. normal `ExecuteMsg` of your contract
    pub msg           :   Binary,
}

```

### Custom Contract Message


Your contract must define and endpoint where a user can pass all the required fields of the `EncryptedParams`. E.g:

```Rust
pub enum ExecuteMsg {
    ...

    Encrypted {
        payload             :   Binary,
        payload_signature   :   Binary,
        payload_hash        :   Binary,
        user_key            :   Binary,
        nonce               :   Binary,
    }
    ...

}
```


If you want to define a custom message, rename the fields or add addition ones there is a helpful trait `WithEncryption` that you can implement. It simply tells the compiler how to extract the essential parameters from your custom message and turin it into `EncryptedParams`
```Rust
trait WithEncryption : Serialize + Clone  {
    fn encrypted(&self)     -> EncryptedParams;
    fn is_encrypted(&self)  -> bool;
}
```

 Implementing the trait for your message will allow you to use other useful methods of the SDK (like `handle_encrypted_wrapper`)  that significantly simplify the development experience. Example of the implementation for the `ExecuteMsg` is as follows:

 ```Rust
 impl WithEncryption for ExecuteMsg {
    fn encrypted(&self)     -> EncryptedParams {
        match self.clone() {
            ExecuteMsg::Encrypted {
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
            _ => panic!("Not encrypted")

        }
    }

    fn is_encrypted(&self)  -> bool {
        if ExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}
```


### Extending existing data structures

The SDK has multiple data structures that already implement `WithEncryption` trait and also use the template engine of Rust to make them easily extendable. Take for example the following message


```Rust
pub enum GatewayExecuteMsg<E = Option<Empty>> 
    where E: JsonSchema
{
    ResetEncryptionKey  {} ,

    Encrypted {
        payload             :   Binary,
        payload_signature   :   Binary,
        payload_hash        :   Binary,
        user_key            :   Binary,
        nonce               :   Binary,
    },

    Extension {
        msg : E
    }
}
```

You can define a new message that extends the `GatewayExecuteMsg` by simply providing a new type for the `Extension` instead of the default `Option<Empty>` like this:

```Rust
// Degine your custom message
#[cw_serde]
pub enum MyCustomMessage {
    HandleFoo {}
    HandleBar {}
}
// Extend the GatewayExecuteMsg
pub type MyGatewayExecuteMsg = GatewayExecuteMsg<MyCustomMessage>;
```

Your extended type in this case be available under `MyGatewayExecuteMsg::Extension` variant and you can use it in your contract like this:
```Rust
/// MyGatewayExecuteMsg
match msg {
    ... 
    ResetEncryptionKey => { ... },

    MyGatewayExecuteMsg::Extension{msg} => {

        /// MyCustomMessage
        match msg {
            MyCustomMessage::HandleFoo{} => {
                // Do something
            }
            MyCustomMessage::HandleBar{} => {
                // Do something
            }
        }
    }
    
    ...
}
```



### Functions and methods


#### `handle_encrypted_wrapper`
If you have a contract meant to be used in both secure and non-secure environment it might be quite convinient to define one common message with regular (secure environment) messages and with encrypted (non-secure environment) variant that contains the same regular variants (vs arbitrary message).  Let's take the following example of the `ExecuteMsg`

```Rust
enum ExecuteMsg {
    ...
    /// for secure environments
    HandleFoo { 
        ... 
    },
    /// for secure environments
    HandleBar { 
        ... 
    },
    /// for non-secure environments
    Encrypted {
        // contains `ExecuteMsg::HandleFoo {...}` or `ExecuteMsg::HandleBar` inside
        payload             :   Binary, 
        payload_signature   :   Binary,
        payload_hash        :   Binary,
        user_key            :   Binary,
        nonce               :   Binary,
    }
    ...
}
```

If the message implements `WithEncryption` trait it is becoming possible to use `handle_encrypted_wrapper` under default `common` feature flag function from the SDK that automatically detects if we are currently dealing with the encrypted version and automatically tries to extract one of the sister (secure environment) variants of the message. 

```Rust
pub fn execute(
    deps: DepsMut,
    env: Env,

    // Secure envieronmnt sender 
    // or relayer / nullified  info
    info: MessageInfo,

    // HandleFoo { ... } or HandleBar { ... }
    // or Encrypted { ... }
    msg: ExecuteMsg,

) -> Result<Response, ContractError> {
    
    let (

        // HandleFoo { ... } or HandleBar { ... }  
        // !!!  only  !!!
        msg, 

        // Authenticated remote chain user info
        info

    ) = sdk::common::handle_encrypted_wrapper(
        deps.api, deps.storage, info, msg
    )?;


    // Your normal contract logic here
    // ... 

```

The function requires `handle_reset_encyption_wallet` to be called beforehand or a keypair (`SecretEncryptionWallet`) to be set manually under the respective storage keys 


#### `chacha20poly1305_decrypt`

In case there is need for complete customisation it's allways possible to take invidual components. They've beem designed to be as unrestrictive as possible. For example the following function can use the following types for as the input parameters which can also be mixed:
   - `cosmwasm_std::Binary`,
   - `std::vec::Vec`.
   - `[u8]`
   - and others that implement `Deref<Target = [u8]>` trait

```Rust
pub fn chacha20poly1305_decrypt(
    ciphertext    :     &impl Deref<Target = [u8]>,
    key           :     &impl Deref<Target = [u8]>,
    nonce         :     &impl Deref<Target = [u8]>,
) -> StdResult<Vec<u8>> {
    ...
}
```

### Various authentication utilities

To verify a message that was was signed through a method `cosmos arbitrary (036)` message format you can use the following function

```Rust
fn verify_arbitrary<M : Display>(api:  &dyn Api, cred: &CosmosCredential<M>) -> StdResult<String>
```
The method takes in a `CosmosCredential` struct as an argument which is a a helpful wrapper over essential required fields rqquried for the verification:

```Rust
pub struct CosmosCredential<M = String> 
    where M: Display
{
    /// public key matching the respective secret that was used to sign message
    pub pubkey    :   Binary,
    /// signed sha256 digest of a message wrapped in arbitary data (036) object
    pub signature :   Binary,
    /// signed inner message before being wrapped with 036
    pub message   :   M,
    /// prefix for the bech32 address on remote cosmos chain
    pub hrp       :   String
}
```
Both `CosmosCredential` and `EncryptedParams` can be used with `String` or base64 encoded `Binary` types


To generate a preamble message for the `cosmos arbitrary (036)` message format you can use the following utility function

```Rust
fn preamble_msg_arb_036(signer: &str, data: &str) -> String
```
The function uses a hardcoded JSON strnig with all the required keys present and sorted


## TypeScript SDK


Similar to the Rust the flow of chacha20poly1305 consist of the two big parts however in this case instead of **decryption**` and **signature verification** as on the contract sife we have **encryption** and **signature generation**

#### Dependencies

it is recommended to use dedicated packages for both of the parts. For encryption we recommend to use `@solar-republic/neutrino` that has useful `chacha20poly1305`related functionalities and additionaly  primitives for generation ephemereal lighweight wallets. 
Installation:

```bash
npm install --save @solar-republic/neutrino
```

For signing, encoding and other cryptographic needs in the Cosmos ecosystem it is common to use the suite of `@cosmjs` packages. You can install the following ones:
 
```bash
npm install --save @cosmjs/crypto @cosmjs/amino @cosmjs/encoding
```

If you are developing in the browser environment or connecting to a public network  you might also need
```bash
npm install --save @cosmjs/stargate
# or
npm install --save @cosmjs/cosmwasm-stargate
```

##### Note:  You can also use any other Typescript / Javascript package managers and runtimes e,g,  `bun`, `yarn`, `pnpm` etc.





### Generating Wallets

For the flow of chacha20poly1305 we need to use a crypthographic keypair and it's advised to use one that isn't the same as the user's wallet. The SDK provides a method for generating a new wallet that can be used for encryption purposes. For our purposes we just need a  private / public keys of Secp256k1 type and there are various ways to generate them. 

##### `@cosmjs/crypto` 
```typescript
import { Slip10Curve, Random, Bip39, Slip10, stringToPath, Secp256k1 } from "@cosmjs/crypto"

const seed = await Bip39.mnemonicToSeed(Bip39.encode(Random.getBytes(16)));
const { privateKey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, stringToPath("m/44'/1'/0'/0"));
const pair = await Secp256k1.makeKeypair(privateKey);
// must be compressed to 33 bytes from 65
const publicKey = Secp256k1.compressPubkey(pair.pubkey);
```

##### `@solar-republic/neutrino` 
```typescript
import { gen_sk, sk_to_pk } from "@solar-republic/neutrino"

const privateKey = gen_sk(); 
const publicKey = sk_to_pk(privateKey);
```

##### `@secretjs` 
```typescript
import { Wallet } from "secretjs";
const { privateKey, publicKey } = new Wallet()
```

### Query Client

Before proceeding to encryption you might want to create a quering client that will be used for querying the state and contract of the Secret Network. At very least it is requited for fetching the public key of a gateway contract for deriving a shared key used later for encryption

To perform a simple query on a secret contract we can use methods from `@solar-republic/neutrino`:

```typescript 
import { SecretContract } from "@solar-republic/neutrino"

// create a contract instantse
const contract = await SecretContract(
    secretNodeEndpoint,
    secretContractAddress
)

// query example:
// get encryption key from a gateway contract
const queryRes = await contract.query({ encryption_key: {} })


// extract res value
const gatewayPublicKey = queryRes[2]
```


For more persistent use-cases you can use `secretjs`

```typescript 
import { SecretNetworkClient } from "secretjs"

// create a client:
const client = new SecretNetworkClient({
    chainId, 
    url // endpoint URL of the node
});

// query the contact and get the value directly
const gatewayPublicKey =  await client.query.compute.queryContract({
    contract_address
    code_hash, // optionally
    { encryption_key: {} } // query msg
});
```

### Signatures

To make sure that malicious applications aren't tricking user into signing an actual blockchain transactions it is discouraged to use sign arbitrary blobs of data. To address the situataion there are various standard that inject additional data to the message before signing it. The most used one in the Cosmos ecosystem is defined in [ADR 036](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-036-arbitrary-signature.md) which is also used in the SDK


#### Browser Wallets
Most of the Cosmos wallets provide a method for signing arbitrary messages following the mentioned specification.

Here is a definition taken from documentation of [Keplr wallet](https://docs.keplr.app/api/#request-signature-for-arbitrary-message):


```typescript 
// window.keplr.signArbi....
signArbitrary(chainId: string, signer: string, data: string | Uint8Array) : Promise<StdSignature>
```
Although the API method requires a `chainId` it is set to empty string before signing the message

##### Cosmology

Cosmology [defines](https://docs.cosmology.zone/cosmos-kit/hooks/use-chain-wallet#methods-from-wallet-client) `signArbitrary` method as part of the interface for their wallet client and provides implementation / intergration for every popular Cosmos wallet out there


#### CosmJS

The logic of the method has already been implemented and proposed as an addition to the library however it has been hanging in a unmerged PR for a while. You can find the full implementation with examples and tests[ [PR] Here](https://github.com/cosmos/cosmjs/issues/844) 


#### Manually


##### Getting Signer and Signer Address

Firstly we need to get an amino signer that will be used for generating the siganture. `@cosmjs` has a defined interface `OfflineAminoSigner` with  `signAmino` and `getAccounts` methods and any other signer that implements it can be used for the purpose. 

```typescript
// In browser environment we can get it from a wallet extension. E.g with Keplr:
const signer = window.keplr.getOfflineSigner(chainId);


//  ...


// In Node environment we can use `Secp256k1Wallet` class that also implements `OfflineAminoSigner` interface
import { Secp256k1Wallet } from "@cosmjs/amino"


// see examples of generating a random above but in this case you will probably be using a persistent one from a .env file 
// you can also pass extra options like prefix as the second argument
const signer = await Secp256k1HdWallet.fromMnemonic(userMnemonic) 


//  ...

// Here we are getting the first accounts from the signer
// accessing the address and renaming it to `signerAddress` 
const [{ address : signerAddress }] =  await signer.getAccounts();

//  ...
```


##### Generating the message and `StdSignDoc`

The use `signAmino` we need to generate a `StdSignDoc` object that will be used for signing the message to pass as an argument


CosmJS provides a function for this: 

```typescript
function makeSignDoc(msgs: AminoMsg[], fee: StdFee, chainId: string, memo: string | undefined, accountNumber: number | string, sequence: number | string, timeout_height?: bigint): StdSignDoc;
```

The 036 standard requires the message the fields to AminoMsg to to be:
```typescript
type AminoMsg = {
    // static type url
    type: ;
    value: {
        // signer address
        signer: string;
        // plaintext or base64 encoded message
        data: string;
    }
}
```
As for the rest of the fields they can be set to empty string or 0. The final example will look like this:

```typescript
const data = "my message";


const signDoc  = makeSignDoc(
    [{                              // list of amino messages
        type: "sign/MsgSignData", 
        value:  { 
            signer: signerAddress, 
            data:   data 
        }
    }], 
    { gas: "0", amount: [] },      // StdFee
    "",                            // chainId 
    "",                            // memo
    0,                             // accountNumber
    0                              // sequence
    // timeout_height
)
```

After getting the document we can only need to sign it with the signer

```typescript
const signRes = await signer.signAmino(signerAddress, signDoc);
``` 


### Encryption

After getting a public key of a gateway contract you can use it to derive a shared key like this

```typescript
import { sha256, Random } from "@cosmjs/crypto"
import { fromBase64, toBase64, toAscii } from "@cosmjs/encoding";
import { chacha20_poly1305_seal, ecdh } from "@solar-republic/neutrino"
// this is a dependency of `@solar-republic/neutrino` so consider importing these methos 
import { concat, json_to_bytes } from "@blake.regalia/belt";


// ...
//   define
//  
//      `clientPrivateKey`
//      `gatewayPublicKey`
//      `signerAddress`
//
//   like described above
// ...


const sharedKey = sha256(ecdh(clientPrivateKey, gatewayPublicKey))

// We also need to generate a one-time nonce, which can be done like this:
const nonce =  Random.getBytes(12)

// Prepare a message for the action you want to take on the contract
const msg = ExecuteMsg { ...}


/// Defining payload structure idential to the Rust data structure
const payload : EncryptedPayload = {
    // e.g, cosmos1...
    user_address: signerAddress,
    // uint8array -> base64  (-> Binary)
    user_pubkey: toBase64(signerPubkey),
    // e.g. "cosmos" from "cosmos1..."
    hrp: signerAddress.split("1")[0], 
    // or to toBinary(msg) from `@cosmjs/cosmwasm-stargate`
    msg: toBase64(json_to_bytes(msg))
}


/// getting the payload ciphertext 
const ciphertext = concat(chacha20_poly1305_seal(
    sharedKey,
    nonce,
    // or toUtf8( JSON.stringify(payload) )  
    json_to_bytes( payload )
));

// finally the payload_hash is sha256 of the ciphertext
const ciphertextHash = sha256(ciphertext);


//    ...

```

### Encrypting + Signing

Produced digest of hashing the ciphertext can be used as our message that we want to sign according to the 036 standard. The final message will look like this:

```typescript
// calling `makeSignDoc` with nullifed fields like described earlier
const signDoc = getArb36SignDoc(signerAddress, ciphertextHash);
// signing the message
const signRes = await signer.signAmino(signerAddress, signDoc);
```

After this we are getting all the required fields for creting an `EncryptedPayload` message or an `ExecuteMsg::Encrypted { ... }`
    
```typescript
const encrypted = {
    // uint8array -> base64  (-> Binary)
    nonce              :     toBase64(nonce),
    // public key of a pair that was used in deriving a shread key 
    user_key           :     toBase64(clientPublicKey),
    // ciphertext of with the user data and actual message
    payload            :     toBase64(ciphertext),
    // sha256 hash of the ciphertext
    payload_hash       :     toBase64(ciphertextHash),
    // signatire over sha256( getArb36SignDoc( payload_hash ) ) already in base64
    payload_signature  :     signRes.signature.signature,
}
```


### Broadcasting the message

The encrypted message is safe to broadcast over public blockchain and other infrasturcture. A common use-case in context of Cosmos account might be broadcasting it over IBC originating from a chain other than the Secret Network. 

The potential use-case might involve broadcasting the message by initiating an IBC message directly and attaching the message as a payload (IBC-Hook) or passing the nessage to a smart contract on remote chain to process and bridge it to the Secret Network itself afterwards

Since Cosmwasm is quite flexible with defining the messages due to supporting JSON serialization it is possible the process is very similar in both cases so we only going to cover IBC-hooks for simplicity


```typescript 
import { MsgTransfer } from "cosmjs-types/ibc/applications/transfer/v1/tx";
import { SigningStargateClient, MsgTransferEncodeObject } from "@cosmjs/stargate";

/// creating a client
const client = await SigningStargateClient(
    "https://rpc.cosmoshub.io"    //  endpoint of the remote network 
    signer,                       // offlineSigner  
)


// defining the IBC transfer message
const msg : MsgTransferEncodeObject = {
    typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",
    value: MsgTransfer.fromPartial({
      sender:   signerAddress,
      receiver: secretGatewayContractAddress,
      sourceChannel: "channel-0",
      sourcePort: "transfer",
      timeoutTimestamp:  BigInt(
        // 5 minutes from now           |  ms -> ns
        Math.floor(Date.now() + 300_000) * 1_000_000
      ),
      // IBC Hook memo msg
      memo: JSON.stringify({
        wasm: {
            // must be same as receiver
            contract: secretGatewayContractAddress,
            // encrypted message defined above
            msg: encrypted
        }
      })
    })
}

// signing and broadcasting the message
const res = await client.signAndBroadcast(signerAddress, [msg])



// - Finish
```
