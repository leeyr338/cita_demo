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

pub trait authority {
    fn auth(&self, o: ::grpc::RequestOptions, p: super::auth::Transation) -> ::grpc::SingleResponse<super::auth::State>;
}

// client

pub struct authorityClient {
    grpc_client: ::grpc::Client,
    method_auth: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::auth::Transation, super::auth::State>>,
}

impl authorityClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        authorityClient {
            grpc_client: grpc_client,
            method_auth: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/auth.authority/auth".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            authorityClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            authorityClient::with_client(c)
        })
    }
}

impl authority for authorityClient {
    fn auth(&self, o: ::grpc::RequestOptions, p: super::auth::Transation) -> ::grpc::SingleResponse<super::auth::State> {
        self.grpc_client.call_unary(o, p, self.method_auth.clone())
    }
}

// server

pub struct authorityServer;


impl authorityServer {
    pub fn new_service_def<H : authority + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/auth.authority",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/auth.authority/auth".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.auth(o, p))
                    },
                ),
            ],
        )
    }
}
