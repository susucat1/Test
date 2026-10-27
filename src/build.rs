//! ==============================================================================
//! Mem Reduct - Unified Win32 Resources & English Strings (Rust Port)
//! Zero-allocation, Minimal RAM footprint, Pure English, High-Performance.
//! ==============================================================================

// ------------------------------------------------------------------------------
// 1. CONTROL & COMMAND IDENTIFIERS (IDs)
// ------------------------------------------------------------------------------
pub mod id {
    // Accelerators
    pub const IDA_MAIN: u16 = 1;

    // Menus & Dialog IDs
    pub const IDM_MAIN: u16 = 100;
    pub const IDM_TRAY: u16 = 101;
    pub const IDD_MAIN: u16 = 100;
    pub const IDD_SETTINGS_GENERAL: u16 = 101;
    pub const IDD_SETTINGS_MEMORY: u16 = 102;
    pub const IDD_SETTINGS_APPEARANCE: u16 = 103;
    pub const IDD_SETTINGS_TRAY: u16 = 104;
    pub const IDD_SETTINGS_ADVANCED: u16 = 105;

    // Main Dialog Controls
    pub const IDC_LISTVIEW: u16 = 102;
    pub const IDC_CLEAN: u16 = 103;

    // GroupBox Titles
    pub const IDC_TITLE_1: u16 = 104; // General configuration
    pub const IDC_TITLE_3: u16 = 106; // Memory regions
    pub const IDC_TITLE_4: u16 = 107; // Memory management
    pub const IDC_TITLE_5: u16 = 108; // Hotkeys
    pub const IDC_TITLE_6: u16 = 109; // Style / Font
    pub const IDC_TITLE_7: u16 = 110; // Color indication
    pub const IDC_TITLE_8: u16 = 111; // Mouse control
    pub const IDC_TITLE_9: u16 = 112; // Balloon tips

    // Settings: General Controls
    pub const IDC_ALWAYSONTOP_CHK: u16 = 113;
    pub const IDC_LOADONSTARTUP_CHK: u16 = 114;
    pub const IDC_STARTMINIMIZED_CHK: u16 = 115;
    pub const IDC_SKIPUACWARNING_CHK: u16 = 116;
    pub const IDC_REDUCTCONFIRMATION_CHK: u16 = 117;
    pub const IDC_CHECKUPDATES_CHK: u16 = 118;

    // Settings: Memory Cleaning Controls
    pub const IDC_REGIONS: u16 = 121;
    pub const IDC_AUTOREDUCTENABLE_CHK: u16 = 122;
    pub const IDC_AUTOREDUCTVALUE_CTRL: u16 = 123;
    pub const IDC_AUTOREDUCTVALUE: u16 = 124;
    pub const IDC_AUTOREDUCTINTERVALENABLE_CHK: u16 = 125;
    pub const IDC_AUTOREDUCTINTERVALVALUE_CTRL: u16 = 126;
    pub const IDC_AUTOREDUCTINTERVALVALUE: u16 = 127;
    pub const IDC_HOTKEY_CLEAN_CHK: u16 = 128;
    pub const IDC_HOTKEY_CLEAN: u16 = 129;

    // Settings: Appearance Controls
    pub const IDC_TRAYUSETRANSPARENCY_CHK: u16 = 130;
    pub const IDC_TRAYSHOWBORDER_CHK: u16 = 131;
    pub const IDC_TRAYROUNDCORNERS_CHK: u16 = 132;
    pub const IDC_TRAYCHANGEBG_CHK: u16 = 133;
    pub const IDC_TRAYUSEANTIALIASING_CHK: u16 = 134;
    pub const IDC_FONT_HINT: u16 = 135;
    pub const IDC_FONT: u16 = 136;
    pub const IDC_COLORS: u16 = 137;

    // Settings: Tray Icon Controls
    pub const IDC_TRAYLEVELWARNING_HINT: u16 = 138;
    pub const IDC_TRAYLEVELWARNING_CTRL: u16 = 139;
    pub const IDC_TRAYLEVELWARNING: u16 = 140;
    pub const IDC_TRAYLEVELDANGER_HINT: u16 = 141;
    pub const IDC_TRAYLEVELDANGER_CTRL: u16 = 142;
    pub const IDC_TRAYLEVELDANGER: u16 = 143;
    pub const IDC_TRAYACTIONSC_HINT: u16 = 144;
    pub const IDC_TRAYACTIONSC: u16 = 145;
    pub const IDC_TRAYACTIONMC_HINT: u16 = 146;
    pub const IDC_TRAYACTIONMC: u16 = 147;
    pub const IDC_NOTIFICATIONSOUND_CHK: u16 = 148;
    pub const IDC_SHOW_CLEAN_RESULT_CHK: u16 = 149;

