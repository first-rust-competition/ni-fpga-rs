#!/bin/sh
set -euxo pipefail
docker run --userns host --rm --user 501:20 -e XARGO_HOME=/xargo -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e USER=cworley -e CROSS_RUNNER= -v /Users/cworley/.xargo:/xargo:Z -v /Users/cworley/.cargo:/cargo:Z -v /cargo/bin -v /Users/cworley/rio_playground/riotest:/project:Z -v /Users/cworley/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu:/rust:Z,ro -v /Users/cworley/rio_playground/riotest/target:/target:Z \
       -v /Users/cworley/rio_playground/ni_fpga-rs:/ni_fpga-rs -w /project -i -t firstrustcompetition/cross:2019.1.0 \
       sh -c "PATH=\$PATH:/rust/bin \"/rust/bin/cargo\" \"build\" \"--target=arm-unknown-linux-gnueabi\" \"--verbose\""
