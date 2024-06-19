import { getRandomValues } from "crypto";
import { consumerWallet } from "./clients";
import { chacha20_poly1305_seal, ecdh } from "@solar-republic/neutrino"
import { bytes, concat, json_to_bytes } from "@blake.regalia/belt";
import { CosmosCredential, MsgSignData, GatewayExecuteMsg, EncryptedPayload } from "./types";
import { getGatewayEncryptionKey } from "./gateway";


import { makeSignDoc, OfflineAminoSigner, StdFee, StdTx, isSecp256k1Pubkey, serializeSignDoc, makeStdTx } from "@cosmjs/amino"
import { Secp256k1, Secp256k1Signature } from "@cosmjs/crypto"
import { fromBase64, toBase64, toAscii } from "@cosmjs/encoding";
import { AminoWallet, StdSignDoc } from "secretjs/dist/wallet_amino";
import { sha256 } from "@noble/hashes/sha256";


const accountNumber = 0;
const sequence = 0;
const chainId = "";
const fee: StdFee = { gas: "0", amount: [] };
const memo = "";



export const getEncryptedSignedMsg = async (
  signer          :   OfflineAminoSigner | AminoWallet,
  msg             :   GatewayExecuteMsg,
  gatewayKey?     :   string,
): Promise<GatewayExecuteMsg> => {

  gatewayKey ??=  await getGatewayEncryptionKey()

  const accounts = await signer.getAccounts();
  const firstAccount = accounts[0];

  const signerAddress = firstAccount.address;
  const signerPubkey = firstAccount.pubkey;

  const nonce            =  getRandomValues(bytes(12))
  const gatewayKeyBytes  =  fromBase64(gatewayKey);

  const sharedKey =  sha256(
    ecdh(consumerWallet.privateKey, gatewayKeyBytes)
  )
  
  const payload : EncryptedPayload = {
    user_address: signerAddress,
    user_pubkey: toBase64(signerPubkey),
    hrp: signerAddress.split("1")[0],
    msg: toBase64(json_to_bytes(msg))
  }
  

  const [ciphertextClient, tagClient] = chacha20_poly1305_seal(
    sharedKey,
    nonce,
    json_to_bytes(payload)
  )

  const ciphertext = concat([ciphertextClient, tagClient]);
  const ciphertextHash = sha256(ciphertext);

  const signDoc = getArb36SignDoc(signerAddress, ciphertextHash);
  const signRes = await signer.signAmino(signerAddress, signDoc);

  const stdTx = makeStdTx(signDoc, signRes.signature);
  const res = await verifyAdr36Tx(stdTx);

  if (!res) {
    throw new Error("Failed to verify signature");
  }

  return {
    encrypted: {
      nonce           :     toBase64(nonce),
      user_key        :     toBase64(consumerWallet.publicKey),
      payload         :     toBase64(ciphertext),
      payload_hash    :     toBase64(ciphertextHash),
      payload_signature:    signRes.signature.signature,
    }
  }
};


export const getArb36SignData = (
  signerAddress: string,
  data: string | Uint8Array,
) : MsgSignData => ({
  type: "sign/MsgSignData",
  value: {
    signer: signerAddress,
    data: typeof data === "string" ? data : toBase64(data),
  }
})



export const getArb36SignDoc = (
  signerAddress: string,
  data: Uint8Array,
) : StdSignDoc => {
  const msg = getArb36SignData(signerAddress, data);
  return makeSignDoc([msg], fee, chainId, memo, accountNumber, sequence);
}



export const getArb36Credential = async (
  signer:   OfflineAminoSigner | AminoWallet,
  data: string | Uint8Array,
) : Promise<CosmosCredential> => {
  const accounts = await signer.getAccounts();
  const firstAccount = accounts[0];
  const signerAddress = firstAccount.address;
  
  const message = typeof data === "string" ? toAscii(data) : data;
  const signDoc = getArb36SignDoc(signerAddress, message);
  const signRes = await signer.signAmino(signerAddress, signDoc);

  const res = {
    signature: signRes.signature.signature,
    pubkey: signRes.signature.pub_key.value,
    message: toBase64(message),
    hrp: firstAccount.address.split("1")[0]
  }

  return res;
}



export const verifyAdr36Tx = async (signed: StdTx): Promise<boolean> => {
  // Restrictions from ADR-036
  if (signed.memo !== "") throw new Error("Memo must be empty.");
  if (signed.fee.gas !== "0") throw new Error("Fee gas must 0.");
  if (signed.fee.amount.length !== 0) throw new Error("Fee amount must be an empty array.");

  const accountNumber = 0;
  const sequence = 0;
  const chainId = "";

  // Check `msg` array
  const signedMessages = signed.msg;
  if (signedMessages.length === 0) {
    throw new Error("No message found. Without messages we cannot determine the signer address.");
  }

  const signatures = signed.signatures;
  if (signatures.length !== 1) throw new Error("Must have exactly one signature to be supported.");
  const signature = signatures[0];
  
  if (!isSecp256k1Pubkey(signature.pub_key)) {
    throw new Error("Only secp256k1 signatures are supported.");
  }

  const signBytes = serializeSignDoc(
    makeSignDoc(signed.msg, signed.fee, chainId, signed.memo, accountNumber, sequence),
  );
  const prehashed = sha256(signBytes);

  const secpSignature = Secp256k1Signature.fromFixedLength(fromBase64(signature.signature));
  const rawSecp256k1Pubkey = fromBase64(signature.pub_key.value);

  const ok = await Secp256k1.verifySignature(secpSignature, prehashed, rawSecp256k1Pubkey);
  return ok;
}