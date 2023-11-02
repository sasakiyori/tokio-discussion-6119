// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRequest {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailResponse {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
}
include!("xx.tonic.rs");
// @@protoc_insertion_point(module)
