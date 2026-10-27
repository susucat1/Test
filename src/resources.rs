//! Resource IDs and Command constants (Cleaned for Rust)

// Accelerators / Hotkeys
pub const IDA_MAIN: u16 = 1;

// Menu IDs
pub const IDM_MAIN: u16 = 100;
pub const IDM_TRAY: u16 = 101;

// Dialog IDs
pub const IDD_MAIN: u16 = 100;
pub const IDD_SETTINGS_GENERAL: u16 = 101;
pub const IDD_SETTINGS_MEMORY: u16 = 102;
pub const IDD_SETTINGS_APPEARANCE: u16 = 103;
pub const IDD_SETTINGS_TRAY: u16 = 104;
pub const IDD_SETTINGS_ADVANCED: u16 = 105;

// Main Dialog Controls
pub const IDC_LISTVIEW: u16 = 102;
pub const IDC_CLEAN: u16 = 103;

// Settings & Toggles
pub const IDC_ALWAYSONTOP_CHK: u16 = 113;
pub const IDC_LOADONSTARTUP_CHK: u16 = 114;
pub const IDC_STARTMINIMIZED_CHK: u16 = 115;
pub const IDC_SKIPUACWARNING_CHK: u16 = 116;
pub const IDC_REDUCTCONFIRMATION_CHK: u16 = 117;
pub const IDC_CHECKUPDATES_CHK: u16 = 118;
pub const IDC_NOTIFICATIONSOUND_CHK: u16 = 148;
pub const IDC_SHOW_CLEAN_RESULT_CHK: u16 = 149;
pub const IDC_ALLOWSTANDBYLISTCLEANUP_CHK: u16 = 150;
pub const IDC_LOGRESULTS_CHK: u16 = 151;

// Auto-clean & Interval Controls
pub const IDC_AUTOREDUCTENABLE_CHK: u16 = 122;
pub const IDC_AUTOREDUCTVALUE: u16 = 124;
pub const IDC_AUTOREDUCTINTERVALENABLE_CHK: u16 = 125;
pub const IDC_AUTOREDUCTINTERVALVALUE: u16 = 127;
pub const IDC_HOTKEY_CLEAN_CHK: u16 = 128;
pub const IDC_HOTKEY_CLEAN: u16 = 129;

// Tray Style Controls
pub const IDC_TRAYUSETRANSPARENCY_CHK: u16 = 130;
pub const IDC_TRAYSHOWBORDER_CHK: u16 = 131;
pub const IDC_TRAYROUNDCORNERS_CHK: u16 = 132;
pub const IDC_TRAYCHANGEBG_CHK: u16 = 133;
pub const IDC_TRAYUSEANTIALIASING_CHK: u16 = 134;

// Actions / Menus
pub const IDM_SETTINGS: u16 = 152;
pub const IDM_EXIT: u16 = 153;
pub const IDM_USEDARKTHEME: u16 = 156;
pub const IDM_WEBSITE: u16 = 161;
pub const IDM_CHECKUPDATES: u16 = 162;
pub const IDM_ABOUT: u16 = 163;

// Tray Specific Menus
pub const IDM_TRAY_SHOW: u16 = 164;
pub const IDM_TRAY_CLEAN: u16 = 165;
pub const IDM_TRAY_SETTINGS: u16 = 168;
pub const IDM_TRAY_EXIT: u16 = 171;

// Memory Regions To Clean
pub const IDM_CLEAN_WORKINGSET: u16 = 180;
pub const IDM_CLEAN_SYSTEMFILECACHE: u16 = 181;
pub const IDM_CLEAN_MODIFIEDFILECACHE: u16 = 182;
pub const IDM_CLEAN_MODIFIEDLIST: u16 = 183;
pub const IDM_CLEAN_STANDBYLIST: u16 = 184;
pub const IDM_CLEAN_STANDBYLISTPRIORITY0: u16 = 185;
pub const IDM_CLEAN_REGISTRYCACHE: u16 = 186;
pub const IDM_CLEAN_COMBINEMEMORYLISTS: u16 = 187;

// Icon ID
pub const IDI_MAIN: u16 = 100;
