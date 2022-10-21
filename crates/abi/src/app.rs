//! Applications

use super::*;

use yxy::AppHandler;

#[repr(C)]
#[derive(Destruct)]
pub struct RoomInfo {
    pub area_id: *mut c_char,
    pub building_code: *mut c_char,
    pub floor_code: *mut c_char,
    pub room_code: *mut c_char,
}

extern_c_destructor!(RoomInfo);

impl From<yxy::RoomInfo> for RoomInfo {
    fn from(info: yxy::RoomInfo) -> Self {
        Self {
            area_id: CString::new(info.area_id).unwrap_or_default().into_raw(),
            building_code: CString::new(info.building_code)
                .unwrap_or_default()
                .into_raw(),
            floor_code: CString::new(info.floor_code).unwrap_or_default().into_raw(),
            room_code: CString::new(info.room_code).unwrap_or_default().into_raw(),
        }
    }
}

impl From<&RoomInfo> for yxy::RoomInfo {
    /// Unsafe implementation [`From<RoomInfo>`] for [`yxy::RoomInfo`]
    fn from(info: &RoomInfo) -> Self {
        unsafe {
            Self {
                area_id: copy_c_string_into_string(info.area_id),
                building_code: copy_c_string_into_string(info.building_code),
                floor_code: copy_c_string_into_string(info.floor_code),
                room_code: copy_c_string_into_string(info.room_code),
            }
        }
    }
}

/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn query_ele_bind(handler: *const AppHandler) -> *mut RoomInfo {
    check_null_return_null!(handler);
    let handler = &*handler;

    match handler.query_electricity_binding() {
        Ok(bind) => {
            let room = yxy::RoomInfo::from(bind);
            Box::into_raw(Box::new(RoomInfo::from(room)))
        }
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

#[repr(C)]
#[derive(Destruct)]
pub struct ElectricityInfo {
    pub area_id: *mut c_char,
    pub building_code: *mut c_char,
    pub floor_code: *mut c_char,
    pub room_code: *mut c_char,
    pub display_room_name: *mut c_char,
    pub room_status: *mut c_char,

    pub total_surplus: c_float,
    pub total_amount: c_float,
    pub surplus: c_float,
    pub surplus_amount: c_float,
    pub subsidy: c_float,
    pub subsidy_amount: c_float,
}

extern_c_destructor!(ElectricityInfo);

impl TryFrom<yxy::ElectricityInfo> for ElectricityInfo {
    type Error = ();

    fn try_from(mut info: yxy::ElectricityInfo) -> Result<Self, Self::Error> {
        if info.surplus_list.is_empty() {
            return Err(());
        }
        let surplus = info.surplus_list.swap_remove(0);

        Ok(Self {
            total_surplus: info.soc,
            total_amount: info.total_soc_amount,
            surplus: surplus.surplus,
            surplus_amount: surplus.amount,
            subsidy: surplus.subsidy,
            subsidy_amount: surplus.subsidy_amount,

            area_id: CString::new(info.area_id).unwrap_or_default().into_raw(),
            building_code: CString::new(info.building_code)
                .unwrap_or_default()
                .into_raw(),
            floor_code: CString::new(info.floor_code).unwrap_or_default().into_raw(),
            room_code: CString::new(info.room_code).unwrap_or_default().into_raw(),
            display_room_name: CString::new(info.display_room_name)
                .unwrap_or_default()
                .into_raw(),
            room_status: CString::new(surplus.room_status)
                .unwrap_or_default()
                .into_raw(),
        })
    }
}

/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn query_ele(handler: *const AppHandler) -> *mut ElectricityInfo {
    check_null_return_null!(handler);
    let handler = &*handler;

    // Get RoomInfo by default binding
    let bind = match handler.query_electricity_binding() {
        Ok(bind) => bind,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };
    let room = yxy::RoomInfo::from(bind);

    match handler.query_electricity(&room) {
        Ok(info) => match ElectricityInfo::try_from(info) {
            Ok(info) => Box::into_raw(Box::new(info)),
            Err(_) => std::ptr::null_mut(),
        },
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn query_ele_by_room_info(
    handler: *const AppHandler,
    info: *const RoomInfo,
) -> *mut ElectricityInfo {
    check_null_return_null!(handler, info);
    let handler = &*handler;
    let room = yxy::RoomInfo::from(&*info);

    match handler.query_electricity(&room) {
        Ok(info) => match ElectricityInfo::try_from(info) {
            Ok(info) => Box::into_raw(Box::new(info)),
            Err(_) => std::ptr::null_mut(),
        },
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}
