use std::ffi::c_void;
use std::sync::Once;

#[repr(C)]
#[allow(non_snake_case)]
pub struct VersionExports {
    pub GetFileVersionInfoA: usize,
    pub GetFileVersionInfoByHandle: usize,
    pub GetFileVersionInfoExA: usize,
    pub GetFileVersionInfoExW: usize,
    pub GetFileVersionInfoSizeA: usize,
    pub GetFileVersionInfoSizeExA: usize,
    pub GetFileVersionInfoSizeExW: usize,
    pub GetFileVersionInfoSizeW: usize,
    pub GetFileVersionInfoW: usize,
    pub VerFindFileA: usize,
    pub VerFindFileW: usize,
    pub VerInstallFileA: usize,
    pub VerInstallFileW: usize,
    pub VerLanguageNameA: usize,
    pub VerLanguageNameW: usize,
    pub VerQueryValueA: usize,
    pub VerQueryValueW: usize,
}

static INIT: Once = Once::new();
static mut PROXY: VersionExports = VersionExports {
    GetFileVersionInfoA: 0,
    GetFileVersionInfoByHandle: 0,
    GetFileVersionInfoExA: 0,
    GetFileVersionInfoExW: 0,
    GetFileVersionInfoSizeA: 0,
    GetFileVersionInfoSizeExA: 0,
    GetFileVersionInfoSizeExW: 0,
    GetFileVersionInfoSizeW: 0,
    GetFileVersionInfoW: 0,
    VerFindFileA: 0,
    VerFindFileW: 0,
    VerInstallFileA: 0,
    VerInstallFileW: 0,
    VerLanguageNameA: 0,
    VerLanguageNameW: 0,
    VerQueryValueA: 0,
    VerQueryValueW: 0,
};

extern "system" {
    fn GetSystemDirectoryW(lpBuffer: *mut u16, uSize: u32) -> u32;
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> usize;
}

