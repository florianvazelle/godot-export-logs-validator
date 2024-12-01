# === Aliases ===

[private]
alias c := cargo

# === Variables ===

# Python virtualenv
venv_dir := justfile_directory() / "venv"

# === Commands ===

# Display all commands
@default:
  just --list

# Python virtualenv wrapper
[private]
@venv *ARGS:
    [ ! -d {{ venv_dir }} ] && python3 -m venv {{ venv_dir }} || true
    . {{ venv_dir }}/bin/activate && {{ ARGS }}

# Download Cargo
[private]
@install-cargo:
  [ ! $(command -v cargo) ] && curl https://sh.rustup.rs -sSf | sh || true

# Cargo binary wrapper
cargo *ARGS: install-cargo
  cargo {{ ARGS }}

# Run files formatters
@fmt *ARGS:
  just venv pip install pre-commit==4.*
  just venv pre-commit run -a

# Run unit tests
@test:
  just cargo test

# Remove cache and binaries created by this Justfile
clean:
  git clean -f -X -d
