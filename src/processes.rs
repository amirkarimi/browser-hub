use sysinfo::System;
use regex::Regex;

pub fn is_profile_active(
    process_names: &Vec<String>,
    cmd_includes_regex: &str,
    cmd_excludes_regex: &Option<String>,
) -> bool {
    let sys = System::new_all();
    let include_re = Regex::new(cmd_includes_regex).unwrap();
    let exclude_re = match cmd_excludes_regex {
        Some(pattern) => Some(Regex::new(pattern).unwrap()),
        None => None,
    };

    for process in sys.processes().values() {
        let process_name = process
            .name()
            .to_str()
            .unwrap_or("");

        let process_found = process_names
            .iter()
            .any(|name| name == process_name);

        if process_found {
            let cmdline = process
                .cmd()
                .iter()
                .filter_map(|c| c.to_str())
                .collect::<Vec<&str>>()
                .join(" ");

            let matches_include = include_re.is_match(&cmdline);
            let matches_exclude = match &exclude_re {
                Some(ex_re) => ex_re.is_match(&cmdline),
                None => false,
            };

            if matches_include && !matches_exclude {
                return true;
            }
        }
    }
    false
}
