/// User is a single entry in the bucket authUsers
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(bytes, tag = "1")]
    pub name: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub password: std::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub roles: ::std::vec::Vec<std::string::String>,
}
/// Permission is a single entity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(enumeration = "permission::Type", tag = "1")]
    pub perm_type: i32,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub range_end: std::vec::Vec<u8>,
}
pub mod permission {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Read = 0,
        Write = 1,
        Readwrite = 2,
    }
}
/// Role is a single entry in the bucket authRoles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    #[prost(bytes, tag = "1")]
    pub name: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub key_permission: ::std::vec::Vec<Permission>,
}
