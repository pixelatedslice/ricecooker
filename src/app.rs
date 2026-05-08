use crate::config::rice_config::RiceConfig;

#[derive(Debug, Default)]
pub struct App {
    rice_configs: Vec<RiceConfig>,
    exit: bool,
}