pub fn init_proxy() {
    INIT.call_once(|| unsafe {
        let mut path = [0u16; 260];
        let len = GetSystemDirectoryW(path.as_mut_ptr(), 260) as usize;
        let suffix = [
            '\\' as u16, 'v' as u16, 'e' as u16, 'r' as u16, 's' as u16, 'i' as u16, 'o' as u16, 'n' as u16,
            '.' as u16, 'd' as u16, 'l' as u16, 'l' as u16, 0u16,
        ];
        for (i, &c) in suffix.iter().enumerate() {
            if len + i < path.len() {
                path[len + i] = c;
            }
        }
        let handle = LoadLibraryW(path.as_ptr());
        if handle.is_null() {
            return;
        }

        PROXY.GetFileVersionInfoA = GetProcAddress(handle, b"GetFileVersionInfoA\0".as_ptr());
        PROXY.GetFileVersionInfoByHandle = GetProcAddress(handle, b"GetFileVersionInfoByHandle\0".as_ptr());
        PROXY.GetFileVersionInfoExA = GetProcAddress(handle, b"GetFileVersionInfoExA\0".as_ptr());
        PROXY.GetFileVersionInfoExW = GetProcAddress(handle, b"GetFileVersionInfoExW\0".as_ptr());
        PROXY.GetFileVersionInfoSizeA = GetProcAddress(handle, b"GetFileVersionInfoSizeA\0".as_ptr());
        PROXY.GetFileVersionInfoSizeExA = GetProcAddress(handle, b"GetFileVersionInfoSizeExA\0".as_ptr());
        PROXY.GetFileVersionInfoSizeExW = GetProcAddress(handle, b"GetFileVersionInfoSizeExW\0".as_ptr());
        PROXY.GetFileVersionInfoSizeW = GetProcAddress(handle, b"GetFileVersionInfoSizeW\0".as_ptr());
        PROXY.GetFileVersionInfoW = GetProcAddress(handle, b"GetFileVersionInfoW\0".as_ptr());
        PROXY.VerFindFileA = GetProcAddress(handle, b"VerFindFileA\0".as_ptr());
        PROXY.VerFindFileW = GetProcAddress(handle, b"VerFindFileW\0".as_ptr());
        PROXY.VerInstallFileA = GetProcAddress(handle, b"VerInstallFileA\0".as_ptr());
        PROXY.VerInstallFileW = GetProcAddress(handle, b"VerInstallFileW\0".as_ptr());
        PROXY.VerLanguageNameA = GetProcAddress(handle, b"VerLanguageNameA\0".as_ptr());
        PROXY.VerLanguageNameW = GetProcAddress(handle, b"VerLanguageNameW\0".as_ptr());
        PROXY.VerQueryValueA = GetProcAddress(handle, b"VerQueryValueA\0".as_ptr());
        PROXY.VerQueryValueW = GetProcAddress(handle, b"VerQueryValueW\0".as_ptr());
    });
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoA(
    filename: *const u8,
    handle: u32,
    len: u32,
    data: *mut c_void,
) -> i32 {
    init_proxy();
    if PROXY.GetFileVersionInfoA != 0 {
        let f: unsafe extern "system" fn(*const u8, u32, u32, *mut c_void) -> i32 =
            std::mem::transmute(PROXY.GetFileVersionInfoA);
        f(filename, handle, len, data)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoByHandle(
    hmem: u32,
    filename: *const u16,
    v2: *mut c_void,
    v3: u32,
) -> i32 {
    init_proxy();
    if PROXY.GetFileVersionInfoByHandle != 0 {
        let f: unsafe extern "system" fn(u32, *const u16, *mut c_void, u32) -> i32 =
            std::mem::transmute(PROXY.GetFileVersionInfoByHandle);
        f(hmem, filename, v2, v3)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoExA(
    flags: u32,
    filename: *const u8,
    handle: u32,
    len: u32,
    data: *mut c_void,
) -> i32 {
    init_proxy();
    if PROXY.GetFileVersionInfoExA != 0 {
        let f: unsafe extern "system" fn(u32, *const u8, u32, u32, *mut c_void) -> i32 =
            std::mem::transmute(PROXY.GetFileVersionInfoExA);
        f(flags, filename, handle, len, data)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoExW(
    flags: u32,
    filename: *const u16,
    handle: u32,
    len: u32,
    data: *mut c_void,
) -> i32 {
    init_proxy();
    if PROXY.GetFileVersionInfoExW != 0 {
        let f: unsafe extern "system" fn(u32, *const u16, u32, u32, *mut c_void) -> i32 =
            std::mem::transmute(PROXY.GetFileVersionInfoExW);
        f(flags, filename, handle, len, data)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoSizeA(
    filename: *const u8,
    handle: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.GetFileVersionInfoSizeA != 0 {
        let f: unsafe extern "system" fn(*const u8, *mut u32) -> u32 =
            std::mem::transmute(PROXY.GetFileVersionInfoSizeA);
        f(filename, handle)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoSizeExA(
    flags: u32,
    filename: *const u8,
    handle: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.GetFileVersionInfoSizeExA != 0 {
        let f: unsafe extern "system" fn(u32, *const u8, *mut u32) -> u32 =
            std::mem::transmute(PROXY.GetFileVersionInfoSizeExA);
        f(flags, filename, handle)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoSizeExW(
    flags: u32,
    filename: *const u16,
    handle: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.GetFileVersionInfoSizeExW != 0 {
        let f: unsafe extern "system" fn(u32, *const u16, *mut u32) -> u32 =
            std::mem::transmute(PROXY.GetFileVersionInfoSizeExW);
        f(flags, filename, handle)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoSizeW(
    filename: *const u16,
    handle: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.GetFileVersionInfoSizeW != 0 {
        let f: unsafe extern "system" fn(*const u16, *mut u32) -> u32 =
            std::mem::transmute(PROXY.GetFileVersionInfoSizeW);
        f(filename, handle)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFileVersionInfoW(
    filename: *const u16,
    handle: u32,
    len: u32,
    data: *mut c_void,
) -> i32 {
    init_proxy();
    if PROXY.GetFileVersionInfoW != 0 {
        let f: unsafe extern "system" fn(*const u16, u32, u32, *mut c_void) -> i32 =
            std::mem::transmute(PROXY.GetFileVersionInfoW);
        f(filename, handle, len, data)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerFindFileA(
    flags: u32,
    filename: *const u8,
    windir: *const u8,
    appdir: *const u8,
    curdir: *mut u8,
    curdirlen: *mut u32,
    destdir: *mut u8,
    destdirlen: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.VerFindFileA != 0 {
        let f: unsafe extern "system" fn(
            u32,
            *const u8,
            *const u8,
            *const u8,
            *mut u8,
            *mut u32,
            *mut u8,
            *mut u32,
        ) -> u32 = std::mem::transmute(PROXY.VerFindFileA);
        f(flags, filename, windir, appdir, curdir, curdirlen, destdir, destdirlen)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerFindFileW(
    flags: u32,
    filename: *const u16,
    windir: *const u16,
    appdir: *const u16,
    curdir: *mut u16,
    curdirlen: *mut u32,
    destdir: *mut u16,
    destdirlen: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.VerFindFileW != 0 {
        let f: unsafe extern "system" fn(
            u32,
            *const u16,
            *const u16,
            *const u16,
            *mut u16,
            *mut u32,
            *mut u16,
            *mut u32,
        ) -> u32 = std::mem::transmute(PROXY.VerFindFileW);
        f(flags, filename, windir, appdir, curdir, curdirlen, destdir, destdirlen)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerInstallFileA(
    flags: u32,
    srcfilename: *const u8,
    destfilename: *const u8,
    srcdir: *const u8,
    destdir: *const u8,
    curdir: *const u8,
    tmpfile: *mut u8,
    tmpfilelen: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.VerInstallFileA != 0 {
        let f: unsafe extern "system" fn(
            u32,
            *const u8,
            *const u8,
            *const u8,
            *const u8,
            *const u8,
            *mut u8,
            *mut u32,
        ) -> u32 = std::mem::transmute(PROXY.VerInstallFileA);
        f(
            flags,
            srcfilename,
            destfilename,
            srcdir,
            destdir,
            curdir,
            tmpfile,
            tmpfilelen,
        )
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerInstallFileW(
    flags: u32,
    srcfilename: *const u16,
    destfilename: *const u16,
    srcdir: *const u16,
    destdir: *const u16,
    curdir: *const u16,
    tmpfile: *mut u16,
    tmpfilelen: *mut u32,
) -> u32 {
    init_proxy();
    if PROXY.VerInstallFileW != 0 {
        let f: unsafe extern "system" fn(
            u32,
            *const u16,
            *const u16,
            *const u16,
            *const u16,
            *const u16,
            *mut u16,
            *mut u32,
        ) -> u32 = std::mem::transmute(PROXY.VerInstallFileW);
        f(
            flags,
            srcfilename,
            destfilename,
            srcdir,
            destdir,
            curdir,
            tmpfile,
            tmpfilelen,
        )
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerLanguageNameA(
    lang: u32,
    szlang: *mut u8,
    cchlang: u32,
) -> u32 {
    init_proxy();
    if PROXY.VerLanguageNameA != 0 {
        let f: unsafe extern "system" fn(u32, *mut u8, u32) -> u32 =
            std::mem::transmute(PROXY.VerLanguageNameA);
        f(lang, szlang, cchlang)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerLanguageNameW(
    lang: u32,
    szlang: *mut u16,
    cchlang: u32,
) -> u32 {
    init_proxy();
    if PROXY.VerLanguageNameW != 0 {
        let f: unsafe extern "system" fn(u32, *mut u16, u32) -> u32 =
            std::mem::transmute(PROXY.VerLanguageNameW);
        f(lang, szlang, cchlang)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerQueryValueA(
    block: *const c_void,
    subblock: *const u8,
    buffer: *mut *mut c_void,
    len: *mut u32,
) -> i32 {
    init_proxy();
    if PROXY.VerQueryValueA != 0 {
        let f: unsafe extern "system" fn(*const c_void, *const u8, *mut *mut c_void, *mut u32) -> i32 =
            std::mem::transmute(PROXY.VerQueryValueA);
        f(block, subblock, buffer, len)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "system" fn VerQueryValueW(
    block: *const c_void,
    subblock: *const u16,
    buffer: *mut *mut c_void,
    len: *mut u32,
) -> i32 {
    init_proxy();
    if PROXY.VerQueryValueW != 0 {
        let f: unsafe extern "system" fn(*const c_void, *const u16, *mut *mut c_void, *mut u32) -> i32 =
            std::mem::transmute(PROXY.VerQueryValueW);
        f(block, subblock, buffer, len)
    } else {
        0
    }
}