    // Settings: Advanced Controls
    pub const IDC_ALLOWSTANDBYLISTCLEANUP_CHK: u16 = 150;
    pub const IDC_LOGRESULTS_CHK: u16 = 151;

    // Main Menu Commands
    pub const IDM_SETTINGS: u16 = 152;
    pub const IDM_EXIT: u16 = 153;
    pub const IDM_ALWAYSONTOP_CHK: u16 = 154;
    pub const IDM_LOADONSTARTUP_CHK: u16 = 155;
    pub const IDM_USEDARKTHEME: u16 = 156;
    pub const IDM_STARTMINIMIZED_CHK: u16 = 157;
    pub const IDM_REDUCTCONFIRMATION_CHK: u16 = 158;
    pub const IDM_SKIPUACWARNING_CHK: u16 = 159;
    pub const IDM_CHECKUPDATES_CHK: u16 = 160;
    pub const IDM_WEBSITE: u16 = 161;
    pub const IDM_CHECKUPDATES: u16 = 162;
    pub const IDM_ABOUT: u16 = 163;

    // Tray Menu Commands
    pub const IDM_TRAY_SHOW: u16 = 164;
    pub const IDM_TRAY_CLEAN: u16 = 165;
    pub const IDM_TRAY_DISABLE_1: u16 = 166;
    pub const IDM_TRAY_DISABLE_2: u16 = 167;
    pub const IDM_TRAY_SETTINGS: u16 = 168;
    pub const IDM_TRAY_WEBSITE: u16 = 169;
    pub const IDM_TRAY_ABOUT: u16 = 170;
    pub const IDM_TRAY_EXIT: u16 = 171;

    // Memory Regions Check Toggles
    pub const IDM_WORKINGSET_CHK: u16 = 172;
    pub const IDM_SYSTEMFILECACHE_CHK: u16 = 173;
    pub const IDM_MODIFIEDFILECACHE_CHK: u16 = 174;
    pub const IDM_MODIFIEDLIST_CHK: u16 = 175;
    pub const IDM_STANDBYLIST_CHK: u16 = 176;
    pub const IDM_STANDBYLISTPRIORITY0_CHK: u16 = 177;
    pub const IDM_REGISTRYCACHE_CHK: u16 = 178;
    pub const IDM_COMBINEMEMORYLISTS_CHK: u16 = 179;

    // Direct Clean Region Action Commands
    pub const IDM_CLEAN_WORKINGSET: u16 = 180;
    pub const IDM_CLEAN_SYSTEMFILECACHE: u16 = 181;
    pub const IDM_CLEAN_MODIFIEDFILECACHE: u16 = 182;
    pub const IDM_CLEAN_MODIFIEDLIST: u16 = 183;
    pub const IDM_CLEAN_STANDBYLIST: u16 = 184;
    pub const IDM_CLEAN_STANDBYLISTPRIORITY0: u16 = 185;
    pub const IDM_CLEAN_REGISTRYCACHE: u16 = 186;
    pub const IDM_CLEAN_COMBINEMEMORYLISTS: u16 = 187;

    // App Icon
    pub const IDI_MAIN: u16 = 100;
}

// ------------------------------------------------------------------------------
// 2. ENGLISH STRINGS TABLE (ZERO-HEAP ALLOCATION)
// ------------------------------------------------------------------------------
pub mod string {
    pub const APP_NAME: &str = "Mem Reduct";
    pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    // Menus & Common Actions
    pub const FILE: &str = "File";
    pub const VIEW: &str = "View";
    pub const SETTINGS: &str = "Settings";
    pub const HELP: &str = "Help";
    pub const EXIT: &str = "Exit";
    pub const CLOSE: &str = "Close";
    pub const RESET: &str = "Reset";
    pub const WEBSITE: &str = "Website";
    pub const CHECKUPDATES: &str = "Check for updates";
    pub const ABOUT: &str = "About";
    pub const DONATE: &str = "Give thanks!";
    pub const CLEAN_MEMORY: &str = "Clean memory";

