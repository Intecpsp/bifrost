# Bifrost Developer Guide

Thank you for taking an interest in Bifrost!

This document serves as the guide for setting up a development environment,

## Development environment

### Setup

To develop Bifrost, you need the [rust language toolchain](https://rustup.rs/)
installed.

After that is done, you're ready to clone the repository and work on the code:

```sh
git clone https://github.com/chrivers/bifrost.git

cd bifrost

cargo build

# read the normal install guide, and then:
$EDITOR config.yaml

# run bifrost!
cargo run
```

You will **probably** get an error about binding to port 80 and 443. See the
next section for a solution.

### Developing as non-root

Bifrost needs to bind to ports 80 and 443 to run, which is normally only allowed
for the root user.

It is bad practice (and inconvenient) to develop as root.

On Linux, there is an easy and safe workaround:

```sh
# allow non-root users to bind low port number (lasts until reboot)
echo 0 | sudo tee /proc/sys/net/ipv4/ip_unprivileged_port_start
```

As a fallback option, you can compile Bifrost normally, and run the resulting
binary as root. You might be tempted to simply run `sudo cargo run`, but DO NOT
do that. It will cause all touched files to be owned by `root`, which is a mess
to work with.

Instead, compile using your normal user account, and use `sudo` only to run the
output:

```sh
cd bifrost

# debug mode:
cargo build
sudo target/debug/bifrost

# release mode:
cargo build -r
sudo target/release/bifrost
```

### Testing and QA

When developing code, or submitting a [pull
request](https://github.com/chrivers/bifrost/pulls), use these commands to make

```sh
# check code formatting
cargo fmt --check --all

# check that everything builds
cargo build --all-targets --workspace

# check that there are no clippy lints
cargo clippy --workspace --all-targets

# check that all tests pass
cargo test --workspace
```

## Submission guidelines (original work)

To have your improvement merged into Bifrost, there are a few simple, overall
requirements.

Bifrost is licenced as GPLv3. If your submission does not consist 100% of your
original work, please make your other sources *very* clear when submitting,
including what license (if any) the other work(s) is under.

In the common (and easy) case where your submission is all your own work, you
simply have to agree to have your work become part of Bifrost.

## Programming guidelines

### Style: `use` clauses

Bifrost follows a 4-block style of imports, where `use` imports are grouped into
4 categories:

 - `std` crates
 - imports from general crates
 - imports from bifrost workspace crates
 - `crate`-local imports

Each category should be separated by a blank line.

Here is an example:

```rust
use std::collections::HashMap;

use axum::Router;
use chrono::Utc;
use log::{info, warn};
use serde_json::{Value, json};
use uuid::Uuid;

use bifrost_api::backend::BackendRequest;
use hue::api::{Device, EntertainmentConfiguration};
use hue::error::{HueError, HueResult};
use hue::legacy_api::{ApiGroup, NewUser, NewUserReply};

use crate::error::{ApiError, ApiResult};
use crate::resource::Resources;
use crate::server::appstate::AppState;
```
