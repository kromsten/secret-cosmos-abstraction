import { SecretNetworkClient, Wallet } from "secretjs"
import { SigningStargateClient } from "@cosmjs/stargate"
import { Secp256k1HdWallet } from "@cosmjs/amino";
import { NonceWallet } from "./types";
import { CONSUMER_DECIMALS, CONSUMER_GAS_PRICE, CONSUMER_TOKEN } from "./env";
import { Decimal } from "@cosmjs/math";


const { 
    CONSUMER_CHAIN_ENDPOINT, 
    CONSUMER_MNEMONIC, 
    CONSUMER_PREFIX, 
    SECRET_CHAIN_ENDPOINT, 
    SECRET_CHAIN_ID, 
    SECRET_MNEMONIC 
} = process.env;


export const secretWallet = new Wallet(SECRET_MNEMONIC);


export const secretClient = new SecretNetworkClient({
    chainId: SECRET_CHAIN_ID!,
    url: SECRET_CHAIN_ENDPOINT!,
    wallet: secretWallet,
    walletAddress: secretWallet.address,
});


let consumerWallet : Secp256k1HdWallet | undefined;
let consumerClient : SigningStargateClient | undefined;
let nonceWallets : { [nonce: string]: NonceWallet } = {};



export const getNonceWallet = async (base64Nonce : string) => {
    if (!nonceWallets[base64Nonce]) {
        nonceWallets[base64Nonce] = new NonceWallet();
    }
    return nonceWallets[base64Nonce];
}


export const getConsumerWallet = async () => {
    if (!consumerWallet) {
        consumerWallet = await Secp256k1HdWallet.fromMnemonic(CONSUMER_MNEMONIC!, { prefix: CONSUMER_PREFIX! });
    }
    return consumerWallet;
}


export const getConsumerClient = async (wallet? : Secp256k1HdWallet) => {
    if (!consumerClient) {
        if (!wallet && !consumerWallet) {
            throw new Error("No wallet avaialable");
        }
        consumerClient = await SigningStargateClient.connectWithSigner(
            CONSUMER_CHAIN_ENDPOINT!, 
            wallet ?? consumerWallet!,
            { gasPrice: { 
                denom: CONSUMER_TOKEN!, 
                amount: Decimal.fromUserInput(
                    CONSUMER_GAS_PRICE ?? "0.25", 
                    CONSUMER_DECIMALS ? Number(CONSUMER_DECIMALS) : 6
                ) 
            }}
        );
    }
    return consumerClient;
}
