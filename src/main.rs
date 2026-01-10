mod config;
mod processes;

use crate::config::{Config, Profile, load_config};
use clap::{arg, command};
use processes::is_profile_active;
use regex::Regex;
use std::process::{Command, ExitStatus};
use std::{env, io};

struct BrowserHub {
    open_url: String,
    config: Config,
}

impl BrowserHub {
    fn new(open_url: String, config: Config) -> Self {
        Self { open_url, config }
    }

    fn find_matched_profile(&self) -> Option<&Profile> {
        self.config.profiles.iter().find(|profile| {
            profile
                .url_patterns
                .iter()
                .any(|p| self.open_url.contains(p))
        })
    }

    fn should_open_last_active_profile(&self) -> bool {
        for url in &self.config.profile_specific_urls {
            if self.open_url.contains(url) {
                return true;
            }
        }
        false
    }

    fn transform_url(&self, profile: &Profile, url: &str) -> String {
        for transformer in &profile.url_transformers {
            if transformer.keywords.iter().any(|k| url.contains(k)) {
                let re = Regex::new(&transformer.from_url_regex).unwrap();
                let transformed_url = re.replace_all(url, transformer.to_url.as_str());
                // Stop after the first matching transformer
                return transformed_url.into_owned();
            }
        }
        url.to_string()
    }

    fn find_active_profile(&self) -> Option<&Profile> {
        for profile in &self.config.profiles {
            let active = is_profile_active(
                &profile.browser.process_names,
                &profile.browser.cmd_includes_regex,
                &profile.browser.cmd_excludes_regex,
            );
            if active {
                return Some(profile);
            }
        }
        None
    }

    fn open(&self) -> io::Result<ExitStatus> {
        let mut profile = self.find_matched_profile();
        if profile.is_none() && self.should_open_last_active_profile() {
            profile = self.find_active_profile();
        }

        let cmd = match profile {
            Some(p) => {
                let url = self.transform_url(p, &self.open_url);
                p.browser.open_cmd.replace("{url}", &url)
            }
            None => self
                .config
                .default_browser_open_cmd
                .replace("{url}", &self.open_url),
        };

        Command::new("sh").arg("-c").arg(&cmd).status()
    }
}

fn cli() {
    let matches = command!()
        .arg(arg!([url] "Optional URL to open"))
        .get_matches();

    let url = matches
        .get_one("url")
        .unwrap_or(&"".to_string())
        .to_string();

    let config = load_config().unwrap();

    let hub = BrowserHub::new(url, config);
    hub.open().expect("Failed to open URL");
}

fn main() {
    cli();
}
