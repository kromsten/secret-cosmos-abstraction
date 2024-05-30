import { MsgExecuteContractParams, MsgInstantiateContractParams, MsgInstantiateContractResponse, TxResultCode } from "secretjs";
import { AccountQuery, Contract, CosmosCredential, RegistryExecuteMsg, RegistryInitMsg, RegistryQueryMsg } from "./types";
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

    console.log("Create account tx: ", tx);

    return tx;
}


export const getAccountInfo = async (acc_query : AccountQuery) => {
    const config = loadContractConfig();

    const query : RegistryQueryMsg = {
        account_info: {
            query: acc_query
        }
    }    

    const res = await secretClient.query.compute.queryContract({
        contract_address: config.registry!.address,
        code_hash: config.registry!.hash,
        query
    });

    return res;
}


export const getAccountInfoWithAuth = async (acc_query : AccountQuery, credentials: CosmosCredential[]) => {
    const config = loadContractConfig();

    const query : RegistryQueryMsg = {
        with_auth_data: {
            query: acc_query,
            auth_data: {
                credentials,
            }
        }
    }    

    const res = await secretClient.query.compute.queryContract({
        contract_address: config.registry!.address,
        code_hash: config.registry!.hash,
        query
    });

    return res;
}







