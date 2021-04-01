/// HistoricalInfo contains header and validator information for a given block.
/// It is stored as part of staking module's state, which persists the `n` most
/// recent HistoricalInfo
/// (`n` is set by the staking module's `historical_entries` parameter).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalInfo {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<tendermint_proto::types::Header>,
    #[prost(message, repeated, tag = "2")]
    pub valset: ::prost::alloc::vec::Vec<Validator>,
}
/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommissionRates {
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub max_change_rate: ::prost::alloc::string::String,
}
/// Commission defines commission parameters for a given validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commission {
    #[prost(message, optional, tag = "1")]
    pub commission_rates: ::core::option::Option<CommissionRates>,
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Description defines a validator description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub consensus_pubkey: ::core::option::Option<::prost_types::Any>,
    #[prost(bool, tag = "3")]
    pub jailed: bool,
    #[prost(enumeration = "BondStatus", tag = "4")]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub delegator_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub description: ::core::option::Option<Description>,
    #[prost(int64, tag = "8")]
    pub unbonding_height: i64,
    #[prost(message, optional, tag = "9")]
    pub unbonding_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub commission: ::core::option::Option<Commission>,
    #[prost(string, tag = "11")]
    pub min_self_delegation: ::prost::alloc::string::String,
}
/// ValAddresses defines a repeated set of validator addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValAddresses {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DVPair is struct that just has a delegator-validator pair with no other data.
/// It is intended to be used as a marshalable pointer. For example, a DVPair can
/// be used to construct the key to getting an UnbondingDelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPair {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// DVPairs defines an array of DVPair objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<DvPair>,
}
/// DVVTriplet is struct that just has a delegator-validator-validator triplet
/// with no other data. It is intended to be used as a marshalable pointer. For
/// example, a DVVTriplet can be used to construct the key to getting a
/// Redelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplet {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
}
/// DVVTriplets defines an array of DVVTriplet objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplets {
    #[prost(message, repeated, tag = "1")]
    pub triplets: ::prost::alloc::vec::Vec<DvvTriplet>,
}
/// Delegation represents the bond with tokens held by an account. It is
/// owned by one delegator, and is associated with the voting power of one
/// validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
}
/// UnbondingDelegation stores all of a single delegator's unbonding bonds
/// for a single validator in an time-ordered list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// unbonding delegation entries
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<UnbondingDelegationEntry>,
}
/// UnbondingDelegationEntry defines an unbonding object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegationEntry {
    #[prost(int64, tag = "1")]
    pub creation_height: i64,
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationEntry defines a redelegation object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntry {
    #[prost(int64, tag = "1")]
    pub creation_height: i64,
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shares_dst: ::prost::alloc::string::String,
}
/// Redelegation contains the list of a particular delegator's redelegating bonds
/// from a particular source validator to a particular destination validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Redelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    /// redelegation entries
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntry>,
}
/// Params defines the parameters for the staking module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub unbonding_time: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint32, tag = "2")]
    pub max_validators: u32,
    #[prost(uint32, tag = "3")]
    pub max_entries: u32,
    #[prost(uint32, tag = "4")]
    pub historical_entries: u32,
    #[prost(string, tag = "5")]
    pub bond_denom: ::prost::alloc::string::String,
}
/// DelegationResponse is equivalent to Delegation except that it contains a
/// balance in addition to shares which is more suitable for client responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation: ::core::option::Option<Delegation>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// RedelegationEntryResponse is equivalent to a RedelegationEntry except that it
/// contains a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation_entry: ::core::option::Option<RedelegationEntry>,
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationResponse is equivalent to a Redelegation except that its entries
/// contain a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation: ::core::option::Option<Redelegation>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntryResponse>,
}
/// Pool is used for tracking bonded and not-bonded token supply of the bond
/// denomination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub not_bonded_tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bonded_tokens: ::prost::alloc::string::String,
}
/// BondStatus is the status of a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}
/// QueryValidatorsRequest is request type for Query/Validators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorsResponse is response type for the Query/Validators RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorRequest is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryValidatorResponse is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryValidatorDelegationsRequest is request type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorDelegationsResponse is response type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorUnbondingDelegationsRequest is required type for the
/// Query/ValidatorUnbondingDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorUnbondingDelegationsResponse is response type for the
/// Query/ValidatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRequest is request type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationResponse {
    /// delegation_responses defines the delegation info of a delegation.
    #[prost(message, optional, tag = "1")]
    pub delegation_response: ::core::option::Option<DelegationResponse>,
}
/// QueryUnbondingDelegationRequest is request type for the
/// Query/UnbondingDelegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/UnbondingDelegation
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationResponse {
    /// unbond defines the unbonding information of a delegation.
    #[prost(message, optional, tag = "1")]
    pub unbond: ::core::option::Option<UnbondingDelegation>,
}
/// QueryDelegatorDelegationsRequest is request type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorDelegationsResponse is response type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsResponse {
    /// delegation_responses defines all the delegations' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorUnbondingDelegationsRequest is request type for the
