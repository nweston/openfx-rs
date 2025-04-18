use crate::constants::ofxstatus;
use openfx_sys;
pub use openfx_sys::{
    OfxHost, OfxImageClipHandle, OfxImageEffectHandle, OfxParamHandle, OfxParamSetHandle,
    OfxPropertySetHandle, OfxRectD, OfxRectI, OfxTime,
};

#[derive(Eq, PartialEq)]
#[must_use]
pub struct OfxStatus(openfx_sys::OfxStatus);

impl OfxStatus {
    pub const fn new(value: openfx_sys::OfxStatus) -> Self {
        Self(value)
    }

    pub const fn from_i32(value: i32) -> Self {
        Self(openfx_sys::OfxStatus(value))
    }

    pub fn failed(&self) -> bool {
        match *self {
            ofxstatus::OK | ofxstatus::ReplyDefault => false,
            _ => true,
        }
    }

    pub fn succeeded(&self) -> bool {
        !self.failed()
    }

    pub fn and_then<T, F>(self, op: F) -> Result<T, OfxStatus>
    where
        F: FnOnce() -> T,
    {
        if self.succeeded() {
            Ok(op())
        } else {
            Err(self)
        }
    }

    pub fn to_result(self) -> Result<(), OfxStatus> {
        self.and_then(|| ())
    }
}

impl Into<openfx_sys::OfxStatus> for OfxStatus {
    fn into(self) -> openfx_sys::OfxStatus {
        self.0
    }
}

fn status_to_str(status: &OfxStatus) -> Option<&'static str> {
    match *status {
        ofxstatus::OK => Some("OK"),
        ofxstatus::Failed => Some("Failed"),
        ofxstatus::ErrFatal => Some("ErrFatal"),
        ofxstatus::ErrUnknown => Some("ErrUnknown"),
        ofxstatus::ErrMissingHostFeature => Some("ErrMissingHostFeature"),
        ofxstatus::ErrUnsupported => Some("ErrUnsupported"),
        ofxstatus::ErrExists => Some("ErrExists"),
        ofxstatus::ErrFormat => Some("ErrFormat"),
        ofxstatus::ErrMemory => Some("ErrMemory"),
        ofxstatus::ErrBadHandle => Some("ErrBadHandle"),
        ofxstatus::ErrBadIndex => Some("ErrBadIndex"),
        ofxstatus::ErrValue => Some("ErrValue"),
        ofxstatus::ReplyYes => Some("ReplyYes"),
        ofxstatus::ReplyNo => Some("ReplyNo"),
        ofxstatus::ReplyDefault => Some("ReplyDefault"),
        ofxstatus::ErrImageFormat => Some("ErrImageFormat"),
        ofxstatus::GLOutOfMemory => Some("GLOutOfMemory"),
        ofxstatus::GLRenderFailed => Some("GLRenderFailed"),
        _ => None,
    }
}

impl std::fmt::Display for OfxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match status_to_str(self) {
            Some(s) => write!(f, "OfxStatus({})", s),
            None => write!(f, "OfxStatus({})", self.0 .0),
        }
    }
}
