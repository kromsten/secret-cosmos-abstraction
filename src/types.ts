import { AminoMsg } from "@cosmjs/amino";
import { Permit } from "secretjs";

export type Code = {
    code_id     :   number;
    code_hash   :   string;
}


export type MultiContract = {
    code_id     :   number;
    code_hash   :   string;
    address?    :   string;
    migrate?    :   boolean;
}


export type CodeConfig = {
    gateway?     :   Code;
    snip20?      :   Code;
}

export type Contract = {
    address: string,
    hash:    string,
}


export type ContractConfig = {
    gateway?   :   Contract;
    sscrt?      :   Contract,
}


export type IbcConfig = {
    secret_channel_id : string;
    consumer_channel_id : string;
    ibc_denom : string;
}




export type TokenData = {
    sscrt_contract   :  Contract,
}



export type CosmosCredential = {    
    pubkey      :   string,
    signature   :   string,
    message     :   string,
    hrp?        :   string
}


export type CosmosAuthData = {
    credentials      :   CosmosCredential[],
    primary_index?   :   number
}


export type Expiration = 
    { at_height: number }  | 
    { at_time: string }    | 
    { never: {} };


export type SessionConfig = {
    generate_on_auth?   :   boolean,
    can_view?           :   boolean,
    expires?            :   Expiration
}





export type GatewaySimpleInitMsg = {
    admin?                   :       string
}




export type ExtendedMethods = 
    { store_secret: { text: string } }       



export type InnerQueries = 
    { get_secret: {} }   |
    { test: {} }        




export type GatewayExecuteMsg = 

    { reset_encryption_key: {} }         |

    { extension: { msg: ExtendedMethods } }           |
    
    { encrypted: { 
        payload: string, 
        payload_signature: string, 
        payload_hash: string,
        user_key: string,
        nonce: string 
    }} 
    

export type EncryptedPayload = {
    user_address: string,
    user_pubkey: string,
    hrp: string,
    msg: string
}
    


export type GatewayQueryMsg = 

    { encryption_key: {} }              |

    { with_permit: { 
        query: InnerQueries, 
        permit: Permit, 
        hrp?: string 
    }}                                  |

    { with_auth_data: { 
        query: InnerQueries, 
        auth_data: CosmosAuthData 
    }}                                  



   /*  
export type BankMsg = 
    {
        send: {
            amount: Coin[];
            to_address: string;
        };
    }                               |

    {
        burn: {
            amount: Coin[];
        };
    };


export type StakingMsg = 
  {
    delegate: {
      amount: Coin;
      validator: string;
    };
  }                         | 
  
  {
    undelegate: {
      amount: Coin;
      validator: string;
    };
  }                         | 
  
  {
    redelegate: {
      amount: Coin;
      dst_validator: string;
      src_validator: string;
    };
  };


export type DistributionMsg = 
    {
        set_withdraw_address: {
            address: string;
        };
    }                                   | 
    
    {
        withdraw_delegator_reward: {
            validator: string;
        };
    };


export interface IbcTimeoutBlock {
    height: number;
    revision: number;
}

export interface IbcTimeout {
    block?: IbcTimeoutBlock;
    timestamp?:     string;
}


export type BasicAllowance = {
    spend_limit  : Coin[];
    expiration?  : string;
}



export type IbcMsg = 
    {
        transfer: {
            amount: Coin;
            channel_id: string;
            timeout: IbcTimeout;
            to_address: string;
            [k: string]: unknown;
        };
    }                                   | 
    
    {
        send_packet: {
            channel_id: string;
            data: string;
            timeout: IbcTimeout;
            [k: string]: unknown;
        };
    }                                   | 
    {
        close_channel: {
            channel_id: string;
            [k: string]: unknown;
        };
    };


export type VoteOption = "yes" | "no" | "abstain" | "no_with_veto";


export type GovMsg = {
    vote: {
        proposal_id: number;
        vote: VoteOption;
    };
};


export type WasmMsg = 
    {
        execute: {
            contract_addr: string;
            funds: Coin[];
            msg: string;
        };
    }                                  | 
    {
        instantiate: {
            admin?: string;
            code_id: number;
            funds: Coin[];
            label: string;
            msg: string;
        };
    }                                  | 
    {
        migrate: {
            contract_addr: string;
            msg: string;
            new_code_id: number;
        };
    }                                  | 
    {
        update_admin: {
            admin: string;
            contract_addr: string;
        };
    }                                  | 
    {
        clear_admin: {
            contract_addr: string;
        };
    };


export type CosmosMsg = 
    { bank: BankMsg }           |
    { staking: StakingMsg }     |
    { distribution: DistributionMsg } |
    { stargate: { type_url: string, value: string }} |
    { ibc: IbcMsg }             |
    { wasm: WasmMsg }           |
    { gov: GovMsg };
 */



export type GatewayMsg<P = any> = {
    signer: string,
    payload: P,
    payload_hash: string,
    payload_signature: string,
    nonce: string,
}


export interface MsgSignData extends AminoMsg {
    readonly type: "sign/MsgSignData";
    readonly value: {
      /** Bech32 account address */
      signer: string;
      /** data to sign */
      data: string;
    };
  }