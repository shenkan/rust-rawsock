/// Default paths (os-specific) where the wpcap dynamically loaded library is stored.
#[cfg(windows)]
pub const DEFAULT_PATHS: [&'static str; 4] = [
    "C:\\Windows\\System32\\Npcap\\Packet.dll",
    "Packet.dll",
    "C:\\Windows\\System32\\Npcap\\wpcap.dll",
    "wpcap.dll"
];

/// Default paths (os-specific) where the wpcap dynamically loaded library is stored.
#[cfg(not(windows))]
pub const DEFAULT_PATHS: [&'static str; 0] = [];