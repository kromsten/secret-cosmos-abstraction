use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use super::json::SerializableJson;

/// ## CallAction
/// This structure describes the fields for call action object structure
#[cw_serde]
#[derive(Eq, PartialOrd, Ord)]
pub enum CallAction {
    /// ## Description
    /// Queries bank module contract's balance and replaces received value in the message
    NativeBalanceFetch {
        /// coin denom to query
        denom: String,
        /// path to a field in the message for replacement
        replacer: String,
    },
    /// ## Description
    /// Queries cw20 token contract's balance and replaces received value in the message
    Cw20BalanceFetch {
        /// cw20 contract address
        contract: String,
        /// path to a field in the message for replacement
        replacer: String,
    },
    /// ## Description
    /// Makes a custom query and replaces msg values using data from the query response
    /// Both [`CallAction::NativeBalanceFetch`] & [`CallAction::Cw20BalanceFetch`] can be done via this call action type
    CustomReplaceQuery {
        /// valid json message of type [`cosmwasm_std::QueryRequest`]
        query_msg: SerializableJson,
        /// list of replacer paths
        replacers: Vec<ReplaceInfo>,
    },
    /// ## Description
    /// Enables ibc tracking for sent ibc transfer messages from the multicall contract
    IbcTracking {
        /// ibc channel
        channel: String,
        /// send denom
        denom: String,
        /// send amount, either amount or replacer must be set
        amount: Option<Uint128>,
        /// path to amount field in the message for replacement
        amount_pointer: Option<String>,
    },
    /// ## Description
    /// Converts specified field into [`Binary`] type
    FieldToBinary {
        /// path to a field in the message for replacement
        replacer: String,
    },
    /// ## Description
    /// Converts specified field into [`Binary`] type encoded using [`prost::Message::encode`] method
    FieldToProtoBinary {
        /// path to a field in the message for replacement
        replacer: String,
        /// Protobuf message type
        proto_msg_type: ProtoMessageType,
    },
}

#[cw_serde]
#[derive(Eq, PartialOrd, Ord)]
pub struct ReplaceInfo {
    /// path to a field in the query response struct to retrieve
    pub response_pointer: String,
    /// path to a field in the message for replacement
    pub replacer: String,
}

#[cw_serde]
#[derive(Eq, PartialOrd, Ord)]
pub enum ProtoMessageType {
    /// ibc message type
    IbcTransfer,
    /// osmosis gamm swap exact amount in type
    OsmosisSwapExactAmtIn,
}