/// Query/DelegatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryUnbondingDelegatorDelegationsResponse is response type for the
/// Query/UnbondingDelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryRedelegationsRequest is request type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// src_validator_addr defines the validator address to redelegate from.
    #[prost(string, tag = "2")]
    pub src_validator_addr: ::prost::alloc::string::String,
    /// dst_validator_addr defines the validator address to redelegate to.
    #[prost(string, tag = "3")]
    pub dst_validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryRedelegationsResponse is response type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub redelegation_responses: ::prost::alloc::vec::Vec<RedelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorsRequest is request type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorValidatorsResponse is response type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the the validators' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorRequest is request type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegatorValidatorResponse response type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryHistoricalInfoRequest is request type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoRequest {
    /// height defines at which height to query the historical info.
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoResponse {
    /// hist defines the historical info at the given height.
    #[prost(message, optional, tag = "1")]
    pub hist: ::core::option::Option<HistoricalInfo>,
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    /// pool defines the pool info.
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use base64;
    use prost::Message;
    pub struct Query {
        host: String,
    }
    impl Query {
        #[doc = " Validators queries all validators that match the given status."]
        pub async fn validators(
            &self,
            request: super::QueryValidatorsRequest,
        ) -> Result<super::QueryValidatorsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Validators"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryValidatorsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Validator queries validator info for given validator address."]
        pub async fn validator(
            &self,
            request: super::QueryValidatorRequest,
        ) -> Result<super::QueryValidatorResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Validator"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryValidatorResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " ValidatorDelegations queries delegate info for given validator."]
        pub async fn validator_delegations(
            &self,
            request: super::QueryValidatorDelegationsRequest,
        ) -> Result<super::QueryValidatorDelegationsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/ValidatorDelegations"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryValidatorDelegationsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " ValidatorUnbondingDelegations queries unbonding delegations of a validator."]
        pub async fn validator_unbonding_delegations(
            &self,
            request: super::QueryValidatorUnbondingDelegationsRequest,
        ) -> Result<super::QueryValidatorUnbondingDelegationsResponse, Box<dyn std::error::Error>>
        {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/ValidatorUnbondingDelegations"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryValidatorUnbondingDelegationsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Delegation queries delegate info for given validator delegator pair."]
        pub async fn delegation(
            &self,
            request: super::QueryDelegationRequest,
        ) -> Result<super::QueryDelegationResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Delegation"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryDelegationResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " UnbondingDelegation queries unbonding info for given validator delegator"]
        #[doc = " pair."]
        pub async fn unbonding_delegation(
            &self,
            request: super::QueryUnbondingDelegationRequest,
        ) -> Result<super::QueryUnbondingDelegationResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/UnbondingDelegation"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryUnbondingDelegationResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " DelegatorDelegations queries all delegations of a given delegator address."]
        pub async fn delegator_delegations(
            &self,
            request: super::QueryDelegatorDelegationsRequest,
        ) -> Result<super::QueryDelegatorDelegationsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/DelegatorDelegations"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryDelegatorDelegationsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " DelegatorUnbondingDelegations queries all unbonding delegations of a given"]
        #[doc = " delegator address."]
        pub async fn delegator_unbonding_delegations(
            &self,
            request: super::QueryDelegatorUnbondingDelegationsRequest,
        ) -> Result<super::QueryDelegatorUnbondingDelegationsResponse, Box<dyn std::error::Error>>
        {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryDelegatorUnbondingDelegationsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Redelegations queries redelegations of given address."]
        pub async fn redelegations(
            &self,
            request: super::QueryRedelegationsRequest,
        ) -> Result<super::QueryRedelegationsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Redelegations"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryRedelegationsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " DelegatorValidators queries all validators info for given delegator"]
        #[doc = " address."]
        pub async fn delegator_validators(
            &self,
            request: super::QueryDelegatorValidatorsRequest,
        ) -> Result<super::QueryDelegatorValidatorsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/DelegatorValidators"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryDelegatorValidatorsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " DelegatorValidator queries validator info for given delegator validator"]
        #[doc = " pair."]
        pub async fn delegator_validator(
            &self,
            request: super::QueryDelegatorValidatorRequest,
        ) -> Result<super::QueryDelegatorValidatorResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/DelegatorValidator"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryDelegatorValidatorResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " HistoricalInfo queries the historical info for given height."]
        pub async fn historical_info(
            &self,
            request: super::QueryHistoricalInfoRequest,
        ) -> Result<super::QueryHistoricalInfoResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/HistoricalInfo"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryHistoricalInfoResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Pool queries the pool info."]
        pub async fn pool(
            &self,
            request: super::QueryPoolRequest,
        ) -> Result<super::QueryPoolResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Pool"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryPoolResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Parameters queries the staking parameters."]
        pub async fn params(
            &self,
            request: super::QueryParamsRequest,
        ) -> Result<super::QueryParamsResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}{}",
                    &self.host, "/cosmos.staking.v1beta1.Query/Params"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryParamsResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        pub fn new(host: String) -> Query {
            Query { host }
        }
    }
}
