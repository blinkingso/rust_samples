use crate::grpc_client::request::Request;
use crate::grpc_service::{Metadata, Payload};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::borrow::{Borrow, Cow};
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};
use std::net::{IpAddr, Ipv4Addr};
use std::ops::{Deref, DerefMut};
use std::result::Result;

const TYPE_NAME_SPLIT: &'static str = "::";
fn get_type_name<T>() -> String {
    let type_name = type_name::<T>();
    if type_name.contains(TYPE_NAME_SPLIT) {
        let (_, r) = type_name.rsplit_once(TYPE_NAME_SPLIT).unwrap();
        return r.to_string();
    }

    type_name.to_string()
}

pub fn convert_request<T>(request: &T) -> Payload
where
    T: Serialize + Deref<Target = Request>,
{
    let metadata = Metadata {
        r#type: get_type_name::<T>(),
        client_ip: local_ip()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .to_string(),
        headers: request.get_headers(),
    };

    convert(metadata, request)
}

fn convert<T: Serialize>(metadata: Metadata, value: &T) -> Payload {
    let json_str = serde_json::to_string(value).unwrap();
    let body = prost_types::Any {
        type_url: "".to_string(),
        value: json_str.into_bytes(),
    };

    Payload {
        metadata: Some(metadata),
        body: Some(body),
    }
}

pub fn parse_request<'de, T>(payload: &'de Payload) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'de> + Deref<Target = Request> + DerefMut<Target = Request>,
{
    if let Some(ref body) = payload.body {
        let bytes = body.value.as_slice();
        let mut req = serde_json::from_slice::<T>(bytes)?;
        if let Some(ref md) = payload.metadata {
            req.put_all_headers(md.headers.clone());
        }
        return Ok(req);
    } else {
        return Err(Box::new(GrpcError::new("payload must not be null")));
    }
}

#[derive(Debug)]
struct GrpcError<'a>(pub Cow<'a, String>);
impl<'a> GrpcError<'a> {
    pub fn new(error_msg: &str) -> Self {
        GrpcError(Cow::Owned(error_msg.to_string()))
    }
}
impl<'a> Display for GrpcError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = format!("GrpcError: {}", *(self.0));
        write!(f, "{}", &msg)
    }
}
impl<'a> std::error::Error for GrpcError<'a> {}

#[test]
fn test() {
    let v = vec![];
}
