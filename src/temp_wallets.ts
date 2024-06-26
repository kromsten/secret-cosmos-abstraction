import { Slip10Curve, Random, Bip39, Slip10, stringToPath, Secp256k1 } from "@cosmjs/crypto"
import { sk_to_pk, gen_sk } from "@solar-republic/neutrino"
import { Wallet } from "secretjs";


export const randomCosmWallet = async () => {
    const seed = await Bip39.mnemonicToSeed(Bip39.encode(Random.getBytes(16)));
    const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, stringToPath("m/44'/1'/0'/0"));
    const pair = await Secp256k1.makeKeypair(privkey);
    const pubkey = Secp256k1.compressPubkey(pair.pubkey);
    console.log("cosm privkey", privkey)
    console.log("cosm pubkey", pubkey)
}

export const randomNobleWallet = async () => {
    const clientPrivKey = gen_sk()
    const clientPubKey = sk_to_pk(clientPrivKey)
    console.log("solar clientPrivKey", clientPrivKey)
    console.log("solar clientPubKey", clientPubKey)
}

export const randomSecretWallet = async () => {
    const { privateKey, publicKey } = new Wallet()
    console.log("secret clientPubKey", privateKey)
    console.log("secret clientPrivKey", publicKey)
}

/* 
export const test = async () => {

    const config = loadContractConfig();
    const gw = config.gateway!
    
    const c = await SecretContract(SECRET_CHAIN_ENDPOINT, gw.address);
    const res = await c.query({ encryption_key: {} })
    console.log("test res", res)
    return res
  }
 */  

await randomCosmWallet()
console.log("=====================================")
await randomNobleWallet()
console.log("=====================================")
await randomSecretWallet()
  