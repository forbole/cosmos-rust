/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
}
/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<BaseAccount>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the auth module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub max_memo_characters: u64,
    #[prost(uint64, tag = "2")]
    pub tx_sig_limit: u64,
    #[prost(uint64, tag = "3")]
    pub tx_size_cost_per_byte: u64,
    #[prost(uint64, tag = "4")]
    pub sig_verify_cost_ed25519: u64,
    #[prost(uint64, tag = "5")]
    pub sig_verify_cost_secp256k1: u64,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<::prost_types::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
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
        #[doc = " Account returns account details based on address."]
        pub async fn account(
            &self,
            request: super::QueryAccountRequest,
        ) -> Result<super::QueryAccountResponse, Box<dyn std::error::Error>> {
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
                    &self.host, "/cosmos.auth.v1beta1.Query/Account"
                ))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;
            let buffer = base64::decode(resp)?;
            let s = super::QueryAccountResponse::decode(&buffer[5..])?;
            Ok(s)
        }
        #[doc = " Params queries all parameters."]
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
                    &self.host, "/cosmos.auth.v1beta1.Query/Params"
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
