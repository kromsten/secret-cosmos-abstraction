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