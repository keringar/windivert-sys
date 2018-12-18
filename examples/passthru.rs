use windivert_sys::*;

use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::winerror::{ERROR_INVALID_PARAMETER, ERROR_ACCESS_DENIED, ERROR_FILE_NOT_FOUND};
use winapi::shared::minwindef::FALSE;

use std::ffi::CString;

fn main() {
    // Open a handle to the packet diverter driver
    let packet_filter = CString::new("true").unwrap();
    let handle = unsafe { WinDivertOpen(packet_filter.as_ptr(), WINDIVERT_LAYER_NETWORK, 0, 0) };
    if handle == INVALID_HANDLE_VALUE {
        let code = unsafe { GetLastError() };
        match code {
            ERROR_INVALID_PARAMETER => eprintln!("error: filter syntax error"),
            ERROR_ACCESS_DENIED => eprintln!("error: insufficient permissions"),
            ERROR_FILE_NOT_FOUND => eprintln!("error: valid driver file not found in executable's directory"),
            _ => eprintln!("error: failed to open the WinDivert device {}", code),
        }

        std::process::exit(-1);
    }

    // Do some packet stuff
    loop {
        break;
    }

    // Close the handle and remove the driver
    let return_value = unsafe { WinDivertClose(handle) };
    if return_value == FALSE {
        eprintln!("error: failed to close the WinDivert device {}", unsafe { GetLastError() });
        std::process::exit(-1);
    }
}