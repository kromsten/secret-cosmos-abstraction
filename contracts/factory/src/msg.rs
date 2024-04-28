use cosmwasm_schema::cw_serde;
use secret_toolkit::permit::Permit;

use cosmwasm_std::Addr;

use crate::structs::{CodeInfo, StoreOffspringInfo};

#[cw_serde]
pub struct InstantiateMsg {
    /// offspring code info
    pub offspring_code_info: CodeInfo,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// CreateOffspring will instantiate a new offspring contract
    CreateOffspring {
        /// String used to label when instantiating offspring contract.
        label: String,
        //  the rest are meant to be contract specific data
        /// address of the owner associated to this offspring contract
        owner: String,
        /// the count for the counter offspring template
        count: i32,
        #[serde(default)]
        description: Option<String>,
    },

    /// Allows the admin to add a new offspring contract version
    NewOffspringContract { offspring_code_info: CodeInfo },

    /// Create a viewing key to be used with all factory and offspring authenticated queries
    CreateViewingKey { entropy: String },

    /// Set a viewing key to be used with all factory and offspring authenticated queries
    SetViewingKey {
        key: String,
        // optional padding can be used so message length doesn't betray key length
        padding: Option<String>,
    },

    /// Allows an admin to start/stop all offspring creation
    SetStatus { stop: bool },

    /// disallow the use of a permit
    RevokePermit {
        /// name of the permit that is no longer valid
        permit_name: String,
        /// optional message length padding
        padding: Option<String>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    /// lists all offspring whose owner is the given address.
    ListMyOffspring {
        /// permit used to validate the querier. Disregarded if viewing key - address pair is provided.
        permit: Option<Permit>,
        /// address whose activity to display
        address: Option<String>,
        /// viewing key
        viewing_key: Option<String>,
        /// optional filter for only active or inactive offspring.  If not specified, lists all
        #[serde(default)]
        filter: Option<FilterTypes>,
        /// start page for the offsprings returned and listed (applies to both active and inactive). Default: 0
        #[serde(default)]
        start_page: Option<u32>,
        /// optional number of offspring to return in this page (applies to both active and inactive). Default: DEFAULT_PAGE_SIZE
        #[serde(default)]
        page_size: Option<u32>,
    },
    /// lists all active offspring in reverse chronological order
    ListActiveOffspring {
        /// start page for the offsprings returned and listed. Default: 0
        #[serde(default)]
        start_page: Option<u32>,
        /// optional number of offspring to return in this page. Default: DEFAULT_PAGE_SIZE
        #[serde(default)]
        page_size: Option<u32>,
    },
    /// lists inactive offspring in reverse chronological order.
    ListInactiveOffspring {
        /// start page for the offsprings returned and listed. Default: 0
        #[serde(default)]
        start_page: Option<u32>,
        /// optional number of offspring to return in this page. Default: DEFAULT_PAGE_SIZE
        #[serde(default)]
        page_size: Option<u32>,
    },
    /// authenticates the supplied address/viewing key. This should be called by offspring.
    IsKeyValid {
        /// address whose viewing key is being authenticated
        address: String,
        /// viewing key
        viewing_key: String,
    },
    /// authenticates the supplied permit. This should be called by offspring.
    IsPermitValid { permit: Permit },
}

#[cw_serde]
pub enum FilterTypes {
    Active,
    Inactive,
    All,
}

#[cw_serde]
pub enum QueryAnswer {
    /// List the offspring where address is associated.
    ListMyOffspring {
        /// lists of the address' active offspring
        #[serde(skip_serializing_if = "Option::is_none")]
        active: Option<Vec<StoreOffspringInfo>>,
        /// lists of the address' inactive offspring
        #[serde(skip_serializing_if = "Option::is_none")]
        inactive: Option<Vec<StoreOffspringInfo>>,
    },
    /// List active offspring
    ListActiveOffspring {
        /// active offspring
        active: Vec<StoreOffspringInfo>,
    },
    /// List inactive offspring in no particular order
    ListInactiveOffspring {
        /// inactive offspring in no particular order
        inactive: Vec<StoreOffspringInfo>,
    },
    /// Viewing Key Error
    ViewingKeyError { error: String },
    /// result of authenticating address/key pair
    IsKeyValid { is_valid: bool },
    /// result of authenticating a permit
    IsPermitValid {
        is_valid: bool,
        /// address of the permit signer if the permit was valid
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<Addr>,
    },
}

/// success or failure response
#[cw_serde]
pub enum ResponseStatus {
    Success,
    Failure,
}

/// Responses from handle functions
#[cw_serde]
pub enum HandleAnswer {
    /// generic status response
    Status {
        /// success or failure
        status: ResponseStatus,
        /// execution description
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}
