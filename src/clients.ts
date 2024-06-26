import { SecretNetworkClient, Wallet } from "secretjs"
const { 
    CONSUMER_CHAIN_ENDPOINT, 
    CONSUMER_CHAIN_ID, 
    CONSUMER_MNEMONIC, 
    CONSUMER_PREFIX, 
    CONSUMER_TOKEN, 

    SECRET_CHAIN_ENDPOINT, 
    SECRET_CHAIN_ID, 
    SECRET_MNEMONIC 
} = process.env;

//import { gen_sk, sk_to_pk } from "@solar-republic/neutrino";


export const secretWallet = new Wallet(SECRET_MNEMONIC);


export const secretClient = new SecretNetworkClient({
    chainId: SECRET_CHAIN_ID!,
    url: SECRET_CHAIN_ENDPOINT!,
    wallet: secretWallet,
    walletAddress: secretWallet.address,
});


export const consumerWallet = new Wallet(CONSUMER_MNEMONIC, {
    bech32Prefix: CONSUMER_PREFIX,
    coinType: CONSUMER_TOKEN == "uscrt" ? 529 : 118,
});


export const consumerClient = new SecretNetworkClient({
    chainId: CONSUMER_CHAIN_ID!,
    url: CONSUMER_CHAIN_ENDPOINT!,
    wallet: consumerWallet,
    walletAddress: consumerWallet.address
});




/* export const clientWallet = (() => {
    const privateKey = gen_sk(); 
    const publicKey = sk_to_pk(privateKey);
    return { privateKey, publicKey }
})()

 */