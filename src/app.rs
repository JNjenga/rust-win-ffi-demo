mod winapi;
use winapi::*;

use std::ffi::CString;

fn main()
{
//     int APIENTRY _tWinMain(
//   _In_ HINSTANCE hInstance,
//   _In_ HINSTANCE hPrevInstance,
//   _In_ LPTSTR     lpCmdLine,
//   _In_ int       nCmdShow

    unsafe
    {
        let hInstance = GetModuleHandle(std::ptr::null());
        let lpCmdLine = GetCommandLine();
        println!("{:?}", CString::from_raw(lpCmdLine));

        let lpText    = CString::new("Windows from rust!").unwrap();
        let lpCaption = CString::new("This is the caption").unwrap();

        MessageBox( std::ptr::null(),
                    lpText.as_ptr(),
                    lpCaption.as_ptr(),
                    MB_OK);
    }
}
