#!/usr/bin/env bash

NAME=${TRAVIS_REPO_SLUG#*/}

function main {
    rustup target add $TARGET
    cargo build --target $TARGET --release
    tar -czf $NAME-$TRAVIS_TAG-$TARGET.tar.gz -C ./target/$TARGET/release/ $NAME
}

main
