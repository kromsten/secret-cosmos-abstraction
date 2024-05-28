import { SecretNetworkClient, Wallet } from "secretjs"
import { 
    CONSUMER_CHAIN_ENDPOINT, CONSUMER_CHAIN_ID, CONSUMER_MNEMONIC, 
    SECRET_CHAIN_ENDPOINT, SECRET_CHAIN_ID, SECRET_MNEMONIC 
} from "./env"


export const secretWallet = new Wallet(SECRET_MNEMONIC);


export const secretClient = new SecretNetworkClient({
    chainId: SECRET_CHAIN_ID!,
    url: SECRET_CHAIN_ENDPOINT!,
    wallet: secretWallet,
    walletAddress: secretWallet.address,
});


export const consumerWallet = new Wallet(CONSUMER_MNEMONIC);


export const consumerClient = new SecretNetworkClient({
    chainId: CONSUMER_CHAIN_ID!,
    url: CONSUMER_CHAIN_ENDPOINT!,
    wallet: consumerWallet,
    walletAddress: consumerWallet.address
});
