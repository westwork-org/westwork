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

pub trait Bootstrap {
    fn SetName(&self, p: super::bootstrap::Name) -> ::grpc::result::GrpcResult<super::bootstrap::Empty>;

    fn SetUserName(&self, p: super::bootstrap::UserName) -> ::grpc::result::GrpcResult<super::bootstrap::Empty>;

    fn GetWifi(&self, p: super::bootstrap::Empty) -> ::grpc::result::GrpcResult<super::bootstrap::WifiList>;

    fn SetWifi(&self, p: super::bootstrap::Network) -> ::grpc::result::GrpcResult<super::bootstrap::Empty>;
}

pub trait BootstrapAsync {
    fn SetName(&self, p: super::bootstrap::Name) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty>;

    fn SetUserName(&self, p: super::bootstrap::UserName) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty>;

    fn GetWifi(&self, p: super::bootstrap::Empty) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::WifiList>;

    fn SetWifi(&self, p: super::bootstrap::Network) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty>;
}

// sync client

pub struct BootstrapClient {
    async_client: BootstrapAsyncClient,
}

impl BootstrapClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        BootstrapAsyncClient::new(host, port, tls).map(|c| {
            BootstrapClient {
                async_client: c,
            }
        })
    }
}

impl Bootstrap for BootstrapClient {
    fn SetName(&self, p: super::bootstrap::Name) -> ::grpc::result::GrpcResult<super::bootstrap::Empty> {
        ::futures::Future::wait(self.async_client.SetName(p))
    }

    fn SetUserName(&self, p: super::bootstrap::UserName) -> ::grpc::result::GrpcResult<super::bootstrap::Empty> {
        ::futures::Future::wait(self.async_client.SetUserName(p))
    }

    fn GetWifi(&self, p: super::bootstrap::Empty) -> ::grpc::result::GrpcResult<super::bootstrap::WifiList> {
        ::futures::Future::wait(self.async_client.GetWifi(p))
    }

    fn SetWifi(&self, p: super::bootstrap::Network) -> ::grpc::result::GrpcResult<super::bootstrap::Empty> {
        ::futures::Future::wait(self.async_client.SetWifi(p))
    }
}

// async client

pub struct BootstrapAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_SetName: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::bootstrap::Name, super::bootstrap::Empty>>,
    method_SetUserName: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::bootstrap::UserName, super::bootstrap::Empty>>,
    method_GetWifi: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::bootstrap::Empty, super::bootstrap::WifiList>>,
    method_SetWifi: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::bootstrap::Network, super::bootstrap::Empty>>,
}

impl BootstrapAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            BootstrapAsyncClient {
                grpc_client: c,
                method_SetName: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Bootstrap/SetName".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_SetUserName: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Bootstrap/SetUserName".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetWifi: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Bootstrap/GetWifi".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_SetWifi: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Bootstrap/SetWifi".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl BootstrapAsync for BootstrapAsyncClient {
    fn SetName(&self, p: super::bootstrap::Name) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        self.grpc_client.call_unary(p, self.method_SetName.clone())
    }

    fn SetUserName(&self, p: super::bootstrap::UserName) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        self.grpc_client.call_unary(p, self.method_SetUserName.clone())
    }

    fn GetWifi(&self, p: super::bootstrap::Empty) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::WifiList> {
        self.grpc_client.call_unary(p, self.method_GetWifi.clone())
    }

    fn SetWifi(&self, p: super::bootstrap::Network) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        self.grpc_client.call_unary(p, self.method_SetWifi.clone())
    }
}

// sync server

pub struct BootstrapServer {
    async_server: BootstrapAsyncServer,
}

struct BootstrapServerHandlerToAsync {
    handler: ::std::sync::Arc<Bootstrap + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl BootstrapAsync for BootstrapServerHandlerToAsync {
    fn SetName(&self, p: super::bootstrap::Name) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.SetName(p)
        })
    }

    fn SetUserName(&self, p: super::bootstrap::UserName) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.SetUserName(p)
        })
    }

    fn GetWifi(&self, p: super::bootstrap::Empty) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::WifiList> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetWifi(p)
        })
    }

    fn SetWifi(&self, p: super::bootstrap::Network) -> ::grpc::futures_grpc::GrpcFutureSend<super::bootstrap::Empty> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.SetWifi(p)
        })
    }
}

impl BootstrapServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Bootstrap + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = BootstrapServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        BootstrapServer {
            async_server: BootstrapAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct BootstrapAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl BootstrapAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : BootstrapAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = BootstrapAsyncServer::new_service_def(h);
        BootstrapAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : BootstrapAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Bootstrap/SetName".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.SetName(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Bootstrap/SetUserName".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.SetUserName(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Bootstrap/GetWifi".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetWifi(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Bootstrap/SetWifi".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.SetWifi(p))
                    },
                ),
            ],
        )
    }
}
