import { MsgExecuteContractParams, MsgInstantiateContractParams, MsgInstantiateContractResponse, TxResultCode } from "secretjs";
import { Contract, CosmosCredential, InnerQueries, GatewayExecuteMsg as GatewayExecuteMsg, GatewaySimpleInitMsg, GatewayQueryMsg } from "./types";
import { loadCodeConfig, loadContractConfig } from "./config";
import { consumerWallet, secretClient } from "./clients";
import { getEncryptedSignedMsg } from "./crypto";
import { OfflineAminoSigner } from "@cosmjs/amino";
import { AminoWallet } from "secretjs/dist/wallet_amino";


export const instantiateGatewaySimple = async () : Promise<Contract> => {
    
    const config = loadCodeConfig();
    const code = config.gateway!;
    const hash = code.code_hash;

    const init_msg : GatewaySimpleInitMsg = {}

    const msg : MsgInstantiateContractParams = {
        code_id: code.code_id,
        code_hash: hash,
        sender: secretClient.address,
        label: `test-${Math.round(Date.now() / 1000)}`,
        init_msg
    }

    
    const tx = await secretClient.tx.compute.instantiateContract(msg, { gasLimit: 300_000 });

    if (tx.code !==  TxResultCode.Success) {
        throw new Error(`Error while instantiating contract: ${tx.rawLog}`);
    }

    const address = MsgInstantiateContractResponse.decode(tx.data[0]).address;

    return { address, hash }
}




export const getGatewayEncryptionKey = async () => {
    const res = await queryGateway({ encryption_key: {} });
    return res as string;
}




export const queryGateway = async (query: GatewayQueryMsg) => {
    const config = loadContractConfig();
    const res = await secretClient.query.compute.queryContract({
        contract_address: config.gateway!.address,
        code_hash: config.gateway!.hash,
        query
    });
    return res;
}


export const queryGatewayAuth = (query: InnerQueries, credentials: CosmosCredential[]) => {
    return queryGateway({
        with_auth_data: {
            query,
            auth_data: {
                credentials,
            }
        }
    })
}


export const executeGateway = async (execute_msg: GatewayExecuteMsg) => {
    const config = loadContractConfig();

    const msg : MsgExecuteContractParams<GatewayExecuteMsg> = {
        msg: execute_msg,
        sender: secretClient.address,
        contract_address: config.gateway!.address,
        code_hash: config.gateway!.hash,
        sent_funds: [],
    }
    const tx = await secretClient.tx.compute.executeContract(msg, { gasLimit: 900_000 });
    return tx;
}



export const executeGatewayEncrypted = async (
    execute_msg: GatewayExecuteMsg, 
    wallet?: OfflineAminoSigner | AminoWallet,
    gatewayKey?: string,
) => {

    return await executeGateway(
        await getEncryptedSignedMsg(
            wallet ?? consumerWallet,
            execute_msg,
            gatewayKey
        )
    )
}



