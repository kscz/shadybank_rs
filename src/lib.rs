mod shadybank;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn shadybank_get_client(shadybank_url: *const c_char) -> *mut shadybank::Client {
    let url = if shadybank_url.is_null() {
        None
    } else {
        if let Ok(y) = CStr::from_ptr(shadybank_url).to_str() {
            Some(String::from(y))
        } else {
            return ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(shadybank::Client::new(url)))
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_login(client_handle: Option<&mut shadybank::Client>, account_id: *const c_char, password: *const c_char) -> i32 {
    let Some(client) = client_handle else {
        return -1;
    };

    if account_id.is_null() || password.is_null() {
        return -1;
    }

    let Ok(account_id) = CStr::from_ptr(account_id).to_str() else {
        return -2;
    };

    let Ok(password) = CStr::from_ptr(password).to_str() else {
        return -2;
    };

    if client.login(account_id, password).is_err() {
        return -3;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_logout(client_handle: Option<&mut shadybank::Client>) -> i32 {
    let Some(client) = client_handle else {
        return -1;
    };

    if client.logout().is_err() {
        return -1;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_credit(client_handle: Option<&mut shadybank::Client>, magstripe: *const c_char, amount: i32) -> i32 {
    let Some(client) = client_handle else {
        return -1;
    };

    if magstripe.is_null() {
        return -1;
    }
    let magstripe = if let Ok(y) = CStr::from_ptr(magstripe).to_str() {
        String::from(y)
    } else {
        return -2;
    };

    if client.credit(&shadybank::MagData::Stripe(magstripe), amount).is_err() {
        return -1;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_authorize_stripe(client_handle: Option<&mut shadybank::Client>, magstripe: *const c_char, amount: i32) -> *mut c_char {
    let Some(client) = client_handle else {
        return ptr::null_mut();
    };

    if magstripe.is_null() {
        return ptr::null_mut();
    }

    let magstripe = if let Ok(y) = CStr::from_ptr(magstripe).to_str() {
        String::from(y)
    } else {
        return ptr::null_mut();
    };

    if let Ok(x) = client.authorize(&shadybank::MagData::Stripe(magstripe), amount) {
        if let Ok(cs) = CString::new(x) {
            cs.into_raw()
        } else {
            ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_authorize_pan_shotp(client_handle: Option<&mut shadybank::Client>, pan: *const c_char, shotp: *const c_char, amount: i32) -> *mut c_char {
    let Some(client) = client_handle else {
        return ptr::null_mut();
    };

    if pan.is_null() || shotp.is_null(){
        return ptr::null_mut();
    }
    let pan = if let Ok(y) = CStr::from_ptr(pan).to_str() {
        String::from(y)
    } else {
        return ptr::null_mut();
    };
    let shotp = if let Ok(y) = CStr::from_ptr(shotp).to_str() {
        String::from(y)
    } else {
        return ptr::null_mut();
    };

    if let Ok(x) = client.authorize(&shadybank::MagData::PanShotp((pan, shotp)), amount) {
        if let Ok(cs) = CString::new(x) {
            cs.into_raw()
        } else {
            ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_void(client_handle: Option<&mut shadybank::Client>, auth_code: *const c_char) -> i32 {
    let Some(client) = client_handle else {
        return -1;
    };

    if auth_code.is_null() {
        return -1;
    }

    let Ok(auth_code) = CStr::from_ptr(auth_code).to_str() else {
        return -1;
    };

    if client.void(auth_code).is_err() {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_capture(client_handle: Option<&mut shadybank::Client>, amount: i32, auth_code: *const c_char) -> i32 {
    let Some(client) = client_handle else {
        return -1;
    };

    if auth_code.is_null() {
        return -1;
    }

    let Ok(auth_code) = CStr::from_ptr(auth_code).to_str() else {
        return -1;
    };

    if client.capture(amount, auth_code).is_err() {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
    }
}
