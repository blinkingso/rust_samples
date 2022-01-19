use self::request::RpcRequest;
use async_trait::async_trait;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

macro_rules! impl_deref_mut {
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

/// structs to handle communication from client to server.
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

    impl_deref_mut!(ServerCheckRequest, RpcRequest, request);
    impl_deref_mut!(HealthCheckRequest, RpcRequest, request);
    impl_deref_mut!(ConnectionSetupRequest, RpcRequest, request);

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

macro_rules! response_code {
    (
        $(
            ($code: expr, $konst:ident, $desc: expr);
        )+
    ) => {
        impl ResponseCode {
            $(
                pub const $konst:ResponseCode = ResponseCode($code);
            )+

            fn desc(code: u16) -> Option<&'static str> {
                match code {
                    $(
                        $code => Some($desc),
                    )+
                    _ => None,
                }
            }
        }
    }
}
/// structs to handle communication from server to client.
pub mod response {
    use log::Level::Error;
    use serde::{Deserialize, Deserializer, Serialize};
    use std::collections::HashMap;
    const CODE_SUCCESS: u16 = 200;
    const CODE_FAIL: u16 = 500;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ResponseCode {
        pub code: u16,
        pub desc: &'static str,
    }

    impl ResponseCode {
        pub const SUCCESS: ResponseCode = ResponseCode {
            code: 200,
            desc: "Response ok",
        };
        pub const FAIL: ResponseCode = ResponseCode {
            code: 500,
            desc: "Response fail",
        };

        pub fn from_u16(code: u16) -> Option<ResponseCode> {
            match code {
                CODE_SUCCESS => Some(ResponseCode::SUCCESS),
                CODE_FAIL => Some(ResponseCode::FAIL),
                _ => None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RpcResponse {
        pub result_code: u16,
        pub error_code: u32,
        pub message: Option<String>,
        pub request_id: Option<String>,
    }

    impl RpcResponse {
        pub fn set_error_info(&mut self, error_code: u32, error_message: String) {
            self.result_code = ResponseCode::FAIL.code;
            self.error_code = error_code;
            self.message = Some(error_message);
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientDetectionResponse {
        #[serde(flatten)]
        response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectResetResponse {
        #[serde(flatten)]
        response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorResponse {
        #[serde(flatten)]
        response: RpcResponse,
    }

    impl ErrorResponse {
        pub fn build(error_code: u32, msg: String) -> ErrorResponse {
            let mut response = RpcResponse {
                result_code: CODE_SUCCESS,
                error_code,
                message: None,
                request_id: None,
            };

            response.set_error_info(error_code, msg);
            ErrorResponse { response }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HealthCheckResponse {
        #[serde(flatten)]
        response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerCheckResponse {
        #[serde(flatten)]
        response: RpcResponse,
        pub connection_id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerLoaderInfoResponse {
        #[serde(flatten)]
        response: RpcResponse,
        pub address: String,
        pub loader_metrics: HashMap<String, Option<String>>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerReloadResponse {
        #[serde(flatten)]
        response: RpcResponse,
    }

    impl_deref_mut!(ClientDetectionResponse, RpcResponse, response);
    impl_deref_mut!(ConnectResetResponse, RpcResponse, response);
    impl_deref_mut!(ErrorResponse, RpcResponse, response);
    impl_deref_mut!(HealthCheckResponse, RpcResponse, response);
    impl_deref_mut!(ServerCheckResponse, RpcResponse, response);
    impl_deref_mut!(ServerLoaderInfoResponse, RpcResponse, response);
    impl_deref_mut!(ServerReloadResponse, RpcResponse, response);
}

#[async_trait]
pub trait Requester {
    type Item: Serialize;
    type Req: DerefMut<Target = RpcRequest>;
}
