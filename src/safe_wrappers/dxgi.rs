#![cfg(feature = "dxgi")]
#![allow(non_snake_case,non_camel_case_types)]
use crate::unknown::COMContainer;

//In the Documentation, there is a fourth Enum Member DXGI_ADAPTER_FLAG_FORCE_DWORD,
// but since it is directly specified as not used and it is not defined as a constant in WINAPI,
// i have decided to omit that member for the moment
CONST_TO_ENUM!(const_enum DXGI_ADAPTER_FLAG, winapi::shared::dxgi::DXGI_ADAPTER_FLAG {
    DXGI_ADAPTER_FLAG_NONE = winapi::shared::dxgi::DXGI_ADAPTER_FLAG_NONE,
    DXGI_ADAPTER_FLAG_REMOTE = winapi::shared::dxgi::DXGI_ADAPTER_FLAG_REMOTE,
    DXGI_ADAPTER_FLAG_SOFTWARE = winapi::shared::dxgi::DXGI_ADAPTER_FLAG_SOFTWARE,
});

CONST_TO_ENUM!(const_enum DXGI_RESIDENCY, winapi::shared::dxgi::DXGI_RESIDENCY {
    DXGI_RESIDENCY_FULLY_RESIDENT = winapi::shared::dxgi::DXGI_RESIDENCY_FULLY_RESIDENT,
    DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = winapi::shared::dxgi::DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY,
    DXGI_RESIDENCY_EVICTED_TO_DISK = winapi::shared::dxgi::DXGI_RESIDENCY_EVICTED_TO_DISK,
});

//This is not defined as a constant in Winapi, but appears to be a valid Option, so we define it here so we have it as a constant
const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: winapi::shared::dxgi::DXGI_ADAPTER_FLAG = 12;

CONST_TO_ENUM!(const_enum DXGI_SWAP_CHAIN_FLAG, winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG {
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_NONPREROTATED,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT,
    DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER,
    DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO,
    DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO,
    DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING = winapi::shared::dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS,
});

CONST_TO_ENUM!(const_enum DXGI_SWAP_EFFECT, winapi::shared::dxgi::DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD = winapi::shared::dxgi::DXGI_SWAP_EFFECT_DISCARD,
    DXGI_SWAP_EFFECT_FLIP_DISCARD = winapi::shared::dxgi::DXGI_SWAP_EFFECT_FLIP_DISCARD,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = winapi::shared::dxgi::DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
    DXGI_SWAP_EFFECT_SEQUENTIAL = winapi::shared::dxgi::DXGI_SWAP_EFFECT_SEQUENTIAL,
});

pub trait DXGI_FACTORY: winapi::Interface {}

impl DXGI_FACTORY for winapi::shared::dxgi::IDXGIFactory {}
impl DXGI_FACTORY for winapi::shared::dxgi::IDXGIFactory1 {}
impl DXGI_FACTORY for winapi::shared::dxgi1_2::IDXGIFactory2 {}
impl DXGI_FACTORY for winapi::shared::dxgi1_3::IDXGIFactory3 {}
impl DXGI_FACTORY for winapi::shared::dxgi1_4::IDXGIFactory4 {}
impl DXGI_FACTORY for winapi::shared::dxgi1_5::IDXGIFactory5 {}
impl DXGI_FACTORY for winapi::shared::dxgi1_6::IDXGIFactory6 {}

pub enum CreateFactoryError {
    Nullptr,
    Result(winapi::shared::winerror::HRESULT)
}

pub fn CreateDXGIFactory<FactoryType: DXGI_FACTORY, ContainerType: COMContainer<FactoryType>>() -> Result<ContainerType,CreateFactoryError> {
    use winapi::ctypes::c_void;
    use winapi::shared::guiddef::{GUID,IID,REFIID};
    use winapi::shared::winerror::{HRESULT,S_OK};
    use winapi::_core::ptr::{null_mut};

    let guid: GUID = FactoryType::uuidof();
    let iid: IID = IID::from(guid);
    let riid: REFIID = REFIID::from(&iid as *const IID);

    let mut p_factory: *mut c_void = null_mut();
    let pp_factory: *mut *mut c_void = &mut p_factory as *mut *mut c_void;

    let result: HRESULT;
    unsafe {
        result = winapi::shared::dxgi::CreateDXGIFactory(riid, pp_factory);
    }

    if result != S_OK
    {
        return Err(CreateFactoryError::Result(result));
    }
    if p_factory == null_mut() {
        return Err(CreateFactoryError::Nullptr);
    }

    let p_factory: *mut FactoryType = p_factory as *mut FactoryType;
    return Ok(ContainerType::from_ptr(p_factory).unwrap());
}

pub fn CreateDXGIFactory1<FactoryType: DXGI_FACTORY, ContainerType: COMContainer<FactoryType>>() -> Result<ContainerType, CreateFactoryError>{
    use winapi::ctypes::c_void;
    use winapi::shared::guiddef::{GUID,IID,REFIID};
    use winapi::shared::winerror::{HRESULT,S_OK};
    use winapi::_core::ptr::{null_mut};

    let guid: GUID = FactoryType::uuidof();
    let iid: IID = IID::from(guid);
    let riid: REFIID = REFIID::from(&iid as *const IID);

    let mut p_factory: *mut c_void = null_mut();
    let pp_factory: *mut *mut c_void = &mut p_factory as *mut *mut c_void;

    let result: HRESULT;
    unsafe {
        result = winapi::shared::dxgi::CreateDXGIFactory1(riid, pp_factory);
    }

    match result {
        S_OK => {},
        _ => return Err(CreateFactoryError::Result(result)),
    };

    if p_factory == null_mut() {
        return Err(CreateFactoryError::Nullptr);
    }

    let p_factory: *mut FactoryType = p_factory as *mut FactoryType;
    return Ok(ContainerType::from_ptr(p_factory).unwrap());

}