//! English String Table (Static Zero-cost Constants)

pub struct Strings;

impl Strings {
    pub const APP_NAME: &'static str = "Mem Reduct";
    pub const CLEAN: &'static str = "Clean memory";
    pub const CLOSE: &'static str = "Close";
    pub const RESET: &'static str = "Reset";
    pub const ABOUT: &'static str = "About";
    pub const SETTINGS: &'static str = "Settings";
    pub const EXIT: &'static str = "Exit";
    pub const WEBSITE: &'static str = "Website";
    pub const DONATE: &'static str = "Give thanks!";
    pub const CHECKUPDATES: &'static str = "Check for updates";

    // Memory info categories
    pub const GROUP_PHYSICAL_MEM: &'static str = "Physical memory";
    pub const GROUP_PAGEFILE: &'static str = "Pagefile";
    pub const GROUP_SYSTEM_WS: &'static str = "System working set";

    pub const ITEM_USAGE: &'static str = "Usage";
    pub const ITEM_AVAILABLE: &'static str = "Available";
    pub const ITEM_TOTAL: &'static str = "Total available";

    // Settings tabs
    pub const TAB_GENERAL: &'static str = "General";
    pub const TAB_MEMORY: &'static str = "Memory cleaning";
    pub const TAB_APPEARANCE: &'static str = "Appearance";
    pub const TAB_TRAY: &'static str = "Tray icon";
    pub const TAB_ADVANCED: &'static str = "Advanced";

    // Options
    pub const ALWAYS_ON_TOP: &'static str = "Always on top";
    pub const LOAD_ON_STARTUP: &'static str = "Load on system startup";
    pub const START_MINIMIZED: &'static str = "Start minimized";
    pub const CONFIRM_CLEAN: &'static str = "Confirm memory cleaning start";
    pub const SKIP_UAC: &'static str = "Skip \"User Account Control\" prompt warning";
    pub const PERIODIC_UPDATES: &'static str = "Periodically check for updates (recommended)";
    pub const USE_DARK_THEME: &'static str = "Use dark theme";

    // Auto-clean
    pub const CLEAN_WHEN_ABOVE: &'static str = "Clean when above: (%)";
    pub const CLEAN_EVERY: &'static str = "Clean every: (min.)";
    pub const HOTKEY_CLEAN: &'static str = "Clean memory:";

    // Tray
    pub const TRAY_SHOW_HIDE: &'static str = "Show/Hide";
    pub const TRAY_DISABLE: &'static str = "Disable";
    pub const TRAY_CLEAN_AREAS: &'static str = "Clean areas";
    pub const TRAY_CLEAN_WHEN_ABOVE: &'static str = "Clean when above";
    pub const TRAY_CLEAN_EVERY: &'static str = "Clean every";
    pub const TRAY_TRANSPARENT_BG: &'static str = "Use transparent background";
    pub const TRAY_SHOW_BORDER: &'static str = "Show border";
    pub const TRAY_ROUND_CORNERS: &'static str = "Round corners";
    pub const TRAY_CHANGE_BG: &'static str = "Change background color (instead of text and border)";
    pub const TRAY_ANTIALIASING: &'static str = "Use antialiasing";

    // Memory regions
    pub const REGION_WORKING_SET: &'static str = "Working set";
    pub const REGION_SYSTEM_FILE_CACHE: &'static str = "System file cache";
    pub const REGION_MODIFIED_FILE_CACHE: &'static str = "Modified file cache";
    pub const REGION_MODIFIED_LIST: &'static str = "Modified page list";
    pub const REGION_STANDBY_LIST: &'static str = "Standby list";
    pub const REGION_STANDBY_LIST_PRIORITY0: &'static str = "Standby list (without priority)";
    pub const REGION_REGISTRY_CACHE: &'static str = "Registry cache";
    pub const REGION_COMBINE_MEMORY: &'static str = "Combine memory lists";

    // Dialog messages
    pub const PROMPT_CLEAN_CONFIRM: &'static str = "Are you sure you want to clean the memory?";
    pub const PROMPT_STANDBY_WARNING: &'static str = "Switching on \"Standby list\" and \"Modified page list\" may freeze your computer for a moment. Are you sure you want to enable this option?";
    pub const PROMPT_RESET_CONFIRM: &'static str = "Are you sure you want to reset all application settings?";
    pub const STATUS_CLEANED: &'static str = "Memory was released successfully.";
    pub const STATUS_NO_PRIVILEGES: &'static str = "Administrator privileges required. Please run program as administrator!";
    
    // Updates
    pub const UPDATE_NO: &'static str = "No updates available.";
    pub const UPDATE_YES: &'static str = "Update available, download and install it?";
    pub const UPDATE_DOWNLOADING: &'static str = "Downloading update...";
    pub const UPDATE_DONE: &'static str = "Downloading update finished.";
    pub const UPDATE_ERROR: &'static str = "Update server connection error.";
}
