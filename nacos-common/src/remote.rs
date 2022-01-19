use self::request::RpcRequest;
use async_trait::async_trait;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

macro_rules! impl_deref_request {
    ($target: ty, $target_ty: ty, $target_ident: ident) => {
        impl ::std::ops::Deref for $target {
            type Target = $target_ty;
            fn deref(&self) -> &Self::Target {
                &self.$target_ident
            }
        }
        impl ::std::ops::DerefMut for $target {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$target_ident
            }
        }
    };
}
macro_rules! impl_internal_request {
   ($($target_ty:ty),+ $(,)?) => {
         $(
            impl $crate::remote::request::InternalRequest for $target_ty {}
        )*
    }
}
macro_rules! impl_client_request {
    ($($target_ty:ty),+ $(,)?) => {
         $(
            impl $crate::remote::request::ClientRequest for $target_ty {}
        )*
    }
}
macro_rules! impl_server_request {
    ($($target_ty:ty),+ $(,)?) => {
         $(
            impl $crate::remote::request::ServerRequest for $target_ty {}
        )*
    }
}

/// structst to handle communication from client to server.
pub mod request {
    use crate::api::ability::ClientAbilities;
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// A mark trait to mark server request.
    pub trait ServerRequest {}
    /// A marker trait to mark client request.
    pub trait ClientRequest {}

    /// A internal request trait to mark  `internal` module.
    pub trait InternalRequest {
        fn get_module(&self) -> String {
            String::from("internal")
        }
    }
    pub trait ConfigRequest {
        fn get_module(&self) -> String {
            String::from("config")
        }
    }

    pub trait NamingRequest {
        fn get_module(&self) -> String {
            String::from("naming")
        }
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RpcRequest {
        pub headers: HashMap<String, String>,
        pub request_id: Option<String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientDetectionRequest {
        #[serde(flatten)]
        request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectionSetupRequest {
        #[serde(flatten)]
        request: RpcRequest,
        pub client_version: String,
        pub abilities: ClientAbilities,
        pub tenant: String,
        pub labels: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectResetRequest {
        #[serde(flatten)]
        request: RpcRequest,
        pub server_ip: String,
        pub server_port: String,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HealthCheckRequest {
        #[serde(flatten)]
        request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PushAckRequest {
        #[serde(flatten)]
        request: RpcRequest,
        pub request_id: String,
        pub success: bool,
        pub exception: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestMeta {
        pub connection_id: String,
        pub client_ip: String,
        pub client_version: String,
        pub labels: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerCheckRequest {
        #[serde(flatten)]
        request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerLoaderInfoRequest {
        #[serde(flatten)]
        request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerReloadRequest {
        #[serde(flatten)]
        request: RpcRequest,
        pub reload_count: usize,
        pub reload_server: String,
    }

    impl Default for RpcRequest {
        fn default() -> Self {
            RpcRequest {
                headers: Default::default(),
                request_id: None,
            }
        }
    }

    impl_deref_request!(ServerCheckRequest, RpcRequest, request);
    impl_deref_request!(HealthCheckRequest, RpcRequest, request);
    impl_deref_request!(ConnectionSetupRequest, RpcRequest, request);

    impl_internal_request!(
        ServerReloadRequest,
        ServerLoaderInfoRequest,
        ServerCheckRequest,
        PushAckRequest,
        HealthCheckRequest,
        ConnectionSetupRequest,
    );
    impl_server_request!(ConnectResetRequest, ClientDetectionRequest);
}

/// structs to handle communication from server to client.
pub mod response {}

#[async_trait]
pub trait Requester {
    type Item: Serialize;
    type Req: DerefMut<Target = RpcRequest>;
}
