use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use crate::{request::DNSRequest, response::DnsResponse};

#[derive(Debug)]
pub enum DnsRequestError {
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    DSOTYPENI,
    BadVersOrSig,
    BadKey,
    BadTime,
    BadMode,
    BadName,
    BadAlg,
    BadTrunc,
    BadCookie,
}

impl DnsRequestError {
    pub fn to_response_code(&self) -> u8 {
        match self {
            DnsRequestError::FormErr => 1,
            DnsRequestError::ServFail => 2,
            DnsRequestError::NXDomain => 3,
            DnsRequestError::NotImp => 4,
            DnsRequestError::Refused => 5,
            DnsRequestError::YXDomain => 6,
            DnsRequestError::YXRRSet => 7,
            DnsRequestError::NXRRSet => 8,
            DnsRequestError::NotAuth => 9,
            DnsRequestError::NotZone => 10,
            DnsRequestError::DSOTYPENI => 11,
            DnsRequestError::BadVersOrSig => 16,
            DnsRequestError::BadKey => 17,
            DnsRequestError::BadTime => 18,
            DnsRequestError::BadMode => 19,
            DnsRequestError::BadName => 20,
            DnsRequestError::BadAlg => 21,
            DnsRequestError::BadTrunc => 22,
            DnsRequestError::BadCookie => 23,
        }
    }
}

impl std::fmt::Display for DnsRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsRequestError::FormErr => write!(f, "FormErr"),
            DnsRequestError::ServFail => write!(f, "ServFail"),
            DnsRequestError::NXDomain => write!(f, "NXDomain"),
            DnsRequestError::NotImp => write!(f, "NotImp"),
            DnsRequestError::Refused => write!(f, "Refused"),
            DnsRequestError::YXDomain => write!(f, "YXDomain"),
            DnsRequestError::YXRRSet => write!(f, "YXRRSet"),
            DnsRequestError::NXRRSet => write!(f, "NXRRSet"),
            DnsRequestError::NotAuth => write!(f, "NotAuth"),
            DnsRequestError::NotZone => write!(f, "NotZone"),
            DnsRequestError::DSOTYPENI => write!(f, "DSOTYPENI"),
            DnsRequestError::BadVersOrSig => write!(f, "BadVersOrSig"),
            DnsRequestError::BadKey => write!(f, "BadKey"),
            DnsRequestError::BadTime => write!(f, "BadTime"),
            DnsRequestError::BadMode => write!(f, "BadMode"),
            DnsRequestError::BadName => write!(f, "BadName"),
            DnsRequestError::BadAlg => write!(f, "BadAlg"),
            DnsRequestError::BadTrunc => write!(f, "BadTrunc"),
            DnsRequestError::BadCookie => write!(f, "BadCookie"),
        }
    }
}

impl From<std::io::Error> for DnsRequestError {
    fn from(_: std::io::Error) -> Self {
        DnsRequestError::ServFail
    }
}

impl From<Box<dyn std::error::Error>> for DnsRequestError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        DnsRequestError::ServFail
    }
}

pub trait DnsRequestHandler: Send + Sync + 'static {
    fn handle_request<'a>(
        self: Arc<Self>,
        request: DNSRequest,
    ) -> Pin<Box<dyn Future<Output = Result<DnsResponse, DnsRequestError>> + Send>>;
}
