#[cfg(feature = "system")] pub const ARX_PATH: &str = "/arx";
#[cfg(feature = "system")] pub const ARX_STORE: &str = "/arx/store";
#[cfg(feature = "system")] pub const ARX_BIN: &str = "/arx/bin";


#[cfg(feature = "user")] pub const ARX_PATH: &str = "/home/user/.arx";
#[cfg(feature = "user")] pub const ARX_STORE: &str = "/home/user/.arx/store";
#[cfg(feature = "user")] pub const ARX_BIN: &str = "/home/user/.arx/bin";