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
// 1. HẰNG SỐ, ID LỆNH & CHUỖI TIẾNG ANH (ZERO-ALLOCATION)
// ==============================================================================
pub const APP_NAME: &str = "Mem Reduct";
pub const APP_VERSION: &str = "3.5.3";

// Menu Command IDs
pub const IDM_CLEAN: usize = 1001;
pub const IDM_SETTINGS: usize = 1002;
pub const IDM_ABOUT: usize = 1003;
pub const IDM_EXIT: usize = 1004;
pub const IDM_AUTOREDUCT_TOGGLE: usize = 1005;

// Memory Masks
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

// Win32 Message / Timers
pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const TIMER_ID: usize = 1337;
pub const HOTKEY_ID: i32 = 101;

// Màu sắc khay hệ thống (RGB)
pub const COLOR_TEXT: u32 = 0x00FFFFFF;     // White
pub const COLOR_BG_NORMAL: u32 = 0x00408000;// Forest Green
pub const COLOR_BG_WARN: u32 = 0x000080FF;  // Orange
pub const COLOR_BG_DANGER: u32 = 0x00241CEC;// Red

// ==============================================================================
// 2. NATIVE NT API (NTDLL DỌN DẸP BỘ NHỚ CẤP HỆ ĐIỀU HÀNH)
// ==============================================================================
type NtSetSystemInformationFn = unsafe extern "system" fn(
    info_class: u32,
    info: *const c_void,
    length: u32,
) -> i32;

const SYSTEM_MEMORY_LIST_INFORMATION: u32 = 80;
const SYSTEM_FILECACHE_INFORMATION_EX: u32 = 21;
const SYSTEM_REGISTRY_RECONCILIATION_INFORMATION: u32 = 135;
const SYSTEM_COMBINE_PHYSICAL_MEMORY_INFORMATION: u32 = 130;

// Các lệnh của SystemMemoryListInformation
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

/// Nâng quyền tiến trình để can thiệp bộ nhớ hệ thống
unsafe fn enable_privileges() {
    let mut token: HANDLE = 0;
    if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) != 0 {
        let priv_names = [
            windows_sys::core::w!("SeIncreaseQuotaPrivilege"),
            windows_sys::core::w!("SeProfileSingleProcessPrivilege"),
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
                    &mut tp,
                    size_of::<TOKEN_PRIVILEGES>() as u32,
                    null_mut(),
                    null_mut(),
                );
            }
        }
        CloseHandle(token);
    }
}

