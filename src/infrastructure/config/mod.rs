use std::env;

use anyhow::Context as _;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
pub struct AppConfig {
    pub discord_bot_token: String,
    pub env_filter: EnvFilter,
    pub dev_mode: bool,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let dev_mode = dev_mode_enabled();
        let env_filter = build_env_filter(dev_mode);
        let discord_bot_token =
            env::var("DISCORD_BOT_TOKEN").context("DISCORD_BOT_TOKEN is not set")?;
        Ok(Self {
            discord_bot_token,
            env_filter,
            dev_mode,
        })
    }
}

fn dev_mode_enabled() -> bool {
    let Ok(value) = env::var("DEV_MODE") else {
        return false;
    };
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn build_env_filter(dev_mode: bool) -> EnvFilter {
    if let Ok(level) = env::var("LOG_LEVEL") {
        return EnvFilter::try_new(level).unwrap_or_else(|err| {
            eprintln!("invalid LOG_LEVEL; falling back to default: {err}");
            default_env_filter(dev_mode)
        });
    }

    if let Ok(filter) = EnvFilter::try_from_default_env() {
        return filter;
    }

    default_env_filter(dev_mode)
}

fn default_env_filter(dev_mode: bool) -> EnvFilter {
    let level = if dev_mode { "debug" } else { "info" };
    EnvFilter::new(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_mode_enabled_with_true_values() {
        for value in [
            "1", "true", "True", "TRUE", "yes", "Yes", "YES", "on", "On", "ON",
        ] {
            env::set_var("DEV_MODE", value);
            assert!(
                dev_mode_enabled(),
                "expected DEV_MODE={} to enable dev mode",
                value
            );
        }
        env::remove_var("DEV_MODE");
    }

    #[test]
    fn test_dev_mode_disabled_with_other_values() {
        for value in ["0", "false", "no", "off", "invalid"] {
            env::set_var("DEV_MODE", value);
            assert!(
                !dev_mode_enabled(),
                "expected DEV_MODE={} to disable dev mode",
                value
            );
        }
        env::remove_var("DEV_MODE");
    }

    #[test]
    fn test_dev_mode_disabled_when_not_set() {
        env::remove_var("DEV_MODE");
        assert!(!dev_mode_enabled());
    }

    #[test]
    fn test_dev_mode_with_whitespace() {
        env::set_var("DEV_MODE", "  true  ");
        assert!(dev_mode_enabled());
        env::remove_var("DEV_MODE");
    }

    #[test]
    fn test_default_env_filter_dev_mode() {
        // dev_mode=trueの場合、debugレベルでフィルタが作成される
        // EnvFilterの内部構造はテストできないため、
        // パニックしないことのみ確認
        let _filter = default_env_filter(true);
    }

    #[test]
    fn test_default_env_filter_production_mode() {
        // dev_mode=falseの場合、infoレベルでフィルタが作成される
        // EnvFilterの内部構造はテストできないため、
        // パニックしないことのみ確認
        let _filter = default_env_filter(false);
    }
}
