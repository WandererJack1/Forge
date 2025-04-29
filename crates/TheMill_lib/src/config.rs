use once_cell::sync::OnceCell;
use TheMill_internal::config::Config;

pub static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();
