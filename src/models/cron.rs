#[derive(Debug, Clone)]
pub struct CronTaskConfig {
    pub name: String,
    pub cron_exp: String,
    pub path: String,
    pub timeout: u32,
}

impl Default for CronTaskConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            cron_exp: String::new(),
            path: String::new(),
            timeout: 0,
        }
    }
}

pub type CronTaskConfigs = Vec<CronTaskConfig>;
