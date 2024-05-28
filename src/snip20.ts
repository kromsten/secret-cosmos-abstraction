import { MsgInstantiateContractParams, MsgInstantiateContractResponse, TxResultCode } from "secretjs"
import { secretClient } from "./clients"
import { loadCodeConfig } from "./config"
import { Snip20InitMsg } from "./types"


export const instantiateSnip20 = async (
    name: string,
    symbol: string,
    deposit_denom: string
) => {
    const code = loadCodeConfig().snip20!;

    const init_msg : Snip20InitMsg = {
        name,
        symbol,
        decimals: 6,
        prng_seed: "YWE",
        initial_balances: [
            {
                address: secretClient.address,
                amount: "1000000000"
            }
        ],
        config: {
            public_total_supply: true,
            enable_deposit: true,
            enable_redeem: true,
            enable_mint: true,
            enable_burn: true,
            can_modify_denoms: true
        },
        supported_denoms: [deposit_denom] 
    }

    const msg : MsgInstantiateContractParams = {
        code_id: code.code_id,
        code_hash: code.code_hash,
        sender: secretClient.address,
        init_funds: [],
        admin: secretClient.address,
        label: `test-${Math.round(Date.now() / 1000)}`,
        init_msg
    }

    console.log("Instantiating snip20 contract msg: ", msg);
    const tx = await secretClient.tx.compute.instantiateContract(msg, { 
        gasLimit: 300_000
    });

    if (tx.code !==  TxResultCode.Success) {
        throw new Error(`Error while instantiating contract: ${tx.rawLog}`);
    }

    const address = MsgInstantiateContractResponse.decode(tx.data[0]).address;

    return { address, hash: code.code_hash }
}
