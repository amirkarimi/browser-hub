import json
from os import path
from sys import stderr
from typing import List, Optional
from pydantic import BaseModel, Field, ValidationError

CONFIG_FILE_PATH = path.expanduser("~/.config/browser-hub/config.json")


class URLTransformer(BaseModel):
    keywords: List[str] = Field(min_length=1)
    from_url_regex: str
    to_url: str


class Browser(BaseModel):
    open_cmd: str
    process_names: List[str] = Field(min_length=1)
    cmd_includes_regex: str
    cmd_excludes_regex: Optional[str] = Field(default=None)


class Profile(BaseModel):
    name: str
    browser: Browser
    url_patterns: List[str] = Field(min_length=1)
    url_transformers: List[URLTransformer] = Field(default_factory=list)


class Config(BaseModel):
    default_browser_open_cmd: str
    profiles: List[Profile] = Field(min_length=1)
    # Open these URLs in the profile-specific browser if one is open
    profile_specific_urls: List[str]


def _err(msg: str) -> None:
    print(msg, file=stderr)


def load_config_or_exit() -> Config:
    if not path.exists(CONFIG_FILE_PATH):
        _err(f"Config file not found. Please create it at '{CONFIG_FILE_PATH}'.")
        exit(1)
    with open(CONFIG_FILE_PATH, "r") as f:
        data = json.load(f)
        try:
            config = Config(**data)
            return config
        except ValidationError as err:
            _err(f"Config file is not valid. File: '{CONFIG_FILE_PATH}'")
            for error in err.errors():
                msg = error["msg"]
                field = ".".join(str(f) for f in error["loc"])
                print(f"  {field}: {msg}")
            exit(1)
