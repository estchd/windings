#![cfg(feature = "unknown")]
#![allow(non_snake_case, dead_code)]
use winapi::shared::winerror::HRESULT;
use winapi::_core::ptr::{null_mut};

pub trait COMContainer<I: winapi::Interface> {
    fn from_ptr(interface_pointer: *mut I) -> Option<Self> where Self: std::marker::Sized;
}

pub struct IUnknownContainer {
    pub (crate) interface: *mut winapi::um::unknwnbase::IUnknown,
}

pub enum QueryInterfaceError {
    NoInterface,
    Nullptr,
    UnknownResult(HRESULT),
}

impl IUnknownContainer {
    // I Don't know if AddRef and Release should be pub because they are needed to use the api, until i get such information, those will remain private

    fn AddRef(&mut self) {
        unsafe {
            let reference = &*self.interface;
            reference.AddRef();
        }
    }

    fn Release(&mut self) {
        unsafe {
            let reference = &*self.interface;
            reference.Release();
        }
    }

    pub fn QueryInterface<InterfaceType: winapi::Interface, ContainerType: COMContainer<InterfaceType>>(&self) -> Result<ContainerType, QueryInterfaceError> {
        use winapi::ctypes::c_void;
        use winapi::shared::guiddef::{GUID,IID,REFIID};
        use winapi::shared::winerror::{S_OK, E_NOINTERFACE,E_POINTER};

        let guid: GUID = InterfaceType::uuidof();
        let iid: IID = IID::from(guid);
        let riid: REFIID = REFIID::from(&iid as *const IID);

        let mut p_interface: *mut c_void = null_mut();
        let pp_interface: *mut *mut c_void = &mut p_interface as *mut *mut c_void;

        let result: HRESULT;
        unsafe {
            let reference = &*self.interface;
            result = reference.QueryInterface(riid, pp_interface);
        }

        match result {
            S_OK => {},
            E_NOINTERFACE => return Err(QueryInterfaceError::NoInterface),
            E_POINTER => panic!("We passed a Nullptr, this shouldn't happen"),
            _ => return Err(QueryInterfaceError::UnknownResult(result))
        };

        if p_interface == null_mut() {
            return Err(QueryInterfaceError::Nullptr);
        }

        let p_interface: *mut InterfaceType = p_interface as *mut InterfaceType;
        return Ok(ContainerType::from_ptr(p_interface).unwrap());
    }
}

impl COMContainer<winapi::um::unknwnbase::IUnknown> for IUnknownContainer {
    fn from_ptr(interface_pointer: *mut winapi::um::unknwnbase::IUnknown) -> Option<IUnknownContainer> {
        if interface_pointer == null_mut() {
            return None;
        };

        return Some(IUnknownContainer {
            interface: interface_pointer,
        });
    }
}

impl Drop for IUnknownContainer {
    fn drop(&mut self) {
        self.Release();
    }
}

pub struct IClassFactoryContainer {
    parent: IUnknownContainer,
    interface: *mut winapi::um::unknwnbase::IClassFactory,
}

impl COMContainer<winapi::um::unknwnbase::IClassFactory> for IClassFactoryContainer {
    fn from_ptr(interface_pointer: *mut winapi::um::unknwnbase::IClassFactory) -> Option<Self> {
        if interface_pointer == null_mut() {
            return None;
        }
        let ptr =  interface_pointer as *mut winapi::um::unknwnbase::IUnknown;
        let parent = match IUnknownContainer::from_ptr(ptr) {
            None => return None,
            Some(parent) => parent,
        };
        
        return Some(IClassFactoryContainer {
            parent,
            interface: interface_pointer,
        });
    }
}

impl IClassFactoryContainer {
    //TODO: Have to Check if this should be mut self
    pub fn CreateInstance<InterfaceType: winapi::Interface>(&self, unknown_outer: Option<IUnknownContainer>) {
        use winapi::ctypes::c_void;
        use winapi::shared::guiddef::{GUID,IID,REFIID};

        let unknown_ptr = match unknown_outer {
            None => {null_mut()},
            Some(container) => {container.interface},
        };

        let guid: GUID = InterfaceType::uuidof();
        let iid: IID = IID::from(guid);
        let riid: REFIID = REFIID::from(&iid as *const IID);

        let mut p_object: *mut c_void = null_mut();
        let pp_object: *mut *mut c_void = &mut p_object as *mut *mut c_void;

        let result: HRESULT;
        unsafe {
            let reference = &*self.interface;
            result = reference.CreateInstance(unknown_ptr, riid, pp_object);
        }

    }

    pub fn LockServer() {

    }
}