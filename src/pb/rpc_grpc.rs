// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait KV {
    fn range(&self, o: ::grpc::RequestOptions, p: super::rpc::RangeRequest) -> ::grpc::SingleResponse<super::rpc::RangeResponse>;

    fn put(&self, o: ::grpc::RequestOptions, p: super::rpc::PutRequest) -> ::grpc::SingleResponse<super::rpc::PutResponse>;

    fn delete_range(&self, o: ::grpc::RequestOptions, p: super::rpc::DeleteRangeRequest) -> ::grpc::SingleResponse<super::rpc::DeleteRangeResponse>;

    fn txn(&self, o: ::grpc::RequestOptions, p: super::rpc::TxnRequest) -> ::grpc::SingleResponse<super::rpc::TxnResponse>;

    fn compact(&self, o: ::grpc::RequestOptions, p: super::rpc::CompactionRequest) -> ::grpc::SingleResponse<super::rpc::CompactionResponse>;
}

// client

pub struct KVClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Range: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::RangeRequest, super::rpc::RangeResponse>>,
    method_Put: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::PutRequest, super::rpc::PutResponse>>,
    method_DeleteRange: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::DeleteRangeRequest, super::rpc::DeleteRangeResponse>>,
    method_Txn: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::TxnRequest, super::rpc::TxnResponse>>,
    method_Compact: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::CompactionRequest, super::rpc::CompactionResponse>>,
}

impl ::grpc::ClientStub for KVClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        KVClient {
            grpc_client: grpc_client,
            method_Range: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.KV/Range".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Put: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.KV/Put".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteRange: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.KV/DeleteRange".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Txn: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.KV/Txn".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Compact: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.KV/Compact".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl KV for KVClient {
    fn range(&self, o: ::grpc::RequestOptions, p: super::rpc::RangeRequest) -> ::grpc::SingleResponse<super::rpc::RangeResponse> {
        self.grpc_client.call_unary(o, p, self.method_Range.clone())
    }

    fn put(&self, o: ::grpc::RequestOptions, p: super::rpc::PutRequest) -> ::grpc::SingleResponse<super::rpc::PutResponse> {
        self.grpc_client.call_unary(o, p, self.method_Put.clone())
    }

    fn delete_range(&self, o: ::grpc::RequestOptions, p: super::rpc::DeleteRangeRequest) -> ::grpc::SingleResponse<super::rpc::DeleteRangeResponse> {
        self.grpc_client.call_unary(o, p, self.method_DeleteRange.clone())
    }

    fn txn(&self, o: ::grpc::RequestOptions, p: super::rpc::TxnRequest) -> ::grpc::SingleResponse<super::rpc::TxnResponse> {
        self.grpc_client.call_unary(o, p, self.method_Txn.clone())
    }

    fn compact(&self, o: ::grpc::RequestOptions, p: super::rpc::CompactionRequest) -> ::grpc::SingleResponse<super::rpc::CompactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_Compact.clone())
    }
}

// server

pub struct KVServer;


