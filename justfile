set windows-shell := ["powershell.exe", "-NoProfile", "-NoLogo", "-Command"]
set shell := ["bash", "-c"]

@_default:
    just --list

build:
    @cargo build --release
