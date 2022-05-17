# Author: hustcer
# Create: 2022/04/29 18:05:20
# Description:
#   Some helper task for setup-nu
# Ref:
#   1. https://github.com/casey/just
#   2. https://www.nushell.sh/book/

set shell := ['nu', '-c']

# The export setting causes all just variables
# to be exported as environment variables.

set export := true
set dotenv-load := true

# If positional-arguments is true, recipe arguments will be
# passed as positional arguments to commands. For linewise
# recipes, argument $0 will be the name of the recipe.

set positional-arguments := true

# Use `just --evaluate` to show env vars

# Used to handle the path seperator issue
SETUP_MUSD_PATH := parent_directory(justfile())
NU_DIR := parent_directory(`(which nu).path.0`)
_query_plugin := if os_family() == 'windows' { 'nu_plugin_query.exe' } else { 'nu_plugin_query' }

# To pass arguments to a dependency, put the dependency
# in parentheses along with the arguments, just like:
# default: (sh-cmd "main")

# List available commands by default
default:
  @just --list --list-prefix "··· "

# Run clippy, fmt tasks all in one time
all: fmt clippy
  @$'(ansi pb)All done!(ansi reset)'

# Format code
fmt:
  @$'(ansi g)Start `fmt` task...(ansi reset)'; \
  $'(ansi p)───────────────────────────────────────(ansi reset)'; \
  cargo fmt --all; \
  $'(ansi g)The `fmt` task finished!(ansi reset)(char nl)';

# Code linting
clippy:
  @$'(ansi g)Start `clippy` task...(ansi reset)'; \
  $'(ansi p)───────────────────────────────────────(ansi reset)'; \
  cargo clippy --all --all-features -- -D warnings -D clippy::unwrap_used -A clippy::needless_collect; \
  $'(ansi g)The `clippy` task finished!(ansi reset)(char nl)';

# Release a new version for `setup-nu`
release updateLog=('false'):
  @source {{ join(SETUP_MUSD_PATH, 'nu', 'common.nu') }}; \
    source {{ join(SETUP_MUSD_PATH, 'nu', 'release.nu') }}; \
    git-check --check-repo=1 {{SETUP_MUSD_PATH}}; \
    release --update-log={{updateLog}}

# Plugins need to be registered only once after nu v0.61
_setup:
  @register -e json {{ join(NU_DIR, _query_plugin) }}
