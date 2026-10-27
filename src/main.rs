#![windows_subsystem = "windows"]

use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Security::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::SystemInformation::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::Shell::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

// ==============================================================================
// 1. TÀI NGUYÊN, ĐỊNH DANH & HẰNG SỐ (ZERO-ALLOCATION)
// ==============================================================================
pub mod id {
    pub const IDM_CLEAN: usize = 1001;
    pub const IDM_AUTOREDUCT_TOGGLE: usize = 1002;
    pub const IDM_ABOUT: usize = 1003;
    pub const IDM_EXIT: usize = 1004;

    // Phân vùng dọn dẹp
    pub const REDUCT_WORKINGSET: u32 = 0x01;
    pub const REDUCT_SYSTEMFILECACHE: u32 = 0x02;
    pub const REDUCT_STANDBYPRIORITY0LIST: u32 = 0x04;
    pub const REDUCT_STANDBYLIST: u32 = 0x08;
    pub const REDUCT_MODIFIEDLIST: u32 = 0x10;
    pub const REDUCT_COMBINEMEMORYLISTS: u32 = 0x20;
    pub const REDUCT_REGISTRYCACHE: u32 = 0x40;

    pub const REDUCT_MASK_ALL: u32 = REDUCT_WORKINGSET
        | REDUCT_SYSTEMFILECACHE
        | REDUCT_STANDBYPRIORITY0LIST
        | REDUCT_STANDBYLIST
        | REDUCT_MODIFIEDLIST
        | REDUCT_COMBINEMEMORYLISTS
        | REDUCT_REGISTRYCACHE;
}

pub const APP_NAME: &str = "Mem Reduct";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const TIMER_ID: usize = 1337;
pub const HOTKEY_ID: i32 = 101;

// Màu sắc khay hệ thống (RGB)
pub const COLOR_TEXT: u32 = 0x00FFFFFF;      // Trắng
pub const COLOR_BG_NORMAL: u32 = 0x00408000; // Xanh lục dịu
pub const COLOR_BG_WARN: u32 = 0x000080FF;   // Cam cảnh báo
pub const COLOR_BG_DANGER: u32 = 0x00241CEC; // Đỏ nguy cấp

// ==============================================================================
// 2. NATIVE NT API ĐỘC QUYỀN (NTDLL SYSTEM CALLS)
// ==============================================================================
type NtSetSystemInformationFn = unsafe extern "system" fn(
    info_class: u32,
    info: *const c_void,
    length: u32,
) -> i32;

const SYSTEM_FILECACHE_INFORMATION_EX: u32 = 21;
const SYSTEM_MEMORY_LIST_INFORMATION: u32 = 80;
const SYSTEM_COMBINE_PHYSICAL_MEMORY_INFORMATION: u32 = 130;
const SYSTEM_REGISTRY_RECONCILIATION_INFORMATION: u32 = 135;

const MEMORY_EMPTY_WORKING_SETS: u32 = 2;
const MEMORY_FLUSH_MODIFIED_LIST: u32 = 3;
const MEMORY_PURGE_STANDBY_LIST: u32 = 4;
const MEMORY_PURGE_LOW_PRIORITY_STANDBY_LIST: u32 = 5;

#[repr(C)]
struct SystemFilecacheInformation {
    current_size: usize,
    peak_size: usize,
    page_fault_count: u32,
    minimum_working_set: usize,
    maximum_working_set: usize,
    unused: [usize; 4],
}

/// Tự động bật quyền SeIncreaseQuotaPrivilege, SeProfileSingleProcessPrivilege và SeDebugPrivilege
unsafe fn enable_privileges() {
    let mut token: HANDLE = null_mut();
    if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) != 0 {
        let priv_names = [
            windows_sys::core::w!("SeIncreaseQuotaPrivilege"),
            windows_sys::core::w!("SeProfileSingleProcessPrivilege"),
            windows_sys::core::w!("SeDebugPrivilege"),
        ];

        for priv_name in priv_names {
            let mut luid: LUID = zeroed();
            if LookupPrivilegeValueW(null(), priv_name, &mut luid) != 0 {
                let mut tp: TOKEN_PRIVILEGES = zeroed();
                tp.PrivilegeCount = 1;
                tp.Privileges[0].Luid = luid;
                tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

                AdjustTokenPrivileges(
                    token,
                    0,
                    &tp,
                    size_of::<TOKEN_PRIVILEGES>() as u32,
                    null_mut(),
                    null_mut(),
                );
            }
        }
        CloseHandle(token);
    }
}

