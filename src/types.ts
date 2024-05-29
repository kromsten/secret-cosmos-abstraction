import { Coin, Permit } from "secretjs";

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
    registry?    :   Code;
    account?     :   Code;
    snip20?      :   Code;
}

export type Contract = {
    address: string,
    hash:    string,
}


export type ContractConfig = {
    registry?   :   Contract;
    sscrt?      :   Contract,
    accounts    :   {
        [name : string] : Contract
    }
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


export type AbstractionParams = {
    session_config?                 :   SessionConfig,
    feegrant_address?               :   string,
    fee_grant_amount?               :   string,
    generate_signing_wallet?        :   boolean,
    signing_wallet_password?        :   string,
}




export type CosmosProxyMsg = {
    abstraction_params          :       AbstractionParams,
    auth_data                   :       CosmosAuthData,
}



export type CreateAccountMsg = {
    code_id         :       number,
    chain_id        :       string,
    code_hash?      :       string,
    padding?        :       string,
    gas_target?     :       number,
    label?          :       string
    msg             :       CosmosProxyMsg
}


export type AccountInitMsg = CosmosProxyMsg


export type RegistryInitMsg = {
    allowed_code_ids         :       number[],
    admin?                   :       string
}




export type AccountQuery = {
    address?        :   string,
    account_id?     :   string,
    credential_id?  :   string,
}


export type RegistryExecuteMsg = 

    { create_account: CreateAccountMsg } |

    { reset_encryption_key: {} }         |

    { extension: { msg: {} } }           |
    
    { encrypted: { msg: string, public_key: string, nonce: string, payload: string } } 
    


export type RegistryQueryMsg = 

    { encryption_key: {} }              |

    { allowed_code_ids: {} }            |

    { account_info: { 
        query: AccountQuery 
    }}                                  |

    { with_permit: { 
        query: AccountQuery, 
        permit: Permit, 
        hrp?: string 
    }}                                  |

    { with_auth_data: { 
        query: AccountQuery, 
        auth_data: CosmosAuthData 
    }}                                  |

    { with_session_key: { 
        query: AccountQuery, 
        session_key: string 
    }}                                  
   

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


export type CosmosAuthMsg =
    { with_auth_data: {
        auth_data: CosmosAuthData,
        msgs: CosmosMsg[]
    }}                                  |

    { with_session_key: {
        key: string,
        msgs: CosmosMsg[]
    }}                                  |

    { encrypted: {
        public_key: string,
        msg: string,
        nonce: string,
        payload: string
    }}                                  

export type CustomCosmosMsg = CosmosMsg | { custom: CosmosAuthMsg }


export type AccountMsg = 
    { execute: { 
        msgs: CosmosMsg[] 
    }}                                      |   

    { fee_grant: {
        grantee: string,
        allowance: BasicAllowance
    }}                                      |

    { reset_fee_grant_wallet: {
        password?: string
    }}                                      |

    { reset_encryption_key: {}}             |

    { create_viewing_key: {
        entropy: string
    }}                                      |

    { set_viewing_key: {
        key: string
    }}                                      |

    { create_proxy_viewing_key: {
        contract: Contract,
        entropy: string
    }}                                      |

    { set_proxy_viewing_key: {
        contract: Contract,
        key: string
    }}                                      



export type AccountExecuteMsg = 

    { with_auth_data: {
        auth_data   :   CosmosAuthData,
        msg         :   AccountMsg,
        padding?    :   string,
        gas_target? :   string
    }}                                      |

    { with_session_key: {
        msg         :   AccountMsg,
        session_key :   string,
        padding?    :   string,
        gas_target? :   string
    }}                                      |

    { execute: { 
        msgs        : CustomCosmosMsg[], 
        gas_target? : string 
    }}                                      |

    { encrypted: { 
        msg         :   string,
        public_key  :   string,
        nonce       :   string,
        payload     :   string
    }}                                      | 

    { evaporate: { gas_target: string } }


export type Balance = {
    address: string;
    amount: string;
}

export type Snip20Config = {
    public_total_supply : boolean;
    enable_deposit      : boolean;
    enable_redeem       : boolean;
    enable_mint         : boolean;
    enable_burn         : boolean;
    can_modify_denoms   : boolean;
}


export type Snip20InitMsg = {
    name                :   string;
    symbol              :   string;
    decimals            :   number;
    prng_seed           :   string;
    initial_balances?   :   Balance[];
    config?             :   Snip20Config;
    admin?              :   string;
    supported_denoms?   :   string[];
}