impl KVServer {
    pub fn new_service_def<H : KV + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/etcdserverpb.KV",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.KV/Range".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.range(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.KV/Put".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.put(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.KV/DeleteRange".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_range(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.KV/Txn".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.txn(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.KV/Compact".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.compact(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Lease {
    fn lease_grant(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseGrantRequest) -> ::grpc::SingleResponse<super::rpc::LeaseGrantResponse>;

    fn lease_revoke(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseRevokeRequest) -> ::grpc::SingleResponse<super::rpc::LeaseRevokeResponse>;

    fn lease_keep_alive(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::StreamingResponse<super::rpc::LeaseKeepAliveResponse>;

    fn lease_time_to_live(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::SingleResponse<super::rpc::LeaseTimeToLiveResponse>;

    fn lease_leases(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseLeasesRequest) -> ::grpc::SingleResponse<super::rpc::LeaseLeasesResponse>;
}

// client

pub struct LeaseClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_LeaseGrant: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::LeaseGrantRequest, super::rpc::LeaseGrantResponse>>,
    method_LeaseRevoke: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::LeaseRevokeRequest, super::rpc::LeaseRevokeResponse>>,
    method_LeaseKeepAlive: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::LeaseKeepAliveRequest, super::rpc::LeaseKeepAliveResponse>>,
    method_LeaseTimeToLive: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::LeaseTimeToLiveRequest, super::rpc::LeaseTimeToLiveResponse>>,
    method_LeaseLeases: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::LeaseLeasesRequest, super::rpc::LeaseLeasesResponse>>,
}

impl ::grpc::ClientStub for LeaseClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LeaseClient {
            grpc_client: grpc_client,
            method_LeaseGrant: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Lease/LeaseGrant".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LeaseRevoke: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Lease/LeaseRevoke".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LeaseKeepAlive: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Lease/LeaseKeepAlive".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LeaseTimeToLive: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Lease/LeaseTimeToLive".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LeaseLeases: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Lease/LeaseLeases".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Lease for LeaseClient {
    fn lease_grant(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseGrantRequest) -> ::grpc::SingleResponse<super::rpc::LeaseGrantResponse> {
        self.grpc_client.call_unary(o, p, self.method_LeaseGrant.clone())
    }

    fn lease_revoke(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseRevokeRequest) -> ::grpc::SingleResponse<super::rpc::LeaseRevokeResponse> {
        self.grpc_client.call_unary(o, p, self.method_LeaseRevoke.clone())
    }

    fn lease_keep_alive(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::LeaseKeepAliveRequest>) -> ::grpc::StreamingResponse<super::rpc::LeaseKeepAliveResponse> {
        self.grpc_client.call_bidi(o, p, self.method_LeaseKeepAlive.clone())
    }

    fn lease_time_to_live(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseTimeToLiveRequest) -> ::grpc::SingleResponse<super::rpc::LeaseTimeToLiveResponse> {
        self.grpc_client.call_unary(o, p, self.method_LeaseTimeToLive.clone())
    }

    fn lease_leases(&self, o: ::grpc::RequestOptions, p: super::rpc::LeaseLeasesRequest) -> ::grpc::SingleResponse<super::rpc::LeaseLeasesResponse> {
        self.grpc_client.call_unary(o, p, self.method_LeaseLeases.clone())
    }
}

// server

pub struct LeaseServer;


impl LeaseServer {
    pub fn new_service_def<H : Lease + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/etcdserverpb.Lease",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseGrant".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lease_grant(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseRevoke".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lease_revoke(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseKeepAlive".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.lease_keep_alive(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseTimeToLive".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lease_time_to_live(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Lease/LeaseLeases".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lease_leases(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Cluster {
    fn member_add(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberAddRequest) -> ::grpc::SingleResponse<super::rpc::MemberAddResponse>;

    fn member_remove(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberRemoveRequest) -> ::grpc::SingleResponse<super::rpc::MemberRemoveResponse>;

    fn member_update(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberUpdateRequest) -> ::grpc::SingleResponse<super::rpc::MemberUpdateResponse>;

    fn member_list(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberListRequest) -> ::grpc::SingleResponse<super::rpc::MemberListResponse>;
}

// client

pub struct ClusterClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_MemberAdd: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::MemberAddRequest, super::rpc::MemberAddResponse>>,
    method_MemberRemove: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::MemberRemoveRequest, super::rpc::MemberRemoveResponse>>,
    method_MemberUpdate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::MemberUpdateRequest, super::rpc::MemberUpdateResponse>>,
    method_MemberList: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::MemberListRequest, super::rpc::MemberListResponse>>,
}

impl ::grpc::ClientStub for ClusterClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ClusterClient {
            grpc_client: grpc_client,
            method_MemberAdd: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Cluster/MemberAdd".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_MemberRemove: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Cluster/MemberRemove".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_MemberUpdate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Cluster/MemberUpdate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_MemberList: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Cluster/MemberList".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Cluster for ClusterClient {
    fn member_add(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberAddRequest) -> ::grpc::SingleResponse<super::rpc::MemberAddResponse> {
        self.grpc_client.call_unary(o, p, self.method_MemberAdd.clone())
    }

    fn member_remove(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberRemoveRequest) -> ::grpc::SingleResponse<super::rpc::MemberRemoveResponse> {
        self.grpc_client.call_unary(o, p, self.method_MemberRemove.clone())
    }

    fn member_update(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberUpdateRequest) -> ::grpc::SingleResponse<super::rpc::MemberUpdateResponse> {
        self.grpc_client.call_unary(o, p, self.method_MemberUpdate.clone())
    }

    fn member_list(&self, o: ::grpc::RequestOptions, p: super::rpc::MemberListRequest) -> ::grpc::SingleResponse<super::rpc::MemberListResponse> {
        self.grpc_client.call_unary(o, p, self.method_MemberList.clone())
    }
}

// server

pub struct ClusterServer;


impl ClusterServer {
    pub fn new_service_def<H : Cluster + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/etcdserverpb.Cluster",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberAdd".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.member_add(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberRemove".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.member_remove(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberUpdate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.member_update(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Cluster/MemberList".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.member_list(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Maintenance {
    fn alarm(&self, o: ::grpc::RequestOptions, p: super::rpc::AlarmRequest) -> ::grpc::SingleResponse<super::rpc::AlarmResponse>;

    fn status(&self, o: ::grpc::RequestOptions, p: super::rpc::StatusRequest) -> ::grpc::SingleResponse<super::rpc::StatusResponse>;

    fn defragment(&self, o: ::grpc::RequestOptions, p: super::rpc::DefragmentRequest) -> ::grpc::SingleResponse<super::rpc::DefragmentResponse>;

    fn hash(&self, o: ::grpc::RequestOptions, p: super::rpc::HashRequest) -> ::grpc::SingleResponse<super::rpc::HashResponse>;

    fn hash_kv(&self, o: ::grpc::RequestOptions, p: super::rpc::HashKVRequest) -> ::grpc::SingleResponse<super::rpc::HashKVResponse>;

    fn snapshot(&self, o: ::grpc::RequestOptions, p: super::rpc::SnapshotRequest) -> ::grpc::StreamingResponse<super::rpc::SnapshotResponse>;

    fn move_leader(&self, o: ::grpc::RequestOptions, p: super::rpc::MoveLeaderRequest) -> ::grpc::SingleResponse<super::rpc::MoveLeaderResponse>;
}

// client

pub struct MaintenanceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Alarm: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AlarmRequest, super::rpc::AlarmResponse>>,
    method_Status: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::StatusRequest, super::rpc::StatusResponse>>,
    method_Defragment: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::DefragmentRequest, super::rpc::DefragmentResponse>>,
    method_Hash: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::HashRequest, super::rpc::HashResponse>>,
    method_HashKV: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::HashKVRequest, super::rpc::HashKVResponse>>,
    method_Snapshot: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SnapshotRequest, super::rpc::SnapshotResponse>>,
    method_MoveLeader: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::MoveLeaderRequest, super::rpc::MoveLeaderResponse>>,
}

impl ::grpc::ClientStub for MaintenanceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MaintenanceClient {
            grpc_client: grpc_client,
            method_Alarm: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/Alarm".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Status: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/Status".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Defragment: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/Defragment".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Hash: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/Hash".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_HashKV: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/HashKV".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Snapshot: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/Snapshot".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_MoveLeader: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Maintenance/MoveLeader".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Maintenance for MaintenanceClient {
    fn alarm(&self, o: ::grpc::RequestOptions, p: super::rpc::AlarmRequest) -> ::grpc::SingleResponse<super::rpc::AlarmResponse> {
        self.grpc_client.call_unary(o, p, self.method_Alarm.clone())
    }

    fn status(&self, o: ::grpc::RequestOptions, p: super::rpc::StatusRequest) -> ::grpc::SingleResponse<super::rpc::StatusResponse> {
        self.grpc_client.call_unary(o, p, self.method_Status.clone())
    }

    fn defragment(&self, o: ::grpc::RequestOptions, p: super::rpc::DefragmentRequest) -> ::grpc::SingleResponse<super::rpc::DefragmentResponse> {
        self.grpc_client.call_unary(o, p, self.method_Defragment.clone())
    }

    fn hash(&self, o: ::grpc::RequestOptions, p: super::rpc::HashRequest) -> ::grpc::SingleResponse<super::rpc::HashResponse> {
        self.grpc_client.call_unary(o, p, self.method_Hash.clone())
    }

    fn hash_kv(&self, o: ::grpc::RequestOptions, p: super::rpc::HashKVRequest) -> ::grpc::SingleResponse<super::rpc::HashKVResponse> {
        self.grpc_client.call_unary(o, p, self.method_HashKV.clone())
    }

    fn snapshot(&self, o: ::grpc::RequestOptions, p: super::rpc::SnapshotRequest) -> ::grpc::StreamingResponse<super::rpc::SnapshotResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_Snapshot.clone())
    }

    fn move_leader(&self, o: ::grpc::RequestOptions, p: super::rpc::MoveLeaderRequest) -> ::grpc::SingleResponse<super::rpc::MoveLeaderResponse> {
        self.grpc_client.call_unary(o, p, self.method_MoveLeader.clone())
    }
}

// server

pub struct MaintenanceServer;


impl MaintenanceServer {
    pub fn new_service_def<H : Maintenance + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/etcdserverpb.Maintenance",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Alarm".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.alarm(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Status".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Defragment".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.defragment(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Hash".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.hash(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/HashKV".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.hash_kv(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/Snapshot".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.snapshot(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Maintenance/MoveLeader".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.move_leader(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Auth {
    fn auth_enable(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthEnableRequest) -> ::grpc::SingleResponse<super::rpc::AuthEnableResponse>;

    fn auth_disable(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthDisableRequest) -> ::grpc::SingleResponse<super::rpc::AuthDisableResponse>;

    fn authenticate(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthenticateRequest) -> ::grpc::SingleResponse<super::rpc::AuthenticateResponse>;

    fn user_add(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserAddRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserAddResponse>;

    fn user_get(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserGetRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserGetResponse>;

    fn user_list(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserListRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserListResponse>;

    fn user_delete(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserDeleteResponse>;

    fn user_change_password(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserChangePasswordResponse>;

    fn user_grant_role(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserGrantRoleResponse>;

    fn user_revoke_role(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserRevokeRoleResponse>;

    fn role_add(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleAddRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleAddResponse>;

    fn role_get(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleGetRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleGetResponse>;

    fn role_list(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleListRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleListResponse>;

    fn role_delete(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleDeleteResponse>;

    fn role_grant_permission(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleGrantPermissionResponse>;

    fn role_revoke_permission(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleRevokePermissionResponse>;
}

// client

pub struct AuthClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_AuthEnable: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthEnableRequest, super::rpc::AuthEnableResponse>>,
    method_AuthDisable: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthDisableRequest, super::rpc::AuthDisableResponse>>,
    method_Authenticate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthenticateRequest, super::rpc::AuthenticateResponse>>,
    method_UserAdd: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserAddRequest, super::rpc::AuthUserAddResponse>>,
    method_UserGet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserGetRequest, super::rpc::AuthUserGetResponse>>,
    method_UserList: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserListRequest, super::rpc::AuthUserListResponse>>,
    method_UserDelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserDeleteRequest, super::rpc::AuthUserDeleteResponse>>,
    method_UserChangePassword: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserChangePasswordRequest, super::rpc::AuthUserChangePasswordResponse>>,
    method_UserGrantRole: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserGrantRoleRequest, super::rpc::AuthUserGrantRoleResponse>>,
    method_UserRevokeRole: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthUserRevokeRoleRequest, super::rpc::AuthUserRevokeRoleResponse>>,
    method_RoleAdd: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleAddRequest, super::rpc::AuthRoleAddResponse>>,
    method_RoleGet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleGetRequest, super::rpc::AuthRoleGetResponse>>,
    method_RoleList: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleListRequest, super::rpc::AuthRoleListResponse>>,
    method_RoleDelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleDeleteRequest, super::rpc::AuthRoleDeleteResponse>>,
    method_RoleGrantPermission: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleGrantPermissionRequest, super::rpc::AuthRoleGrantPermissionResponse>>,
    method_RoleRevokePermission: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AuthRoleRevokePermissionRequest, super::rpc::AuthRoleRevokePermissionResponse>>,
}

impl ::grpc::ClientStub for AuthClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        AuthClient {
            grpc_client: grpc_client,
            method_AuthEnable: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/AuthEnable".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthDisable: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/AuthDisable".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Authenticate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/Authenticate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserAdd: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserAdd".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserGet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserGet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserList: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserList".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserDelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserDelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserChangePassword: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserChangePassword".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserGrantRole: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserGrantRole".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UserRevokeRole: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/UserRevokeRole".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleAdd: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleAdd".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleGet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleGet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleList: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleList".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleDelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleDelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleGrantPermission: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleGrantPermission".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RoleRevokePermission: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/etcdserverpb.Auth/RoleRevokePermission".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Auth for AuthClient {
    fn auth_enable(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthEnableRequest) -> ::grpc::SingleResponse<super::rpc::AuthEnableResponse> {
        self.grpc_client.call_unary(o, p, self.method_AuthEnable.clone())
    }

    fn auth_disable(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthDisableRequest) -> ::grpc::SingleResponse<super::rpc::AuthDisableResponse> {
        self.grpc_client.call_unary(o, p, self.method_AuthDisable.clone())
    }

    fn authenticate(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthenticateRequest) -> ::grpc::SingleResponse<super::rpc::AuthenticateResponse> {
        self.grpc_client.call_unary(o, p, self.method_Authenticate.clone())
    }

    fn user_add(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserAddRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserAddResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserAdd.clone())
    }

    fn user_get(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserGetRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserGetResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserGet.clone())
    }

    fn user_list(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserListRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserListResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserList.clone())
    }

    fn user_delete(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserDeleteRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserDeleteResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserDelete.clone())
    }

    fn user_change_password(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserChangePasswordRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserChangePasswordResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserChangePassword.clone())
    }

    fn user_grant_role(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserGrantRoleRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserGrantRoleResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserGrantRole.clone())
    }

    fn user_revoke_role(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthUserRevokeRoleRequest) -> ::grpc::SingleResponse<super::rpc::AuthUserRevokeRoleResponse> {
        self.grpc_client.call_unary(o, p, self.method_UserRevokeRole.clone())
    }

    fn role_add(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleAddRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleAddResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleAdd.clone())
    }

    fn role_get(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleGetRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleGetResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleGet.clone())
    }

    fn role_list(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleListRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleListResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleList.clone())
    }

    fn role_delete(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleDeleteRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleDeleteResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleDelete.clone())
    }

    fn role_grant_permission(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleGrantPermissionRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleGrantPermissionResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleGrantPermission.clone())
    }

    fn role_revoke_permission(&self, o: ::grpc::RequestOptions, p: super::rpc::AuthRoleRevokePermissionRequest) -> ::grpc::SingleResponse<super::rpc::AuthRoleRevokePermissionResponse> {
        self.grpc_client.call_unary(o, p, self.method_RoleRevokePermission.clone())
    }
}

// server

pub struct AuthServer;


impl AuthServer {
    pub fn new_service_def<H : Auth + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/etcdserverpb.Auth",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/AuthEnable".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.auth_enable(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/AuthDisable".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.auth_disable(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/Authenticate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authenticate(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserAdd".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_add(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserGet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_get(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserList".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_list(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserDelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_delete(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserChangePassword".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_change_password(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserGrantRole".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_grant_role(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/UserRevokeRole".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.user_revoke_role(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleAdd".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_add(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleGet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_get(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleList".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_list(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleDelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_delete(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleGrantPermission".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_grant_permission(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/etcdserverpb.Auth/RoleRevokePermission".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.role_revoke_permission(o, p))
                    },
                ),
            ],
        )
    }
}