    // Memory info categories
    pub const GROUP_PHYSICAL_MEM: &str = "Physical memory";
    pub const GROUP_PAGEFILE: &str = "Pagefile";
    pub const GROUP_SYSTEM_WS: &str = "System working set";

    pub const ITEM_USAGE: &str = "Usage";
    pub const ITEM_AVAILABLE: &str = "Available";
    pub const ITEM_TOTAL: &str = "Total available";

    // Tab Header Labels
    pub const TAB_GENERAL: &str = "General";
    pub const TAB_MEMORY: &str = "Memory cleaning";
    pub const TAB_APPEARANCE: &str = "Appearance";
    pub const TAB_TRAY: &str = "Tray icon";
    pub const TAB_ADVANCED: &str = "Advanced";

    // Settings: General Checkboxes
    pub const TITLE_GENERAL: &str = "General configuration";
    pub const ALWAYS_ON_TOP: &str = "Always on top";
    pub const LOAD_ON_STARTUP: &str = "Load on system startup";
    pub const START_MINIMIZED: &str = "Start minimized";
    pub const CONFIRM_CLEAN: &str = "Confirm memory cleaning start";
    pub const SKIP_UAC: &str = "Skip \"User Account Control\" prompt warning";
    pub const PERIODIC_UPDATES: &str = "Periodically check for updates (recommended)";
    pub const USE_DARK_THEME: &str = "Use dark theme";

    // Settings: Memory Cleaning
    pub const TITLE_REGIONS: &str = "Memory regions to be cleaned";
    pub const TITLE_MANAGEMENT: &str = "Memory management";
    pub const TITLE_HOTKEYS: &str = "Hotkeys (global)";
    pub const CLEAN_WHEN_ABOVE: &str = "Clean when above: (%)";
    pub const CLEAN_EVERY: &str = "Clean every: (min.)";
    pub const HOTKEY_CLEAN: &str = "Clean memory:";

    // Settings: Appearance & Style
    pub const TITLE_STYLE: &str = "Style";
    pub const TITLE_COLOR: &str = "Color indication";
    pub const TRAY_TRANSPARENT_BG: &str = "Use transparent background";
    pub const TRAY_SHOW_BORDER: &str = "Show border";
    pub const TRAY_ROUND_CORNERS: &str = "Round corners";
    pub const TRAY_CHANGE_BG: &str = "Change background color (instead of text and border)";
    pub const TRAY_ANTIALIASING: &str = "Use antialiasing";
    pub const FONT_HINT: &str = "Font:";
    pub const COLOR_TEXT_HINT: &str = "Text color";
    pub const COLOR_BG_HINT: &str = "Background color";
    pub const COLOR_WARNING_HINT: &str = "Warning level color";
    pub const COLOR_DANGER_HINT: &str = "Danger level color";

    // Settings: Tray Controls
    pub const TITLE_MOUSE: &str = "Mouse control";
    pub const TITLE_TIPS: &str = "Balloon tips";
    pub const TRAY_LEVEL_WARNING_HINT: &str = "Warning level starts at: (%)";
    pub const TRAY_LEVEL_DANGER_HINT: &str = "Danger level starts at: (%)";
    pub const TRAY_ACTION_SC_HINT: &str = "Single click:";
    pub const TRAY_ACTION_MC_HINT: &str = "Middle click:";
    pub const TRAY_ACTION_SHOW_HIDE: &str = "Show/Hide";
    pub const TRAY_ACTION_CLEAN: &str = "Clean memory";
    pub const TRAY_ACTION_TASK_MGR: &str = "Open task manager";
    pub const SHOW_CLEAN_RESULT: &str = "Show memory cleaning results";
    pub const NOTIFICATION_SOUND: &str = "Enable notifications sound";

    // Tray Menu Labels
    pub const TRAY_SHOW_HIDE: &str = "Show/Hide";
    pub const TRAY_DISABLE: &str = "Disable";
    pub const TRAY_CLEAN_AREAS: &str = "Clean areas";
    pub const TRAY_CLEAN_WHEN_ABOVE: &str = "Clean when above";
    pub const TRAY_CLEAN_EVERY: &str = "Clean every";

    // Memory Regions
    pub const REGION_WORKING_SET: &str = "Working set";
    pub const REGION_SYSTEM_FILE_CACHE: &str = "System file cache";
    pub const REGION_MODIFIED_FILE_CACHE: &str = "Modified file cache";
    pub const REGION_MODIFIED_LIST: &str = "Modified page list";
    pub const REGION_STANDBY_LIST: &str = "Standby list";
    pub const REGION_STANDBY_LIST_PRIORITY0: &str = "Standby list (without priority)";
    pub const REGION_REGISTRY_CACHE: &str = "Registry cache";
    pub const REGION_COMBINE_MEMORY: &str = "Combine memory lists";

