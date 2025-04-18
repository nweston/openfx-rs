#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::strings::OfxStr;
use crate::types::*;
use openfx_sys::{
    OfxImageClipHandle, OfxImageEffectHandle, OfxImageMemoryHandle, OfxParamHandle,
    OfxParamSetHandle, OfxPropertySetHandle,
};
use openfx_sys::{OfxImageEffectSuiteV1, OfxParameterSuiteV1, OfxPropertySuiteV1};
use openfx_sys::{OfxRangeD, OfxRectD, OfxTime};
use std::ffi::{c_char, c_double, c_int, c_uint, c_void};

// Convert values to types which can be passed to the OFX C API.
trait ToOfx {
    type OfxType;

    // Is this value a valid argument to the OFX C API?
    fn is_valid(&self) -> bool {
        true
    }

    // Convert to corresponding OFX type
    fn to_ofx(self) -> Self::OfxType;
}

impl ToOfx for OfxPropertySetHandle {
    type OfxType = OfxPropertySetHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxImageEffectHandle {
    type OfxType = OfxImageEffectHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxImageClipHandle {
    type OfxType = OfxImageClipHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxImageMemoryHandle {
    type OfxType = OfxImageMemoryHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxParamSetHandle {
    type OfxType = OfxParamSetHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxParamHandle {
    type OfxType = OfxParamHandle;

    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxStr<'_> {
    type OfxType = *const c_char;
    fn to_ofx(self) -> Self::OfxType {
        self.as_ptr()
    }
}

impl ToOfx for c_int {
    type OfxType = Self;
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for c_double {
    type OfxType = Self;
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for usize {
    type OfxType = Self;
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for u32 {
    type OfxType = Self;
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl ToOfx for OfxTime {
    type OfxType = Self;
    fn to_ofx(self) -> Self::OfxType {
        self
    }
}

impl<T> ToOfx for *const T {
    type OfxType = *const T;
    fn to_ofx(self) -> Self {
        self
    }
}

impl<T> ToOfx for *mut T {
    type OfxType = *mut T;
    fn to_ofx(self) -> Self {
        self
    }
}

// Generate a wrapper for a suite function.
//
// Use within the impl of a struct which stores the suite in self.suite.
//
// All arguments must implement ToOfx.
//
// Assert that all arguments are valid, convert them to corresponding
// OFX values, and call the named suite function.
macro_rules! wrap {
    (
        $fnname:ident(
            $($name:ident: $type:ty),+
        )

    ) => {
        pub fn $fnname(&self, $($name: $type,)+) -> OfxStatus {
            $(assert!($name.is_valid());)+
            unsafe {(self.suite.$fnname.unwrap())($($name.to_ofx(),)+) }
        }
    };
}

pub struct ImageEffectSuiteV1 {
    suite: &'static OfxImageEffectSuiteV1,
}

impl ImageEffectSuiteV1 {
    // TODO: check suite validity
    pub fn new(suite: &'static OfxImageEffectSuiteV1) -> Self {
        Self { suite }
    }

    wrap!(getPropertySet(
        imageEffect: OfxImageEffectHandle,
        propHandle: *mut OfxPropertySetHandle));
    wrap!(getParamSet(
        imageEffect: OfxImageEffectHandle,
        paramSet: *mut OfxParamSetHandle));
    wrap!(clipDefine(
        imageEffect: OfxImageEffectHandle,
        name: *const c_char,
        propertySet: *mut OfxPropertySetHandle));
    wrap!(clipGetHandle(
        imageEffect: OfxImageEffectHandle,
        name: *const c_char,
        clip: *mut OfxImageClipHandle,
        propertySet: *mut OfxPropertySetHandle));
    wrap!(clipGetPropertySet(
        clip: OfxImageClipHandle,
        propHandle: *mut OfxPropertySetHandle));
    wrap!(clipGetImage(
        clip: OfxImageClipHandle,
        time: OfxTime,
        region: *const OfxRectD,
        imageHandle: *mut OfxPropertySetHandle));
    wrap!(clipReleaseImage(imageHandle: OfxPropertySetHandle));
    wrap!(clipGetRegionOfDefinition(
        clip: OfxImageClipHandle,
        time: OfxTime,
        bounds: *mut OfxRectD));

    pub fn abort(&self, imageEffect: OfxImageEffectHandle) -> c_int {
        assert!(imageEffect.is_valid());
        unsafe { (self.suite.abort.unwrap())(imageEffect) }
    }

    wrap!(imageMemoryAlloc(
        instanceHandle: OfxImageEffectHandle,
        nBytes: usize,
        memoryHandle: *mut OfxImageMemoryHandle));
    wrap!(imageMemoryFree(memoryHandle: OfxImageMemoryHandle));
    wrap!(imageMemoryLock(
        memoryHandle: OfxImageMemoryHandle,
        returnedPtr: *mut *mut c_void));
    wrap!(imageMemoryUnlock(memoryHandle: OfxImageMemoryHandle));
}

pub struct PropertySuiteV1 {
    suite: &'static OfxPropertySuiteV1,
}

impl PropertySuiteV1 {
    // TODO: check suite validity
    pub fn new(suite: &'static OfxPropertySuiteV1) -> Self {
        Self { suite }
    }

    wrap!(propSetPointer(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *mut c_void));

    wrap!(propSetString(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *const c_char));

    wrap!(propSetDouble(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: c_double));

    wrap!(propSetInt(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: c_int));

    wrap!(propSetPointerN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *const *mut c_void));

    wrap!(propSetStringN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *const *const c_char));

    wrap!(propSetDoubleN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *const c_double));

    wrap!(propSetIntN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *const c_int));

    wrap!(propGetPointer(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *mut *mut c_void));

    wrap!(propGetString(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *mut *mut c_char));

    wrap!(propGetDouble(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *mut c_double));

    wrap!(propGetInt(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        index: c_int,
        value: *mut c_int));

    wrap!(propGetPointerN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *mut *mut c_void));

    wrap!(propGetStringN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *mut *mut c_char));

    wrap!(propGetDoubleN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *mut c_double));

    wrap!(propGetIntN(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: c_int,
        value: *mut c_int));

    wrap!(propReset(
        properties: OfxPropertySetHandle,
        property: OfxStr));

    wrap!(propGetDimension(
        properties: OfxPropertySetHandle,
        property: OfxStr,
        count: *mut c_int));
}

type ParamGetValueFn =
    unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> openfx_sys::OfxStatus;

type ParamGetValueAtTimeFn = unsafe extern "C" fn(
    paramHandle: OfxParamHandle,
    time: OfxTime,
    ...
) -> openfx_sys::OfxStatus;

pub trait ParamGetValue {
    fn get_value(
        self,
        fun: ParamGetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus;

    fn get_value_at_time(
        self,
        fun: ParamGetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus;
}

impl ParamGetValue for [*mut c_void; 1] {
    fn get_value(
        self,
        fun: ParamGetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        unsafe { fun(handle, self[0]) }
    }

    fn get_value_at_time(
        self,
        fun: ParamGetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        unsafe { fun(handle, time, self[0]) }
    }
}

impl ParamGetValue for [*mut c_void; 2] {
    fn get_value(
        self,
        fun: ParamGetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        unsafe { fun(handle, self[0], self[1]) }
    }

    fn get_value_at_time(
        self,
        fun: ParamGetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        unsafe { fun(handle, time, self[0], self[1]) }
    }
}

impl ParamGetValue for [*mut c_void; 3] {
    fn get_value(
        self,
        fun: ParamGetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        unsafe { fun(handle, self[0], self[1], self[2]) }
    }

    fn get_value_at_time(
        self,
        fun: ParamGetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        unsafe { fun(handle, time, self[0], self[1], self[2]) }
    }
}

impl ParamGetValue for [*mut c_void; 4] {
    fn get_value(
        self,
        fun: ParamGetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        assert!(self[3].is_valid());
        unsafe { fun(handle, self[0], self[1], self[2], self[3]) }
    }

    fn get_value_at_time(
        self,
        fun: ParamGetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        assert!(self[3].is_valid());
        unsafe { fun(handle, time, self[0], self[1], self[2], self[3]) }
    }
}

type ParamSetValueFn =
    unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> openfx_sys::OfxStatus;

type ParamSetValueAtTimeFn = unsafe extern "C" fn(
    paramHandle: OfxParamHandle,
    time: OfxTime,
    ...
) -> openfx_sys::OfxStatus;

pub trait ParamSetValue {
    fn set_value(
        self,
        fun: ParamSetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus;

    fn set_value_at_time(
        self,
        fun: ParamSetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus;
}

impl ParamSetValue for [*const c_void; 1] {
    fn set_value(
        self,
        fun: ParamSetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        unsafe { fun(handle, self[0]) }
    }

    fn set_value_at_time(
        self,
        fun: ParamSetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        unsafe { fun(handle, time, self[0]) }
    }
}

impl ParamSetValue for [*const c_void; 2] {
    fn set_value(
        self,
        fun: ParamSetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        unsafe { fun(handle, self[0], self[1]) }
    }

    fn set_value_at_time(
        self,
        fun: ParamSetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        unsafe { fun(handle, time, self[0], self[1]) }
    }
}

impl ParamSetValue for [*const c_void; 3] {
    fn set_value(
        self,
        fun: ParamSetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        unsafe { fun(handle, self[0], self[1], self[2]) }
    }

    fn set_value_at_time(
        self,
        fun: ParamSetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        unsafe { fun(handle, time, self[0], self[1], self[2]) }
    }
}

impl ParamSetValue for [*const c_void; 4] {
    fn set_value(
        self,
        fun: ParamSetValueFn,
        handle: OfxParamHandle,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        assert!(self[3].is_valid());
        unsafe { fun(handle, self[0], self[1], self[2], self[3]) }
    }

    fn set_value_at_time(
        self,
        fun: ParamSetValueAtTimeFn,
        handle: OfxParamHandle,
        time: OfxTime,
    ) -> openfx_sys::OfxStatus {
        assert!(self[0].is_valid());
        assert!(self[1].is_valid());
        assert!(self[2].is_valid());
        assert!(self[3].is_valid());
        unsafe { fun(handle, time, self[0], self[1], self[2], self[3]) }
    }
}

#[repr(C)]
pub struct ParameterSuiteV1 {
    suite: &'static OfxParameterSuiteV1,
}

impl ParameterSuiteV1 {
    // TODO: check suite validity
    pub fn new(suite: &'static OfxParameterSuiteV1) -> Self {
        Self { suite }
    }

    wrap!(paramDefine(
        paramSet: OfxParamSetHandle,
        paramType: *const c_char,
        name: *const c_char,
        propertySet: *mut OfxPropertySetHandle));
    wrap!(paramGetHandle(
        paramSet: OfxParamSetHandle,
        name: *const c_char,
        param: *mut OfxParamHandle,
        propertySet: *mut OfxPropertySetHandle));
    wrap!(paramSetGetPropertySet(
        paramSet: OfxParamSetHandle,
        propHandle: *mut OfxPropertySetHandle));
    wrap!(paramGetPropertySet(
        paramHandle: OfxParamHandle,
        propHandle: *mut OfxPropertySetHandle));

    pub fn paramGetValue<T, const N: usize>(
        &self,
        paramHandle: OfxParamHandle,
        result: [T; N],
    ) -> OfxStatus
    where
        [T; N]: ParamGetValue,
    {
        result.get_value(self.suite.paramGetValue.unwrap(), paramHandle)
    }

    pub fn paramGetValueAtTime<T, const N: usize>(
        &self,
        paramHandle: OfxParamHandle,
        time: OfxTime,
        result: [T; N],
    ) -> OfxStatus
    where
        [T; N]: ParamGetValue,
    {
        result.get_value_at_time(
            self.suite.paramGetValueAtTime.unwrap(),
            paramHandle,
            time,
        )
    }

    wrap!(paramGetDerivative(paramHandle: OfxParamHandle, time: OfxTime));
    wrap!(paramGetIntegral(
        paramHandle: OfxParamHandle,
        time1: OfxTime,
        time2: OfxTime));

    pub fn paramSetValue<T, const N: usize>(
        &self,
        paramHandle: OfxParamHandle,
        result: [T; N],
    ) -> OfxStatus
    where
        [T; N]: ParamSetValue,
    {
        result.set_value(self.suite.paramSetValue.unwrap(), paramHandle)
    }

    pub fn paramSetValueAtTime<T, const N: usize>(
        &self,
        paramHandle: OfxParamHandle,
        time: OfxTime,
        result: [T; N],
    ) -> OfxStatus
    where
        [T; N]: ParamSetValue,
    {
        result.set_value_at_time(
            self.suite.paramSetValueAtTime.unwrap(),
            paramHandle,
            time,
        )
    }

    wrap!(paramGetNumKeys(
        paramHandle: OfxParamHandle,
        numberOfKeys: *mut c_uint));
    wrap!(paramGetKeyTime(
        paramHandle: OfxParamHandle,
        nthKey: c_uint,
        time: *mut OfxTime));
    wrap!(paramGetKeyIndex(
        paramHandle: OfxParamHandle,
        time: OfxTime,
        direction: c_int,
        index: *mut c_int));
    wrap!(paramDeleteKey(paramHandle: OfxParamHandle, time: OfxTime));
    wrap!(paramDeleteAllKeys(paramHandle: OfxParamHandle));
    wrap!(paramCopy(
        paramTo: OfxParamHandle,
        paramFrom: OfxParamHandle,
        dstOffset: OfxTime,
        frameRange: *const OfxRangeD));
    wrap!(paramEditBegin(paramSet: OfxParamSetHandle, name: *const c_char));
    wrap!(paramEditEnd(paramSet: OfxParamSetHandle));
}
