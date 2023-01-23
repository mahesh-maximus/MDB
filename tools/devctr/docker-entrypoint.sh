#!/bin/sh

rm -fr /usr/local/rust/registry

# Commented due to scrolling issue.
# tmux

clear
fish

exec "$@"