/// Thực thi dọn dẹp các phân vùng bộ nhớ
unsafe fn perform_memory_cleanup(mask: u32) -> u64 {
    let mem_before = get_memory_info();

    let ntdll_name = windows_sys::core::w!("ntdll.dll");
    let ntdll = GetModuleHandleW(ntdll_name);
    if ntdll == 0 {
        return 0;
    }

    let proc_addr = GetProcAddress(ntdll, b"NtSetSystemInformation\0".as_ptr());
    if proc_addr.is_none() {
        return 0;
    }
    let nt_set_system_info: NtSetSystemInformationFn = std::mem::transmute(proc_addr);

    // 1. Working Sets
    if (mask & REDUCT_WORKINGSET) != 0 {
        let cmd = MEMORY_EMPTY_WORKING_SETS;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 2. System File Cache
    if (mask & REDUCT_SYSTEMFILECACHE) != 0 {
        let mut sfci: SystemFilecacheInformation = zeroed();
        sfci.minimum_working_set = usize::MAX;
        sfci.maximum_working_set = usize::MAX;
        nt_set_system_info(SYSTEM_FILECACHE_INFORMATION_EX, &sfci as *const _ as *const c_void, size_of::<SystemFilecacheInformation>() as u32);
    }

    // 3. Modified Page List
    if (mask & REDUCT_MODIFIEDLIST) != 0 {
        let cmd = MEMORY_FLUSH_MODIFIED_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 4. Standby List
    if (mask & REDUCT_STANDBYLIST) != 0 {
        let cmd = MEMORY_PURGE_STANDBY_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 5. Standby List (Priority 0)
    if (mask & REDUCT_STANDBYPRIORITY0LIST) != 0 {
        let cmd = MEMORY_PURGE_LOW_PRIORITY_STANDBY_LIST;
        nt_set_system_info(SYSTEM_MEMORY_LIST_INFORMATION, &cmd as *const _ as *const c_void, size_of::<u32>() as u32);
    }

    // 6. Registry Cache
    if (mask & REDUCT_REGISTRYCACHE) != 0 {
        nt_set_system_info(SYSTEM_REGISTRY_RECONCILIATION_INFORMATION, null(), 0);
    }

    // 7. Combine Memory Lists
    if (mask & REDUCT_COMBINEMEMORYLISTS) != 0 {
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
// 3. QUẢN LÝ DỮ LIỆU BỘ NHỚ & TẠO ICON ĐỘNG CHO SYSTEM TRAY
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
        used_bytes: status.ullTotalPhys - status.ullAvailPhys,
        free_bytes: status.ullAvailPhys,
        total_bytes: status.ullTotalPhys,
    }
}

/// Vẽ icon khay hệ thống hiển thị % RAM trực tiếp (Dynamic GDI Icon)
unsafe fn create_tray_icon(percent: u32) -> HICON {
    let hdc = GetDC(0);
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

    // Vẽ text hiển thị số %
    SetBkMode(mem_dc, TRANSPARENT as i32);
    SetTextColor(mem_dc, COLOR_TEXT);

    let font = CreateFontW(
        -10, 0, 0, 0, FW_BOLD as i32, 0, 0, 0,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH, windows_sys::core::w!("Segoe UI"),
    );
    let old_font = SelectObject(mem_dc, font);

    let text_str = format!("{}\0", percent);
    let wide_text: Vec<u16> = text_str.encode_utf16().collect();
    DrawTextW(mem_dc, wide_text.as_ptr(), -1, &mut rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

    // Xóa mask (opaque)
    let black_brush = GetStockObject(BLACK_BRUSH);
    FillRect(mask_dc, &rect, black_brush);

    SelectObject(mem_dc, old_font);
    DeleteObject(font);
    SelectObject(mem_dc, old_bmp);
    SelectObject(mask_dc, old_mask);

    DeleteDC(mem_dc);
    DeleteDC(mask_dc);
    ReleaseDC(0, hdc);

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
// 4. APPLICATION STATE & WINDOW PROCEDURE
// ==============================================================================
static mut G_CURRENT_ICON: HICON = 0;
static mut G_AUTO_CLEAN: bool = true;
const CLEAN_THRESHOLD: u32 = 85;

unsafe fn update_tray(hwnd: HWND, add: bool) {
    let stats = get_memory_info();

    if G_CURRENT_ICON != 0 {
        DestroyIcon(G_CURRENT_ICON);
    }
    G_CURRENT_ICON = create_tray_icon(stats.percent);

    let tip = format!(
        "Mem Reduct\nRAM Usage: {}%\nTotal: {:.1} GB | Free: {:.1} GB\0",
        stats.percent,
        stats.total_bytes as f64 / 1_073_741_824.0,
        stats.free_bytes as f64 / 1_073_741_824.0
    );

    let mut nid: NOTIFYICONDATAW = zeroed();
    nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = G_CURRENT_ICON;

    let tip_wide: Vec<u16> = tip.encode_utf16().collect();
    for (i, &c) in tip_wide.iter().enumerate().take(127) {
        nid.szTip[i] = c;
    }

    if add {
        Shell_NotifyIconW(NIM_ADD, &nid);
    } else {
        Shell_NotifyIconW(NIM_MODIFY, &nid);
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            enable_privileges();
            update_tray(hwnd, true);
            SetTimer(hwnd, TIMER_ID, 1000, None);

            // Đăng ký phím tắt: Ctrl + F1 để dọn RAM tức thì
            RegisterHotKey(hwnd, HOTKEY_ID, MOD_CONTROL as u32, VK_F1 as u32);
            0
        }

        WM_TIMER => {
            if wparam == TIMER_ID {
                let stats = get_memory_info();
                if G_AUTO_CLEAN && stats.percent >= CLEAN_THRESHOLD {
                    perform_memory_cleanup(REDUCT_MASK_ALL);
                }
                update_tray(hwnd, false);
            }
            0
        }

        WM_HOTKEY => {
            if wparam as i32 == HOTKEY_ID {
                let freed = perform_memory_cleanup(REDUCT_MASK_ALL);
                update_tray(hwnd, false);
                let msg = format!("Cleaned: {:.1} MB released!\0", freed as f64 / 1_048_576.0);
                let msg_w: Vec<u16> = msg.encode_utf16().collect();
                MessageBoxW(hwnd, msg_w.as_ptr(), windows_sys::core::w!("Mem Reduct"), MB_OK | MB_ICONINFORMATION);
            }
            0
        }

        WM_TRAYICON => {
            let event = lparam as u32;
            if event == WM_RBUTTONUP || event == WM_LBUTTONUP {
                if event == WM_LBUTTONUP {
                    // Click chuột trái: Dọn dẹp nhanh
                    let freed = perform_memory_cleanup(REDUCT_MASK_ALL);
                    update_tray(hwnd, false);
                    let msg = format!("Memory Cleaned!\nFreed: {:.1} MB\0", freed as f64 / 1_048_576.0);
                    let msg_w: Vec<u16> = msg.encode_utf16().collect();
                    MessageBoxW(hwnd, msg_w.as_ptr(), windows_sys::core::w!("Mem Reduct"), MB_OK | MB_ICONINFORMATION);
                } else {
                    // Click chuột phải: Mở Menu Context
                    let mut pt: POINT = zeroed();
                    GetCursorPos(&mut pt);
                    SetForegroundWindow(hwnd);

                    let hmenu = CreatePopupMenu();
                    AppendMenuW(hmenu, MF_STRING, IDM_CLEAN, windows_sys::core::w!("Clean Memory Now\tCtrl+F1"));
                    AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

                    let auto_flag = if G_AUTO_CLEAN { MF_CHECKED } else { MF_UNCHECKED };
                    AppendMenuW(hmenu, MF_STRING | auto_flag, IDM_AUTOREDUCT_TOGGLE, windows_sys::core::w!("Auto-Clean (> 85%)"));
                    AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

                    AppendMenuW(hmenu, MF_STRING, IDM_ABOUT, windows_sys::core::w!("About"));
                    AppendMenuW(hmenu, MF_STRING, IDM_EXIT, windows_sys::core::w!("Exit"));

                    TrackPopupMenu(hmenu, TPM_RIGHTALIGN | TPM_BOTTOMALIGN, pt.x, pt.y, 0, hwnd, null());
                    DestroyMenu(hmenu);
                }
            }
            0
        }

        WM_COMMAND => {
            let cmd_id = (wparam & 0xFFFF) as usize;
            match cmd_id {
                IDM_CLEAN => {
                    perform_memory_cleanup(REDUCT_MASK_ALL);
                    update_tray(hwnd, false);
                }
                IDM_AUTOREDUCT_TOGGLE => {
                    G_AUTO_CLEAN = !G_AUTO_CLEAN;
                }
                IDM_ABOUT => {
                    let text = format!("Mem Reduct v{}\nHigh-performance Memory Cleaner (Rust Edition)\nZero Overhead, English only.\0", APP_VERSION);
                    let text_w: Vec<u16> = text.encode_utf16().collect();
                    MessageBoxW(hwnd, text_w.as_ptr(), windows_sys::core::w!("About Mem Reduct"), MB_OK | MB_ICONINFORMATION);
                }
                IDM_EXIT => {
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

            if G_CURRENT_ICON != 0 {
                DestroyIcon(G_CURRENT_ICON);
            }
            PostQuitMessage(0);
            0
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ==============================================================================
// 5. MAIN ENTRY POINT
// ==============================================================================
fn main() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let class_name = windows_sys::core::w!("MemReductRustClass");

        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: 0,
            hCursor: LoadCursorW(0, IDC_ARROW),
            hbrBackground: 0,
            lpszMenuName: null(),
            lpszClassName: class_name,
        };

        RegisterClassW(&wc);

        // Tạo window ẩn (chỉ chạy background + system tray icon)
        let hwnd = CreateWindowExW(
            0,
            class_name,
            windows_sys::core::w!("Mem Reduct"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0,
            0,
            instance,
            null(),
        );

        if hwnd.is_null() {
            return;
        }

        // Vòng lặp Win32 Message Loop
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
           TranslateMessage(&msg);
           DispatchMessageW(&msg);
        }
    }
}
