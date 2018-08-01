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
    fn send_transaction(&self, o: ::grpc::RequestOptions, p: super::auth::SendTransactionReq) -> ::grpc::SingleResponse<super::auth::SendTransactionRes>;

    fn pack_transactions(&self, o: ::grpc::RequestOptions, p: super::auth::PackTransactionsReq) -> ::grpc::SingleResponse<super::auth::PackTransactionsRes>;
}

// client

pub struct AuthClient {
    grpc_client: ::grpc::Client,
    method_SendTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::SendTransactionReq, super::auth::SendTransactionRes>>,
    method_PackTransactions: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::PackTransactionsReq, super::auth::PackTransactionsRes>>,
}

impl AuthClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        AuthClient {
            grpc_client: grpc_client,
            method_SendTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/SendTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PackTransactions: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Auth/PackTransactions".to_string(),
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
    fn send_transaction(&self, o: ::grpc::RequestOptions, p: super::auth::SendTransactionReq) -> ::grpc::SingleResponse<super::auth::SendTransactionRes> {
        self.grpc_client.call_unary(o, p, self.method_SendTransaction.clone())
    }

    fn pack_transactions(&self, o: ::grpc::RequestOptions, p: super::auth::PackTransactionsReq) -> ::grpc::SingleResponse<super::auth::PackTransactionsRes> {
        self.grpc_client.call_unary(o, p, self.method_PackTransactions.clone())
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
                        name: "/Auth/SendTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_transaction(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Auth/PackTransactions".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.pack_transactions(o, p))
                    },
                ),
            ],
        )
    }
}