/// Dọn dẹp các phân vùng bộ nhớ được chỉ định qua Native NT API
unsafe fn perform_memory_cleanup(mask: u32) -> u64 {
    let mem_before = get_memory_info();

    let ntdll = GetModuleHandleW(windows_sys::core::w!("ntdll.dll"));
    if ntdll.is_null() {
        return 0;
    }

    let proc_addr = GetProcAddress(ntdll, b"NtSetSystemInformation\0".as_ptr());
    let Some(proc_addr) = proc_addr else {
        return 0;
    };
    let nt_set_system_info: NtSetSystemInformationFn = std::mem::transmute(proc_addr);

    // 1. Working Sets
    if (mask & id::REDUCT_WORKINGSET) != 0 {
        let cmd = MEMORY_EMPTY_WORKING_SETS;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 2. System File Cache
    if (mask & id::REDUCT_SYSTEMFILECACHE) != 0 {
        let mut sfci: SystemFilecacheInformation = zeroed();
        sfci.minimum_working_set = usize::MAX;
        sfci.maximum_working_set = usize::MAX;
        nt_set_system_info(SYSTEM_FILECACHE_INFORMATION_EX, &sfci as *const _ as *const c_void, size_of::<SystemFilecacheInformation>() as u32);
    }

    // 3. Modified Page List
    if (mask & id::REDUCT_MODIFIEDLIST) != 0 {
        let cmd = MEMORY_FLUSH_MODIFIED_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 4. Standby List
    if (mask & id::REDUCT_STANDBYLIST) != 0 {
        let cmd = MEMORY_PURGE_STANDBY_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 5. Standby List (Priority 0)
    if (mask & id::REDUCT_STANDBYPRIORITY0LIST) != 0 {
        let cmd = MEMORY_PURGE_LOW_PRIORITY_STANDBY_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 6. Registry Cache
    if (mask & id::REDUCT_REGISTRYCACHE) != 0 {
        nt_set_system_info(SYSTEM_REGISTRY_RECONCILIATION_INFORMATION, null(), 0);
    }

    // 7. Combine Memory Lists
    if (mask & id::REDUCT_COMBINEMEMORYLISTS) != 0 {
        let dummy: usize = 0;
        nt_set_system_info(SYSTEM_COMBINE_PHYSICAL_MEMORY_INFORMATION, &dummy as *const _ as *const c_void, size_of::<usize>() as u32);
    }

    let mem_after = get_memory_info();
    if mem_before.used_bytes > mem_after.used_bytes {
        mem_before.used_bytes - mem_after.used_bytes
    } else {
        0
    }
}

// ==============================================================================
// 3. ZERO-ALLOCATION STACK FORMATTERS
// ==============================================================================
pub struct MemStats {
    pub percent: u32,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

unsafe fn get_memory_info() -> MemStats {
    let mut status: MEMORYSTATUSEX = zeroed();
    status.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
    GlobalMemoryStatusEx(&mut status);

    MemStats {
        percent: status.dwMemoryLoad,
        used_bytes: status.ullTotalPhys.saturating_sub(status.ullAvailPhys),
        free_bytes: status.ullAvailPhys,
        total_bytes: status.ullTotalPhys,
    }
}

/// Ghi số nguyên u32 vào bộ đệm UTF-16 trên stack mà không cấp phát heap
fn write_u32_utf16(mut n: u32, buf: &mut [u16]) -> usize {
    if n == 0 {
        if !buf.is_empty() {
            buf[0] = b'0' as u16;
            return 1;
        }
        return 0;
    }
    let mut temp = [0u16; 10];
    let mut i = 0;
    while n > 0 && i < temp.len() {
        temp[i] = (b'0' as u32 + (n % 10)) as u16;
        n /= 10;
        i += 1;
    }
    let len = i;
    for j in 0..len {
        if j < buf.len() {
            buf[j] = temp[len - 1 - j];
        }
    }
    len
}

/// Định dạng dung lượng GB với 1 chữ số thập phân trên stack
fn write_gb_utf16(bytes: u64, buf: &mut [u16]) -> usize {
    // 1 GiB = 1_073_741_824 bytes
    let tenths = (bytes.saturating_mul(10) + 536_870_912) / 1_073_741_824;
    let whole = (tenths / 10) as u32;
    let frac = (tenths % 10) as u32;

    let mut pos = write_u32_utf16(whole, buf);
    if pos < buf.len() {
        buf[pos] = b'.' as u16;
        pos += 1;
    }
    if pos < buf.len() {
        buf[pos] = (b'0' as u32 + frac) as u16;
        pos += 1;
    }
    pos
}

/// Định dạng chuỗi thông tin khay hệ thống trực tiếp vào `szTip` mà không cấp phát String/Vec
fn format_tray_tip(stats: &MemStats, out: &mut [u16; 128]) {
    const P1: &[u16] = &[
        b'M' as u16, b'e' as u16, b'm' as u16, b' ' as u16, b'R' as u16, b'e' as u16, b'd' as u16, b'u' as u16, b'c' as u16, b't' as u16,
        b'\n' as u16,
        b'R' as u16, b'A' as u16, b'M' as u16, b':' as u16, b' ' as u16,
    ];
    const P2: &[u16] = &[
        b'%' as u16, b'\n' as u16,
        b'T' as u16, b'o' as u16, b't' as u16, b'a' as u16, b'l' as u16, b':' as u16, b' ' as u16,
    ];
    const P3: &[u16] = &[
        b' ' as u16, b'G' as u16, b'B' as u16, b' ' as u16, b'|' as u16, b' ' as u16,
        b'F' as u16, b'r' as u16, b'e' as u16, b'e' as u16, b':' as u16, b' ' as u16,
    ];
    const P4: &[u16] = &[b' ' as u16, b'G' as u16, b'B' as u16, 0];

    let mut pos = 0;
    for &c in P1 {
        if pos < 127 { out[pos] = c; pos += 1; }
    }
    pos += write_u32_utf16(stats.percent, &mut out[pos..127]);
    for &c in P2 {
        if pos < 127 { out[pos] = c; pos += 1; }
    }
    pos += write_gb_utf16(stats.total_bytes, &mut out[pos..127]);
    for &c in P3 {
        if pos < 127 { out[pos] = c; pos += 1; }
    }
    pos += write_gb_utf16(stats.free_bytes, &mut out[pos..127]);
    for &c in P4 {
        if pos < 128 { out[pos] = c; pos += 1; }
    }
}

// ==============================================================================
// 4. VẼ ICON ĐỘNG CHO SYSTEM TRAY
// ==============================================================================
unsafe fn create_tray_icon(percent: u32) -> HICON {
    let hdc = GetDC(null_mut());
    let mem_dc = CreateCompatibleDC(hdc);
    let mask_dc = CreateCompatibleDC(hdc);

    let cx = GetSystemMetrics(SM_CXSMICON);
    let cy = GetSystemMetrics(SM_CYSMICON);

    let hbmp = CreateCompatibleBitmap(hdc, cx, cy);
    let hmask = CreateBitmap(cx, cy, 1, 1, null());

    let old_bmp = SelectObject(mem_dc, hbmp);
    let old_mask = SelectObject(mask_dc, hmask);

    let bg_color = if percent >= 90 {
        COLOR_BG_DANGER
    } else if percent >= 75 {
        COLOR_BG_WARN
    } else {
        COLOR_BG_NORMAL
    };

    let brush = CreateSolidBrush(bg_color);
    let mut rect = RECT { left: 0, top: 0, right: cx, bottom: cy };
    FillRect(mem_dc, &rect, brush);
    DeleteObject(brush);

    SetBkMode(mem_dc, TRANSPARENT as i32);
    SetTextColor(mem_dc, COLOR_TEXT);

    let font = CreateFontW(
        -10, 0, 0, 0, FW_BOLD as i32, 0, 0, 0,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH, windows_sys::core::w!("Segoe UI"),
    );
    let old_font = SelectObject(mem_dc, font);

    let mut text_buf = [0u16; 6];
    let text_len = write_u32_utf16(percent, &mut text_buf);
    DrawTextW(mem_dc, text_buf.as_ptr(), text_len as i32, &mut rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    let black_brush = GetStockObject(BLACK_BRUSH);
    FillRect(mask_dc, &rect, black_brush);

    SelectObject(mem_dc, old_font);
    DeleteObject(font);
    SelectObject(mem_dc, old_bmp);
    SelectObject(mask_dc, old_mask);

    DeleteDC(mem_dc);
    DeleteDC(mask_dc);
    ReleaseDC(null_mut(), hdc);

    let mut icon_info = ICONINFO {
        fIcon: 1,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hmask,
        hbmColor: hbmp,
    };

    let icon = CreateIconIndirect(&mut icon_info);
    DeleteObject(hbmp);
    DeleteObject(hmask);

    icon
}

// ==============================================================================
// 5. TRẠNG THÁI ỨNG DỤNG & XỬ LÝ SỰ KIỆN WINDOWS
// ==============================================================================
static mut G_CURRENT_ICON: HICON = null_mut();
static mut G_AUTO_CLEAN: bool = true;
static mut WM_TASKBAR_CREATED: u32 = 0;
const CLEAN_THRESHOLD: u32 = 85;

unsafe fn update_tray(hwnd: HWND, add: bool) {
    let stats = get_memory_info();

    let old_icon = G_CURRENT_ICON;
    G_CURRENT_ICON = create_tray_icon(stats.percent);
    if !old_icon.is_null() {
        DestroyIcon(old_icon);
    }

    let mut nid: NOTIFYICONDATAW = zeroed();
    nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = G_CURRENT_ICON;

    format_tray_tip(&stats, &mut nid.szTip);

    if add {
        Shell_NotifyIconW(NIM_ADD, &nid);
    } else {
        Shell_NotifyIconW(NIM_MODIFY, &nid);
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // Tự động khôi phục icon nếu Windows Explorer khởi động lại
    if msg == WM_TASKBAR_CREATED && WM_TASKBAR_CREATED != 0 {
        update_tray(hwnd, true);
        return 0;
    }

    match msg {
        WM_CREATE => {
            enable_privileges();
            update_tray(hwnd, true);
            SetTimer(hwnd, TIMER_ID, 1000, None);

            // Đăng ký phím tắt toàn cục: Ctrl + F1 dọn dẹp RAM tức thì
            RegisterHotKey(hwnd, HOTKEY_ID, MOD_CONTROL as u32, VK_F1 as u32);
            0
        }

        WM_TIMER => {
            if wparam == TIMER_ID {
                let stats = get_memory_info();
                if G_AUTO_CLEAN && stats.percent >= CLEAN_THRESHOLD {
                    perform_memory_cleanup(id::REDUCT_MASK_ALL);
                }
                update_tray(hwnd, false);
            }
            0
        }

        WM_HOTKEY => {
            if wparam as i32 == HOTKEY_ID {
                let freed = perform_memory_cleanup(id::REDUCT_MASK_ALL);
                update_tray(hwnd, false);

                let mut msg_buf = [0u16; 64];
                let freed_mb = (freed + 524_288) / 1_048_576;
                let text = format!("Memory Cleaned!\nFreed: {} MB\0", freed_mb);
                for (i, c) in text.encode_utf16().enumerate().take(63) {
                    msg_buf[i] = c;
                }
                MessageBoxW(hwnd, msg_buf.as_ptr(), windows_sys::core::w!("Mem Reduct"), MB_OK | MB_ICONINFORMATION);
            }
            0
        }

        WM_TRAYICON => {
            let event = lparam as u32;
            if event == WM_LBUTTONUP {
                let freed = perform_memory_cleanup(id::REDUCT_MASK_ALL);
                update_tray(hwnd, false);

                let mut msg_buf = [0u16; 64];
                let freed_mb = (freed + 524_288) / 1_048_576;
                let text = format!("Memory Cleaned!\nFreed: {} MB\0", freed_mb);
                for (i, c) in text.encode_utf16().enumerate().take(63) {
                    msg_buf[i] = c;
                }
                MessageBoxW(hwnd, msg_buf.as_ptr(), windows_sys::core::w!("Mem Reduct"), MB_OK | MB_ICONINFORMATION);
            } else if event == WM_RBUTTONUP {
                let mut pt: POINT = zeroed();
                GetCursorPos(&mut pt);
                SetForegroundWindow(hwnd);

                let hmenu = CreatePopupMenu();
                AppendMenuW(hmenu, MF_STRING, id::IDM_CLEAN, windows_sys::core::w!("Clean Memory Now\tCtrl+F1"));
                AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

                let auto_flag = if G_AUTO_CLEAN { MF_CHECKED } else { MF_UNCHECKED };
                AppendMenuW(hmenu, MF_STRING | auto_flag, id::IDM_AUTOREDUCT_TOGGLE, windows_sys::core::w!("Auto-Clean (> 85%)"));
                AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

                AppendMenuW(hmenu, MF_STRING, id::IDM_ABOUT, windows_sys::core::w!("About"));
                AppendMenuW(hmenu, MF_STRING, id::IDM_EXIT, windows_sys::core::w!("Exit"));

                TrackPopupMenu(hmenu, TPM_RIGHTALIGN | TPM_BOTTOMALIGN, pt.x, pt.y, 0, hwnd, null());
                DestroyMenu(hmenu);
            }
            0
        }

        WM_COMMAND => {
            let cmd_id = (wparam & 0xFFFF) as usize;
            match cmd_id {
                id::IDM_CLEAN => {
                    perform_memory_cleanup(id::REDUCT_MASK_ALL);
                    update_tray(hwnd, false);
                }
                id::IDM_AUTOREDUCT_TOGGLE => {
                    G_AUTO_CLEAN = !G_AUTO_CLEAN;
                }
                id::IDM_ABOUT => {
                    let about_text = windows_sys::core::w!(
                        "Mem Reduct - Rust Native Edition\nVersion: 3.5.3\nUltra-low RAM overhead (< 1.5MB)\nHigh-performance System Native API.\n\nAuthor: Henry++\nPorted with Zero-Allocation Engine."
                    );
                    MessageBoxW(hwnd, about_text, windows_sys::core::w!("About Mem Reduct"), MB_OK | MB_ICONINFORMATION);
                }
                id::IDM_EXIT => {
                    DestroyWindow(hwnd);
                }
                _ => {}
            }
            0
        }

        WM_DESTROY => {
            KillTimer(hwnd, TIMER_ID);
            UnregisterHotKey(hwnd, HOTKEY_ID);

            let mut nid: NOTIFYICONDATAW = zeroed();
            nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            Shell_NotifyIconW(NIM_DELETE, &nid);

            if !G_CURRENT_ICON.is_null() {
                DestroyIcon(G_CURRENT_ICON);
                G_CURRENT_ICON = null_mut();
            }
            PostQuitMessage(0);
            0
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ==============================================================================
// 6. MAIN ENTRY POINT
// ==============================================================================
fn main() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let class_name = windows_sys::core::w!("MemReductRustClass");

        WM_TASKBAR_CREATED = RegisterWindowMessageW(windows_sys::core::w!("TaskbarCreated"));

        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: null_mut(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: class_name,
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name,
            windows_sys::core::w!("Mem Reduct"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            instance,
            null(),
        );

        if hwnd.is_null() {
            return;
        }

        // Vòng lặp thông điệp Win32 chuẩn
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