    // Settings: Advanced
    pub const TITLE_ADVANCED: &str = "Advanced";
    pub const ALLOW_STANDBY_CLEANUP: &str = "Allow \"Standby lists\" and \"Modified page list\" cleanup on auto-clean";
    pub const LOG_RESULTS: &str = "Log cleaning results into a debug log";

    // Prompts and Notifications
    pub const PROMPT_CLEAN_CONFIRM: &str = "Are you sure you want to clean the memory?";
    pub const PROMPT_STANDBY_WARNING: &str = "Enabling \"Standby list\" and \"Modified page list\" may freeze your computer for a moment. Are you sure you want to enable this option?";
    pub const PROMPT_RESET_CONFIRM: &str = "Are you sure you want to reset all application settings?";
    pub const STATUS_CLEANED_SUCCESS: &str = "Memory was released successfully.";
    pub const STATUS_NO_PRIVILEGES: &str = "Administrator privileges required. Please run program as administrator!";

    // Updates
    pub const UPDATE_NO: &str = "No updates available.";
    pub const UPDATE_YES: &str = "Update available. Do you want to download and install it?";
    pub const UPDATE_DOWNLOADING: &str = "Downloading update...";
    pub const UPDATE_DONE: &str = "Downloading update finished.";
    pub const UPDATE_ERROR: &str = "Update server connection error.";
    pub const UPDATE_INIT: &str = "Checking for new releases...";
}

// ------------------------------------------------------------------------------
// 3. WIN32 UTF-16 WIDE STRING UTILITY (ZERO-ALLOCATION)
// ------------------------------------------------------------------------------
/// Macro chuyển chuỗi &'static str thành &'static [u16] kết thúc bằng 0
/// Tương thích 100% với Win32 API PCWSTR mà không sinh heap memory.
#[macro_export]
macro_rules! wstr {
    ($s:expr) => {{
        const S: &str = $s;
        const LEN: usize = S.len() + 1;
        const fn to_utf16() -> [u16; LEN] {
            let bytes = S.as_bytes();
            let mut buf = [0u16; LEN];
            let mut i = 0;
            while i < bytes.len() {
                buf[i] = bytes[i] as u16;
                i += 1;
            }
            buf[LEN - 1] = 0;
            buf
        }
        &to_utf16()
    }};
}

// ------------------------------------------------------------------------------
// 4. ACCELERATOR KEYS DEFINITION (Thay thế IDA_MAIN trong .rc)
// ------------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Win32Accel {
    pub f_virt: u8,
    pub key: u16,
    pub cmd: u16,
}

pub mod accel {
    use super::id::*;
    use super::Win32Accel;

    pub const FVIRTKEY: u8 = 0x01;
    pub const FALT: u8 = 0x10;

    // Phím ảo Win32
    pub const VK_F1: u16 = 0x70;
    pub const VK_F2: u16 = 0x71;
    pub const VK_F4: u16 = 0x73;

    /// Danh sách bảng phím tắt (F1: About, F2: Settings, Alt+F4: Hide/Show Tray)
    pub const MAIN_ACCELERATORS: [Win32Accel; 3] = [
        Win32Accel { f_virt: FVIRTKEY, key: VK_F1, cmd: IDM_ABOUT },
        Win32Accel { f_virt: FVIRTKEY, key: VK_F2, cmd: IDM_SETTINGS },
        Win32Accel { f_virt: FVIRTKEY | FALT, key: VK_F4, cmd: IDM_TRAY_SHOW },
    ];
}

// ------------------------------------------------------------------------------
// 5. APPLICATION VERSION & METADATA (Thay thế VERSIONINFO trong .rc)
// ------------------------------------------------------------------------------
pub struct AppMetadata;
impl AppMetadata {
    pub const NAME: &'static str = "Mem Reduct";
    pub const SHORT_NAME: &'static str = "memreduct";
    pub const COMPANY: &'static str = "Henry++";
    pub const COPYRIGHT: &'static str = "Copyright (c) 2011-2026 Henry++";
    pub const ORIGINAL_FILENAME: &'static str = "memreduct.exe";
}
