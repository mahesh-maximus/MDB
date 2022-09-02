#!/bin/sh

rm -fr /usr/local/rust/registry

tmux
exec "$@"
