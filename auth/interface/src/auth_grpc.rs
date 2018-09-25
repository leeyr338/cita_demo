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

pub trait Auth {
    fn add_unverify_tx(&self, o: ::grpc::RequestOptions, p: super::auth::AddUnverifyTxReq) -> ::grpc::SingleResponse<super::auth::AddUnverifyTxRes>;

    fn verify_batch_txs(&self, o: ::grpc::RequestOptions, p: super::auth::VerifyBatchTxsReq) -> ::grpc::SingleResponse<super::auth::RpcStatus>;

    fn get_txs_hashes(&self, o: ::grpc::RequestOptions, p: super::auth::GetTxsHashesReq) -> ::grpc::SingleResponse<super::auth::GetTxsHashesRes>;

    fn store_txs(&self, o: ::grpc::RequestOptions, p: super::auth::StoreTxsReq) -> ::grpc::SingleResponse<super::auth::StoreTxsRes>;

    fn clean_txs_pool(&self, o: ::grpc::RequestOptions, p: super::auth::CleanTxsPoolReq) -> ::grpc::SingleResponse<super::auth::RpcStatus>;
}

// client

pub struct AuthClient {
    grpc_client: ::grpc::Client,
    method_AddUnverifyTx: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::AddUnverifyTxReq, super::auth::AddUnverifyTxRes>>,
    method_VerifyBatchTxs: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::VerifyBatchTxsReq, super::auth::RpcStatus>>,
    method_GetTxsHashes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::GetTxsHashesReq, super::auth::GetTxsHashesRes>>,
    method_StoreTxs: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::StoreTxsReq, super::auth::StoreTxsRes>>,
    method_CleanTxsPool: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::CleanTxsPoolReq, super::auth::RpcStatus>>,
}

impl AuthClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        AuthClient {
            grpc_client: grpc_client,
            method_AddUnverifyTx: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/AddUnverifyTx".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_VerifyBatchTxs: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/VerifyBatchTxs".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetTxsHashes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/GetTxsHashes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_StoreTxs: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/StoreTxs".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CleanTxsPool: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/CleanTxsPool".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            AuthClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            AuthClient::with_client(c)
        })
    }
}

impl Auth for AuthClient {
    fn add_unverify_tx(&self, o: ::grpc::RequestOptions, p: super::auth::AddUnverifyTxReq) -> ::grpc::SingleResponse<super::auth::AddUnverifyTxRes> {
        self.grpc_client.call_unary(o, p, self.method_AddUnverifyTx.clone())
    }

    fn verify_batch_txs(&self, o: ::grpc::RequestOptions, p: super::auth::VerifyBatchTxsReq) -> ::grpc::SingleResponse<super::auth::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_VerifyBatchTxs.clone())
    }

    fn get_txs_hashes(&self, o: ::grpc::RequestOptions, p: super::auth::GetTxsHashesReq) -> ::grpc::SingleResponse<super::auth::GetTxsHashesRes> {
        self.grpc_client.call_unary(o, p, self.method_GetTxsHashes.clone())
    }

    fn store_txs(&self, o: ::grpc::RequestOptions, p: super::auth::StoreTxsReq) -> ::grpc::SingleResponse<super::auth::StoreTxsRes> {
        self.grpc_client.call_unary(o, p, self.method_StoreTxs.clone())
    }

    fn clean_txs_pool(&self, o: ::grpc::RequestOptions, p: super::auth::CleanTxsPoolReq) -> ::grpc::SingleResponse<super::auth::RpcStatus> {
        self.grpc_client.call_unary(o, p, self.method_CleanTxsPool.clone())
    }
}

// server

pub struct AuthServer;


impl AuthServer {
    pub fn new_service_def<H : Auth + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Auth",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/AddUnverifyTx".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_unverify_tx(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/VerifyBatchTxs".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_batch_txs(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/GetTxsHashes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_txs_hashes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/StoreTxs".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.store_txs(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/CleanTxsPool".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.clean_txs_pool(o, p))
                    },
                ),
            ],
        )
    }
}
