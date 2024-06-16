import { MsgExecuteContractParams, MsgInstantiateContractParams, MsgInstantiateContractResponse, TxResultCode } from "secretjs";
import { Contract, CosmosCredential, InnerMethods, InnerQueries, RegistryExecuteMsg, RegistryInitMsg, RegistryQueryMsg } from "./types";
import { loadCodeConfig, loadContractConfig } from "./config";
import { secretClient } from "./clients";
import { SECRET_CHAIN_ID } from "./env";


export const instantiateRegistry = async () : Promise<Contract> => {
    
    const config = loadCodeConfig();
    const code = config.registry!;
    const hash = code.code_hash;

    const init_msg : RegistryInitMsg = {
        allowed_code_ids  : [config.account!.code_id],
    }

    

    const msg : MsgInstantiateContractParams = {
        code_id: code.code_id,
        code_hash: hash,
        sender: secretClient.address,
        label: `test-${Math.round(Date.now() / 1000)}`,
        init_msg
    }

    console.log("Instantiating registry contract msg: ", msg);

    const tx = await secretClient.tx.compute.instantiateContract(msg, { gasLimit: 300_000 });

    if (tx.code !==  TxResultCode.Success) {
        throw new Error(`Error while instantiating contract: ${tx.rawLog}`);
    }

    const address = MsgInstantiateContractResponse.decode(tx.data[0]).address;

    return { address, hash }
}




export const createAccount = async (
    credentials: CosmosCredential[],
) => {
    
    const code = loadCodeConfig().account!;
    const config = loadContractConfig();


    const execute_msg : RegistryExecuteMsg = {
        create_account: {
            chain_id: SECRET_CHAIN_ID!,
            code_id: code.code_id,
            code_hash: code.code_hash,
            msg: {
                abstraction_params: {
                    session_config: {
                        generate_on_auth: false,
                    },
                    generate_signing_wallet: true,
                },
                auth_data: {
                    credentials,
                },
                extension: {}
            }
        }
    }

    const msg : MsgExecuteContractParams<RegistryExecuteMsg>  = {
        msg: execute_msg,
        sender: secretClient.address,
        contract_address: config.registry!.address,
        code_hash: config.registry!.hash,
        sent_funds: [],
    }

    const tx = await secretClient.tx.compute.executeContract(msg, { gasLimit: 900_000 });


    return tx;
}




export const getGatewayRegistryEncryptionKey = async () => {
    const res = await queryGatewayRegistry({ encryption_key: {} });
    return res as string;
}




export const queryGatewayRegistry = async (query: RegistryQueryMsg) => {
    const config = loadContractConfig();
    const res = await secretClient.query.compute.queryContract({
        contract_address: config.registry!.address,
        code_hash: config.registry!.hash,
        query
    });
    return res;
}


export const queryGatewayRegistryInner = (query: InnerQueries, credentials: CosmosCredential[]) => {
    return queryGatewayRegistry({
        with_auth_data: {
            query,
            auth_data: {
                credentials,
            }
        }
    })
}


export const executeGatewayRegistry = async (execute_msg: RegistryExecuteMsg) => {
    const config = loadContractConfig();

    const msg : MsgExecuteContractParams<RegistryExecuteMsg> = {
        msg: execute_msg,
        sender: secretClient.address,
        contract_address: config.registry!.address,
        code_hash: config.registry!.hash,
        sent_funds: [],
    }

    const tx = await secretClient.tx.compute.executeContract(msg, { gasLimit: 900_000 });

    console.log("Execute registry tx: ", tx);

    return tx;
}


export const executeGatewayRegistryEncryptedInner = async (
    execute_msg: InnerMethods, 
    credentials: CosmosCredential[],
    key?: string
) => {

    key ??= await getGatewayRegistryEncryptionKey();

    /* 

    return executeGatewayRegistry({
        with_auth_data: {
            execute: execute_msg,
            auth_data: {
                credentials,
            }
        }
    }) */
}



