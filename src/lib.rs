mod Shadybank;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn shadybank_get_client() -> *mut Shadybank::Client {
    Box::into_raw(Box::new(Shadybank::Client::new()))
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_login(client_handle: *mut Shadybank::Client, account_id: *const c_char, password: *const c_char) -> i32 {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {
        return -1;
    };

    let account_id = if let Some(x) = account_id.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return -2;
        }
    } else {
        return -1;
    };

    let password = if let Some(x) = password.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return -2;
        }
    } else {
        return -1;
    };

    if client_handle.login(account_id, password).is_err() {
        return -1;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_logout(client_handle: *mut Shadybank::Client) -> i32 {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {

        return -1;
    };

    if client_handle.logout().is_err() {
        return -1;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_credit(client_handle: *mut Shadybank::Client, magstripe: *const c_char, amount: i32) -> i32 {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {
        return -1;
    };

    let magstripe = if let Some(x) = magstripe.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return -2;
        }
    } else {
        return -1;
    };

    if client_handle.credit(magstripe, amount).is_err() {
        return -1;
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_authorize(client_handle: *mut Shadybank::Client, magstripe: *const c_char, amount: i32) -> *mut c_char {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {
        return ptr::null_mut();
    };

    let magstripe = if let Some(x) = magstripe.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return ptr::null_mut();
        }
    } else {
        return ptr::null_mut();
    };

    if let Ok(x) = client_handle.authorize(magstripe, amount) {
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
pub unsafe extern "C" fn shadybank_void(client_handle: *mut Shadybank::Client, auth_code: *const c_char) -> i32 {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {
        return -1;
    };

    let auth_code = if let Some(x) = auth_code.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return -1;
        }
    } else {
        return -1;
    };

    if client_handle.void(auth_code).is_err() {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn shadybank_capture(client_handle: *mut Shadybank::Client, amount: i32, auth_code: *const c_char) -> i32 {
    let client_handle = if let Some(x) = client_handle.as_mut() {
        x
    } else {
        return -1;
    };

    let auth_code = if let Some(x) = auth_code.as_ref() {
        if let Ok(y) = CStr::from_ptr(x).to_str() {
            y
        } else {
            return -1;
        }
    } else {
        return -1;
    };

    if client_handle.capture(amount, auth_code).is_err() {
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
