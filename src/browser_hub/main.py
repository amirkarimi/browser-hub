import argparse
import os
import re
from typing import Optional


from browser_hub.processes import is_profile_active
from browser_hub.config import (
    Config,
    Profile,
    load_config_or_exit,
)


class BrowserHub:
    open_url: str
    config: Config

    def __init__(self, open_url: str, config: Config) -> None:
        self.open_url = open_url
        self.config = config

    def find_matched_profile(self) -> Optional[Profile]:
        for profile in self.config.profiles:
            matches = any(p in self.open_url for p in profile.url_patterns)
            if matches:
                return profile
        return None

    def should_open_last_active_profile(self) -> bool:
        for url in self.config.profile_specific_urls:
            if url in self.open_url:
                return True
        return False

    def transform_url(self, profile: Profile, url: str) -> str:
        for transformer in profile.url_transformers:
            keywords_matches = any(k for k in transformer.keywords if k in url)
            if keywords_matches:
                transformed_url = re.sub(
                    transformer.from_url_regex,
                    transformer.to_url,
                    url,
                )
                return transformed_url
        return url

    def find_active_profile(self) -> Optional[Profile]:
        for profile in self.config.profiles:
            active = is_profile_active(
                profile.browser.process_names,
                profile.browser.cmd_includes_regex,
                profile.browser.cmd_excludes_regex,
            )
            if active:
                return profile
        return None

    def open(self) -> None:
        profile = self.find_matched_profile()
        if not profile and self.should_open_last_active_profile():
            # Find the active profile based on the running process
            profile = self.find_active_profile()

        if profile:
            url = self.transform_url(profile, self.open_url)
            os.system(profile.browser.open_cmd.format(url=url))
        else:
            os.system(self.config.default_browser_open_cmd.format(url=self.open_url))


def cli() -> None:
    parser = argparse.ArgumentParser(
        prog="browser-hub",
        description="Open the right browser profile",
    )
    parser.add_argument("url")
    args = parser.parse_args()

    config = load_config_or_exit()

    BrowserHub(open_url=args.url, config=config).open()


if __name__ == "__main__":
    cli()
