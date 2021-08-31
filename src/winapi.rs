pub type PVOID     = *const std::ffi::c_void;
pub type HANDLE    = PVOID;
pub type HWND      = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMODULE   = HINSTANCE;
pub type LPWSTR    = *const std::os::raw::c_char;
pub type LPTSTR    = LPWSTR;
pub type LPCSTR    = *const std::os::raw::c_char;
pub type LPSTR     = *mut i8;
pub type UINT      = usize;

pub const MB_OK : usize = 0x00000000;

extern
{

    #[link_name="GetModuleHandleA"]
    pub fn GetModuleHandle(_: LPCSTR) -> HMODULE;
    #[link_name="GetCommandLineA"]
    pub fn GetCommandLine() -> LPSTR;
    #[link_name="MessageBoxA"]
    pub fn MessageBox(hWnd :HWND, lpText :LPCSTR, lpCaption : LPCSTR, uType : UINT) -> isize;
}
