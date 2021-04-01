/// Params defines the parameters for the bank module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
    #[prost(bool, tag = "2")]
    pub default_send_enabled: bool,
}
/// SendEnabled maps coin denom to a send_enabled status (whether a denom is
/// sendable).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEnabled {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Input models transaction input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Output models transaction outputs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Supply represents a struct that passively keeps track of the total supply
/// amounts in the network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Supply {
    #[prost(message, repeated, tag = "1")]
    pub total: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DenomUnit represents a struct that describes a given
/// denomination unit of the basic token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomUnit {
    /// denom represents the string name of the given denom unit (e.g uatom).
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 1^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    #[prost(uint32, tag = "2")]
    pub exponent: u32,
    /// aliases is a list of string aliases for the given denom
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata represents a struct that describes
/// a basic token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// denom_units represents the list of DenomUnit's for a given coin
    #[prost(message, repeated, tag = "2")]
    pub denom_units: ::prost::alloc::vec::Vec<DenomUnit>,
    /// base represents the base denom (should be the DenomUnit with exponent = 0).
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    /// display indicates the suggested denom that should be
    /// displayed in clients.
    #[prost(string, tag = "4")]
    pub display: ::prost::alloc::string::String,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the coin.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryBalanceRequest is the request type for the Query/AllBalances RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesResponse {
    /// balances is the balances of all the coins.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryTotalSupplyRequest is the request type for the Query/TotalSupply RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSupplyRequest {}
/// QueryTotalSupplyResponse is the response type for the Query/TotalSupply RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalSupplyResponse {
    /// supply is the supply of the coins
    #[prost(message, repeated, tag = "1")]
    pub supply: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// QuerySupplyOfRequest is the request type for the Query/SupplyOf RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyOfRequest {
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyOfResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryParamsRequest defines the request type for querying x/bank parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/bank parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
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
        #[doc = " Balance queries the balance of a single coin for a single account."]
        pub async fn balance(
            &self,
            request: super::QueryBalanceRequest,
        ) -> Result<super::QueryBalanceResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.bank.v1beta1.Query/Balance"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryBalanceResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " AllBalances queries the balance of all coins for a single account."]
        pub async fn all_balances(
            &self,
            request: super::QueryAllBalancesRequest,
        ) -> Result<super::QueryAllBalancesResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.bank.v1beta1.Query/AllBalances"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryAllBalancesResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " TotalSupply queries the total supply of all coins."]
        pub async fn total_supply(
            &self,
            request: super::QueryTotalSupplyRequest,
        ) -> Result<super::QueryTotalSupplyResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.bank.v1beta1.Query/TotalSupply"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryTotalSupplyResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " SupplyOf queries the supply of a single coin."]
        pub async fn supply_of(
            &self,
            request: super::QuerySupplyOfRequest,
        ) -> Result<super::QuerySupplyOfResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.bank.v1beta1.Query/SupplyOf"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QuerySupplyOfResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Params queries the parameters of x/bank module."]
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
                    &self.host, "/cosmos.bank.v1beta1.Query/Params"
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
/// MsgSend represents a message to send coins from one account to another.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgSendResponse defines the Msg/Send response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSend {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
/// MsgMultiSendResponse defines the Msg/MultiSend response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSendResponse {}
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use base64;
    use prost::Message;
    pub struct Msg {
        host: String,
    }
    impl Msg {
        #[doc = " Send defines a method for sending coins from one account to another account."]
        pub async fn send(
            &self,
            request: super::MsgSend,
        ) -> Result<super::MsgSendResponse, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("{}{}", &self.host, "/cosmos.bank.v1beta1.Msg/Send"))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::MsgSendResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " MultiSend defines a method for sending coins from some accounts to other accounts."]
        pub async fn multi_send(
            &self,
            request: super::MsgMultiSend,
        ) -> Result<super::MsgMultiSendResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.bank.v1beta1.Msg/MultiSend"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::MsgMultiSendResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        pub fn new(host: String) -> Msg {
            Msg { host }
        }
    }
}
