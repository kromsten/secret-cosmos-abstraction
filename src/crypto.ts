import { getRandomValues } from "crypto";
import { consumerWallet } from "./clients";
import { chacha20_poly1305_seal, ecdh } from "@solar-republic/neutrino"
import { bytes, concat, json_to_bytes, JsonValue, sha256 } from "@blake.regalia/belt";
import { InnerMethods, RegistryExecuteMsg } from "./types";
import { fromBase64, toBase64 } from "secretjs";
import { getGatewayRegistryEncryptionKey } from "./registry";


export const getEncryptedGatewayRegistryMsg = async (
  msg             :   InnerMethods,
  gatewayKey?     :   string,
): Promise<RegistryExecuteMsg> => {


  gatewayKey ??=  await getGatewayRegistryEncryptionKey()

  const nonce            =  getRandomValues(bytes(12))
  const gatewayKeyBytes  =  fromBase64(gatewayKey);

  const sharedKey =  await sha256(
    ecdh(consumerWallet.publicKey, gatewayKeyBytes)
  )

  const [ciphertextClient, tagClient] = chacha20_poly1305_seal(
    sharedKey,
    nonce,
    json_to_bytes(msg as JsonValue)
  )

  const ciphertext = concat([ciphertextClient, tagClient]);
  const ciphertextHash = await sha256(ciphertext);
  const msgHash = await sha256(json_to_bytes(msg as JsonValue));

  /* const _info = {
    user_key: hexlify(userPublicKeyBytes),
    user_pubkey: user_pubkey,
    routing_code_hash: routing_code_hash,
    task_destination_network: "pulsar-3", //Destination for the task, here: pulsar-3 testnet
    handle: handle,
    nonce: hexlify(nonce),
    payload: hexlify(ciphertext),
    payload_signature: payloadSignature,
    callback_gas_limit: Number(callbackGasLimit),
  }; */


  return {
    encrypted: {
      msg             :     toBase64(ciphertext),
      nonce           :     toBase64(nonce),
      public_key      :     toBase64(consumerWallet.publicKey),
    }
  }
};