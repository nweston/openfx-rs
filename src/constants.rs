#![allow(non_upper_case_globals)]
#![allow(dead_code)]

macro_rules! constant {
    ($name: ident, $const: ident) => {
        pub const $name: crate::strings::OfxStr =
            crate::strings::OfxStr::from_bytes(openfx_sys::$const);
    };
}

pub mod ofxstatus {
    pub use openfx_sys::ofxstatus::*;
}

constant!(ActionLoad, kOfxActionLoad);
constant!(ActionOpenGLContextAttached, kOfxActionOpenGLContextAttached);
constant!(ActionOpenGLContextDetached, kOfxActionOpenGLContextDetached);
constant!(ActionDescribe, kOfxActionDescribe);
constant!(ActionUnload, kOfxActionUnload);
constant!(ActionPurgeCaches, kOfxActionPurgeCaches);
constant!(ActionSyncPrivateData, kOfxActionSyncPrivateData);
constant!(ActionCreateInstance, kOfxActionCreateInstance);
constant!(ActionDestroyInstance, kOfxActionDestroyInstance);
constant!(ActionInstanceChanged, kOfxActionInstanceChanged);
constant!(ActionBeginInstanceChanged, kOfxActionBeginInstanceChanged);
constant!(ActionEndInstanceChanged, kOfxActionEndInstanceChanged);
constant!(ActionBeginInstanceEdit, kOfxActionBeginInstanceEdit);
constant!(ActionEndInstanceEdit, kOfxActionEndInstanceEdit);
constant!(MultiThreadSuite, kOfxMultiThreadSuite);
constant!(ProgressSuite, kOfxProgressSuite);
constant!(PropertySuite, kOfxPropertySuite);
constant!(InteractSuite, kOfxInteractSuite);
constant!(TimeLineSuite, kOfxTimeLineSuite);
constant!(OpenGLRenderSuite, kOfxOpenGLRenderSuite);
constant!(ImageEffectSuite, kOfxImageEffectSuite);
constant!(MemorySuite, kOfxMemorySuite);
constant!(ParameterSuite, kOfxParameterSuite);
constant!(ParametricParameterSuite, kOfxParametricParameterSuite);
constant!(MessageSuite, kOfxMessageSuite);
constant!(DialogSuite, kOfxDialogSuite);
constant!(ParamHostPropMaxPages, kOfxParamHostPropMaxPages);
constant!(ParamHostPropMaxParameters, kOfxParamHostPropMaxParameters);
constant!(
    ParamHostPropPageRowColumnCount,
    kOfxParamHostPropPageRowColumnCount
);
constant!(
    ParamHostPropSupportsBooleanAnimation,
    kOfxParamHostPropSupportsBooleanAnimation
);
constant!(
    ParamHostPropSupportsChoiceAnimation,
    kOfxParamHostPropSupportsChoiceAnimation
);
constant!(
    ParamHostPropSupportsCustomAnimation,
    kOfxParamHostPropSupportsCustomAnimation
);
constant!(
    ParamHostPropSupportsCustomInteract,
    kOfxParamHostPropSupportsCustomInteract
);
constant!(
    ParamHostPropSupportsParametricAnimation,
    kOfxParamHostPropSupportsParametricAnimation
);
constant!(
    ParamHostPropSupportsStrChoiceAnimation,
    kOfxParamHostPropSupportsStrChoiceAnimation
);
constant!(
    ParamHostPropSupportsStringAnimation,
    kOfxParamHostPropSupportsStringAnimation
);
constant!(
    ImageEffectHostPropIsBackground,
    kOfxImageEffectHostPropIsBackground
);
constant!(
    ImageEffectHostPropNativeOrigin,
    kOfxImageEffectHostPropNativeOrigin
);
constant!(
    ImageEffectPluginPropHostFrameThreading,
    kOfxImageEffectPluginPropHostFrameThreading
);
constant!(PropHostOSHandle, kOfxPropHostOSHandle);
constant!(HostNativeOriginBottomLeft, kOfxHostNativeOriginBottomLeft);
constant!(HostNativeOriginCenter, kOfxHostNativeOriginCenter);
constant!(HostNativeOriginTopLeft, kOfxHostNativeOriginTopLeft);
constant!(
    ImageEffectActionBeginSequenceRender,
    kOfxImageEffectActionBeginSequenceRender
);
constant!(
    ImageEffectActionDescribeInContext,
    kOfxImageEffectActionDescribeInContext
);
constant!(
    ImageEffectActionEndSequenceRender,
    kOfxImageEffectActionEndSequenceRender
);
constant!(
    ImageEffectActionGetClipPreferences,
    kOfxImageEffectActionGetClipPreferences
);
constant!(
    ImageEffectActionGetFramesNeeded,
    kOfxImageEffectActionGetFramesNeeded
);
constant!(
    ImageEffectActionGetRegionOfDefinition,
    kOfxImageEffectActionGetRegionOfDefinition
);
constant!(
    ImageEffectActionGetRegionsOfInterest,
    kOfxImageEffectActionGetRegionsOfInterest
);
constant!(
    ImageEffectActionGetTimeDomain,
    kOfxImageEffectActionGetTimeDomain
);
constant!(ImageEffectActionIsIdentity, kOfxImageEffectActionIsIdentity);
constant!(ImageEffectActionRender, kOfxImageEffectActionRender);
constant!(ImageEffectContextFilter, kOfxImageEffectContextFilter);
constant!(ImageEffectContextGeneral, kOfxImageEffectContextGeneral);
constant!(ImageEffectContextGenerator, kOfxImageEffectContextGenerator);
constant!(ImageEffectContextPaint, kOfxImageEffectContextPaint);
constant!(ImageEffectContextRetimer, kOfxImageEffectContextRetimer);
constant!(
    ImageEffectContextTransition,
    kOfxImageEffectContextTransition
);
constant!(ImageEffectFrameVarying, kOfxImageEffectFrameVarying);
constant!(
    ImageEffectInstancePropEffectDuration,
    kOfxImageEffectInstancePropEffectDuration
);
constant!(
    ImageEffectInstancePropSequentialRender,
    kOfxImageEffectInstancePropSequentialRender
);
constant!(ImageEffectOutputClipName, kOfxImageEffectOutputClipName);
constant!(ImageEffectPluginApi, kOfxImageEffectPluginApi);
pub const ImageEffectPluginApiVersion: u32 = openfx_sys::kOfxImageEffectPluginApiVersion;
constant!(
    ImageEffectPluginPropFieldRenderTwiceAlways,
    kOfxImageEffectPluginPropFieldRenderTwiceAlways
);
constant!(
    ImageEffectPluginPropGrouping,
    kOfxImageEffectPluginPropGrouping
);
constant!(
    ImageEffectPluginPropOverlayInteractV1,
    kOfxImageEffectPluginPropOverlayInteractV1
);
constant!(
    ImageEffectPluginPropSingleInstance,
    kOfxImageEffectPluginPropSingleInstance
);
constant!(
    ImageEffectPluginRenderThreadSafety,
    kOfxImageEffectPluginRenderThreadSafety
);
constant!(
    ImageEffectPropClipPreferencesSlaveParam,
    kOfxImageEffectPropClipPreferencesSlaveParam
);
constant!(ImageEffectPropComponents, kOfxImageEffectPropComponents);
constant!(ImageEffectPropContext, kOfxImageEffectPropContext);
constant!(ImageEffectPropCudaEnabled, kOfxImageEffectPropCudaEnabled);
constant!(
    ImageEffectPropCudaRenderSupported,
    kOfxImageEffectPropCudaRenderSupported
);
constant!(ImageEffectPropCudaStream, kOfxImageEffectPropCudaStream);
constant!(
    ImageEffectPropCudaStreamSupported,
    kOfxImageEffectPropCudaStreamSupported
);
constant!(
    ImageEffectPropFieldToRender,
    kOfxImageEffectPropFieldToRender
);
constant!(ImageEffectPropFrameRange, kOfxImageEffectPropFrameRange);
constant!(ImageEffectPropFrameRate, kOfxImageEffectPropFrameRate);
constant!(ImageEffectPropFrameStep, kOfxImageEffectPropFrameStep);
constant!(
    ImageEffectPropInteractiveRenderStatus,
    kOfxImageEffectPropInteractiveRenderStatus
);
constant!(
    ImageEffectPropMetalCommandQueue,
    kOfxImageEffectPropMetalCommandQueue
);
constant!(ImageEffectPropMetalEnabled, kOfxImageEffectPropMetalEnabled);
constant!(
    ImageEffectPropMetalRenderSupported,
    kOfxImageEffectPropMetalRenderSupported
);
constant!(
    ImageEffectPropOpenCLCommandQueue,
    kOfxImageEffectPropOpenCLCommandQueue
);
constant!(
    ImageEffectPropOpenCLEnabled,
    kOfxImageEffectPropOpenCLEnabled
);
constant!(
    ImageEffectPropOpenCLRenderSupported,
    kOfxImageEffectPropOpenCLRenderSupported
);
constant!(
    ImageEffectPropOpenGLEnabled,
    kOfxImageEffectPropOpenGLEnabled
);
constant!(
    ImageEffectPropOpenGLRenderSupported,
    kOfxImageEffectPropOpenGLRenderSupported
);
constant!(
    ImageEffectPropOpenGLTextureIndex,
    kOfxImageEffectPropOpenGLTextureIndex
);
constant!(
    ImageEffectPropOpenGLTextureTarget,
    kOfxImageEffectPropOpenGLTextureTarget
);
constant!(ImageEffectPropPixelDepth, kOfxImageEffectPropPixelDepth);
constant!(ImageEffectPropPluginHandle, kOfxImageEffectPropPluginHandle);
constant!(
    ImageEffectPropPreMultiplication,
    kOfxImageEffectPropPreMultiplication
);
constant!(
    ImageEffectPropProjectExtent,
    kOfxImageEffectPropProjectExtent
);
constant!(
    ImageEffectPropProjectOffset,
    kOfxImageEffectPropProjectOffset
);
constant!(
    ImageEffectPropProjectPixelAspectRatio,
    kOfxImageEffectPropProjectPixelAspectRatio
);
constant!(ImageEffectPropProjectSize, kOfxImageEffectPropProjectSize);
constant!(
    ImageEffectPropRegionOfDefinition,
    kOfxImageEffectPropRegionOfDefinition
);
constant!(
    ImageEffectPropRegionOfInterest,
    kOfxImageEffectPropRegionOfInterest
);
constant!(
    ImageEffectPropRenderQualityDraft,
    kOfxImageEffectPropRenderQualityDraft
);
constant!(ImageEffectPropRenderScale, kOfxImageEffectPropRenderScale);
constant!(ImageEffectPropRenderWindow, kOfxImageEffectPropRenderWindow);
constant!(
    ImageEffectPropSequentialRenderStatus,
    kOfxImageEffectPropSequentialRenderStatus
);
constant!(
    ImageEffectPropSetableFielding,
    kOfxImageEffectPropSetableFielding
);
constant!(
    ImageEffectPropSetableFrameRate,
    kOfxImageEffectPropSetableFrameRate
);
constant!(
    ImageEffectPropSupportedComponents,
    kOfxImageEffectPropSupportedComponents
);
constant!(
    ImageEffectPropSupportedContexts,
    kOfxImageEffectPropSupportedContexts
);
constant!(
    ImageEffectPropSupportedPixelDepths,
    kOfxImageEffectPropSupportedPixelDepths
);
constant!(
    ImageEffectPropSupportsMultipleClipDepths,
    kOfxImageEffectPropSupportsMultipleClipDepths
);
constant!(
    ImageEffectPropSupportsMultipleClipPARs,
    kOfxImageEffectPropSupportsMultipleClipPARs
);
constant!(
    ImageEffectPropSupportsMultiResolution,
    kOfxImageEffectPropSupportsMultiResolution
);
constant!(
    ImageEffectPropSupportsOverlays,
    kOfxImageEffectPropSupportsOverlays
);
constant!(
    ImageEffectPropSupportsTiles,
    kOfxImageEffectPropSupportsTiles
);
constant!(
    ImageEffectPropTemporalClipAccess,
    kOfxImageEffectPropTemporalClipAccess
);
constant!(
    ImageEffectPropUnmappedFrameRange,
    kOfxImageEffectPropUnmappedFrameRange
);
constant!(
    ImageEffectPropUnmappedFrameRate,
    kOfxImageEffectPropUnmappedFrameRate
);
constant!(ImageEffectRenderFullySafe, kOfxImageEffectRenderFullySafe);
constant!(
    ImageEffectRenderInstanceSafe,
    kOfxImageEffectRenderInstanceSafe
);
constant!(ImageEffectRenderUnsafe, kOfxImageEffectRenderUnsafe);
constant!(ImageEffectRetimerParamName, kOfxImageEffectRetimerParamName);
constant!(
    ImageEffectSimpleSourceClipName,
    kOfxImageEffectSimpleSourceClipName
);
constant!(
    ImageEffectTransitionParamName,
    kOfxImageEffectTransitionParamName
);
constant!(
    ImageEffectTransitionSourceFromClipName,
    kOfxImageEffectTransitionSourceFromClipName
);
constant!(
    ImageEffectTransitionSourceToClipName,
    kOfxImageEffectTransitionSourceToClipName
);
constant!(ImageClipPropConnected, kOfxImageClipPropConnected);
constant!(
    ImageClipPropContinuousSamples,
    kOfxImageClipPropContinuousSamples
);
constant!(
    ImageClipPropFieldExtraction,
    kOfxImageClipPropFieldExtraction
);
constant!(ImageClipPropFieldOrder, kOfxImageClipPropFieldOrder);
constant!(ImageClipPropIsMask, kOfxImageClipPropIsMask);
constant!(ImageClipPropOptional, kOfxImageClipPropOptional);
constant!(
    ImageClipPropUnmappedComponents,
    kOfxImageClipPropUnmappedComponents
);
constant!(
    ImageClipPropUnmappedPixelDepth,
    kOfxImageClipPropUnmappedPixelDepth
);
constant!(ImageComponentAlpha, kOfxImageComponentAlpha);
constant!(ImageComponentNone, kOfxImageComponentNone);
constant!(ImageComponentRGB, kOfxImageComponentRGB);
constant!(ImageComponentRGBA, kOfxImageComponentRGBA);
constant!(ImageFieldBoth, kOfxImageFieldBoth);
constant!(ImageFieldDoubled, kOfxImageFieldDoubled);
constant!(ImageFieldLower, kOfxImageFieldLower);
constant!(ImageFieldNone, kOfxImageFieldNone);
constant!(ImageFieldSingle, kOfxImageFieldSingle);
constant!(ImageFieldUpper, kOfxImageFieldUpper);
constant!(ImageOpaque, kOfxImageOpaque);
constant!(ImagePreMultiplied, kOfxImagePreMultiplied);
constant!(ImageUnPreMultiplied, kOfxImageUnPreMultiplied);
constant!(ImagePropBounds, kOfxImagePropBounds);
constant!(ImagePropData, kOfxImagePropData);
constant!(ImagePropField, kOfxImagePropField);
constant!(ImagePropPixelAspectRatio, kOfxImagePropPixelAspectRatio);
constant!(ImagePropRegionOfDefinition, kOfxImagePropRegionOfDefinition);
constant!(ImagePropRowBytes, kOfxImagePropRowBytes);
constant!(ImagePropUniqueIdentifier, kOfxImagePropUniqueIdentifier);
constant!(PropAPIVersion, kOfxPropAPIVersion);
constant!(PropChangeReason, kOfxPropChangeReason);
constant!(PropEffectInstance, kOfxPropEffectInstance);
constant!(PropIcon, kOfxPropIcon);
constant!(PropInstanceData, kOfxPropInstanceData);
constant!(PropIsInteractive, kOfxPropIsInteractive);
constant!(PropKeyString, kOfxPropKeyString);
constant!(PropKeySym, kOfxPropKeySym);
constant!(PropLabel, kOfxPropLabel);
constant!(PropLongLabel, kOfxPropLongLabel);
constant!(PropName, kOfxPropName);
constant!(PropParamSetNeedsSyncing, kOfxPropParamSetNeedsSyncing);
constant!(PropPluginDescription, kOfxPropPluginDescription);
constant!(PropShortLabel, kOfxPropShortLabel);
constant!(PropTime, kOfxPropTime);
constant!(PropType, kOfxPropType);
constant!(PropVersion, kOfxPropVersion);
constant!(PropVersionLabel, kOfxPropVersionLabel);
constant!(ParamCoordinatesCanonical, kOfxParamCoordinatesCanonical);
constant!(ParamCoordinatesNormalised, kOfxParamCoordinatesNormalised);
constant!(ParamDoubleTypeAbsoluteTime, kOfxParamDoubleTypeAbsoluteTime);
constant!(ParamDoubleTypeAngle, kOfxParamDoubleTypeAngle);
constant!(ParamDoubleTypePlain, kOfxParamDoubleTypePlain);
constant!(ParamDoubleTypeScale, kOfxParamDoubleTypeScale);
constant!(ParamDoubleTypeTime, kOfxParamDoubleTypeTime);
constant!(ParamDoubleTypeX, kOfxParamDoubleTypeX);
constant!(ParamDoubleTypeXAbsolute, kOfxParamDoubleTypeXAbsolute);
constant!(ParamDoubleTypeXY, kOfxParamDoubleTypeXY);
constant!(ParamDoubleTypeXYAbsolute, kOfxParamDoubleTypeXYAbsolute);
constant!(ParamDoubleTypeY, kOfxParamDoubleTypeY);
constant!(ParamDoubleTypeYAbsolute, kOfxParamDoubleTypeYAbsolute);
constant!(ParamInvalidateAll, kOfxParamInvalidateAll);
constant!(ParamInvalidateValueChange, kOfxParamInvalidateValueChange);
constant!(
    ParamInvalidateValueChangeToEnd,
    kOfxParamInvalidateValueChangeToEnd
);
constant!(ParamPageSkipColumn, kOfxParamPageSkipColumn);
constant!(ParamPageSkipRow, kOfxParamPageSkipRow);
constant!(ParamPropAnimates, kOfxParamPropAnimates);
constant!(ParamPropCacheInvalidation, kOfxParamPropCacheInvalidation);
constant!(ParamPropCanUndo, kOfxParamPropCanUndo);
constant!(ParamPropChoiceEnum, kOfxParamPropChoiceEnum);
constant!(ParamPropChoiceOption, kOfxParamPropChoiceOption);
constant!(
    ParamPropCustomInterpCallbackV1,
    kOfxParamPropCustomInterpCallbackV1
);
constant!(ParamPropCustomValue, kOfxParamPropCustomValue);
constant!(ParamPropDataPtr, kOfxParamPropDataPtr);
constant!(ParamPropDefault, kOfxParamPropDefault);
constant!(
    ParamPropDefaultCoordinateSystem,
    kOfxParamPropDefaultCoordinateSystem
);
constant!(ParamPropDigits, kOfxParamPropDigits);
constant!(ParamPropDimensionLabel, kOfxParamPropDimensionLabel);
constant!(ParamPropDisplayMax, kOfxParamPropDisplayMax);
constant!(ParamPropDisplayMin, kOfxParamPropDisplayMin);
constant!(ParamPropDoubleType, kOfxParamPropDoubleType);
constant!(ParamPropEnabled, kOfxParamPropEnabled);
constant!(ParamPropEvaluateOnChange, kOfxParamPropEvaluateOnChange);
constant!(ParamPropGroupOpen, kOfxParamPropGroupOpen);
constant!(
    ParamPropHasHostOverlayHandle,
    kOfxParamPropHasHostOverlayHandle
);
constant!(ParamPropHint, kOfxParamPropHint);
constant!(ParamPropIncrement, kOfxParamPropIncrement);
constant!(
    ParamPropInteractMinimumSize,
    kOfxParamPropInteractMinimumSize
);
constant!(
    ParamPropInteractPreferedSize,
    kOfxParamPropInteractPreferedSize
);
constant!(ParamPropInteractSize, kOfxParamPropInteractSize);
constant!(ParamPropInteractSizeAspect, kOfxParamPropInteractSizeAspect);
constant!(ParamPropInteractV1, kOfxParamPropInteractV1);
constant!(
    ParamPropInterpolationAmount,
    kOfxParamPropInterpolationAmount
);
constant!(ParamPropInterpolationTime, kOfxParamPropInterpolationTime);
constant!(ParamPropIsAnimating, kOfxParamPropIsAnimating);
constant!(ParamPropIsAutoKeying, kOfxParamPropIsAutoKeying);
constant!(ParamPropMax, kOfxParamPropMax);
constant!(ParamPropMin, kOfxParamPropMin);
constant!(ParamPropPageChild, kOfxParamPropPageChild);
constant!(
    ParamPropParametricDimension,
    kOfxParamPropParametricDimension
);
constant!(
    ParamPropParametricInteractBackground,
    kOfxParamPropParametricInteractBackground
);
constant!(ParamPropParametricRange, kOfxParamPropParametricRange);
constant!(ParamPropParametricUIColour, kOfxParamPropParametricUIColour);
constant!(ParamPropParent, kOfxParamPropParent);
constant!(ParamPropPersistant, kOfxParamPropPersistant);
constant!(ParamPropPluginMayWrite, kOfxParamPropPluginMayWrite);
constant!(ParamPropScriptName, kOfxParamPropScriptName);
constant!(ParamPropSecret, kOfxParamPropSecret);
constant!(ParamPropShowTimeMarker, kOfxParamPropShowTimeMarker);
constant!(
    ParamPropStringFilePathExists,
    kOfxParamPropStringFilePathExists
);
constant!(ParamPropStringMode, kOfxParamPropStringMode);
constant!(ParamPropType, kOfxParamPropType);
constant!(
    ParamPropUseHostOverlayHandle,
    kOfxParamPropUseHostOverlayHandle
);
constant!(ParamStringIsDirectoryPath, kOfxParamStringIsDirectoryPath);
constant!(ParamStringIsFilePath, kOfxParamStringIsFilePath);
constant!(ParamStringIsLabel, kOfxParamStringIsLabel);
constant!(ParamStringIsMultiLine, kOfxParamStringIsMultiLine);
constant!(ParamStringIsRichTextFormat, kOfxParamStringIsRichTextFormat);
constant!(ParamStringIsSingleLine, kOfxParamStringIsSingleLine);
constant!(ParamTypeBoolean, kOfxParamTypeBoolean);
constant!(ParamTypeChoice, kOfxParamTypeChoice);
constant!(ParamTypeCustom, kOfxParamTypeCustom);
constant!(ParamTypeDouble, kOfxParamTypeDouble);
constant!(ParamTypeDouble2D, kOfxParamTypeDouble2D);
constant!(ParamTypeDouble3D, kOfxParamTypeDouble3D);
constant!(ParamTypeGroup, kOfxParamTypeGroup);
constant!(ParamTypeInteger, kOfxParamTypeInteger);
constant!(ParamTypeInteger2D, kOfxParamTypeInteger2D);
constant!(ParamTypeInteger3D, kOfxParamTypeInteger3D);
constant!(ParamTypePage, kOfxParamTypePage);
constant!(ParamTypeParametric, kOfxParamTypeParametric);
constant!(ParamTypePushButton, kOfxParamTypePushButton);
constant!(ParamTypeRGB, kOfxParamTypeRGB);
constant!(ParamTypeRGBA, kOfxParamTypeRGBA);
constant!(ParamTypeStrChoice, kOfxParamTypeStrChoice);
constant!(ParamTypeString, kOfxParamTypeString);
constant!(ActionDescribeInteract, kOfxActionDescribeInteract);
constant!(
    ActionCreateInstanceInteract,
    kOfxActionCreateInstanceInteract
);
constant!(
    ActionDestroyInstanceInteract,
    kOfxActionDestroyInstanceInteract
);
constant!(InteractActionDraw, kOfxInteractActionDraw);
constant!(InteractActionGainFocus, kOfxInteractActionGainFocus);
constant!(InteractActionKeyDown, kOfxInteractActionKeyDown);
constant!(InteractActionKeyRepeat, kOfxInteractActionKeyRepeat);
constant!(InteractActionKeyUp, kOfxInteractActionKeyUp);
constant!(InteractActionLoseFocus, kOfxInteractActionLoseFocus);
constant!(InteractActionPenDown, kOfxInteractActionPenDown);
constant!(InteractActionPenMotion, kOfxInteractActionPenMotion);
constant!(InteractActionPenUp, kOfxInteractActionPenUp);
constant!(
    InteractPropBackgroundColour,
    kOfxInteractPropBackgroundColour
);
constant!(InteractPropBitDepth, kOfxInteractPropBitDepth);
constant!(InteractPropHasAlpha, kOfxInteractPropHasAlpha);
constant!(InteractPropPenPosition, kOfxInteractPropPenPosition);
constant!(InteractPropPenPressure, kOfxInteractPropPenPressure);
constant!(
    InteractPropPenViewportPosition,
    kOfxInteractPropPenViewportPosition
);
constant!(InteractPropPixelScale, kOfxInteractPropPixelScale);
constant!(InteractPropSlaveToParam, kOfxInteractPropSlaveToParam);
constant!(InteractPropSuggestedColour, kOfxInteractPropSuggestedColour);
constant!(MessageError, kOfxMessageError);
constant!(MessageFatal, kOfxMessageFatal);
constant!(MessageLog, kOfxMessageLog);
constant!(MessageMessage, kOfxMessageMessage);
constant!(MessageQuestion, kOfxMessageQuestion);
constant!(MessageWarning, kOfxMessageWarning);
constant!(BitDepthByte, kOfxBitDepthByte);
constant!(BitDepthFloat, kOfxBitDepthFloat);
constant!(BitDepthHalf, kOfxBitDepthHalf);
constant!(BitDepthNone, kOfxBitDepthNone);
constant!(BitDepthShort, kOfxBitDepthShort);
constant!(ChangePluginEdited, kOfxChangePluginEdited);
constant!(ChangeTime, kOfxChangeTime);
constant!(ChangeUserEdited, kOfxChangeUserEdited);
constant!(OpenGLPropPixelDepth, kOfxOpenGLPropPixelDepth);
constant!(PluginPropFilePath, kOfxPluginPropFilePath);
constant!(PluginPropParamPageOrder, kOfxPluginPropParamPageOrder);
constant!(TypeClip, kOfxTypeClip);
constant!(TypeImage, kOfxTypeImage);
constant!(TypeImageEffect, kOfxTypeImageEffect);
constant!(TypeImageEffectHost, kOfxTypeImageEffectHost);
constant!(TypeImageEffectInstance, kOfxTypeImageEffectInstance);
constant!(TypeParameter, kOfxTypeParameter);
constant!(TypeParameterInstance, kOfxTypeParameterInstance);
