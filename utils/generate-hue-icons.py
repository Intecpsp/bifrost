#!/usr/bin/env python3

# curl https://raw.githubusercontent.com/arallsopp/hass-hue-icons/refs/heads/main/dist/hass-hue-icons.js | utils/generate-hue-icons.py | rustfmt > crates/bifrost-frontend/src/hue_icons.rs

import os
import sys
import json

HEADER = r"""
//! GENERATED FILE - DO NOT EDIT
//!
//! This file is derived from hass-hue-icons
//!
//!   <https://github.com/arallsopp/hass-hue-icons>
//!
//! These icons are licensed under Creative Commons:
//!
//!   [CC BY-NC-SA 4.0](http://creativecommons.org/licenses/by-nc-sa/4.0/)
"""

def upcase(name):
    return name.upper().replace("-", "_")

def main():
    input = sys.stdin.read()
    body = input.split("};\n", 1)[0].split("{", 1)[1]
    body = body.replace("path:", '"path":').replace("keywords:", '"keywords":')

    js = f"{{ {body} }}"

    data = json.loads(js)

    print(HEADER)

    for key, value in data.items():
        name = upcase(key)
        print(f"pub const {name}: &str = \"{value['path']}\";\n")

if __name__ == "__main__":
    sys.exit(main())
