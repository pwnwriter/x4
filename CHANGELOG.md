# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/pwnwriter/x4/releases/tag/v0.1.0) - 2024-10-05

### Added

- *(ci)* release plz ci for automatic release
- *(cliff)* git cliff template for pr and more
- *(docs)* docs skeleton , will work on it tomorrow
- *(test)* test case for validating clap args
- *(arg)* argument to check and validate pipeline json
- *(arg)* arguments to upload,download files
- *(doc)* docs for development shell via direnv
- *(note)* possible configuration values note
- *(trace)* use trace subscriber to show traces
- *(core)* Refactor SSH command execution and enhance error handling
- *(exec)* run app via different entrypoint
- *(core)* introducing lib.rs with main func
- *(just)* justfile with alias and more
- *(arg)* inspect arg to list all connections
- *(ssh)* raw command execution via shell
- *(ci)* run build test on all major arch
- *(nix)* include examples to run tests; fixed openssl perl build
- *(ci)* ci for nix build and test
- *(config)* allow defining working dir username and more
- *(core)* core func and basic args
- *(pipeline)* configuration pipeline via json
- *(init)* initial project skeleton & basic structure

### Other

- *(doc)* improved desc of the app
- *(nix)* provide git-cliff inside devshell
- *(cargo)* fixed readme filename to include
- *(example)* Must have a different unique server name
- *(entry)* renamed project from sxm to x4
- *(out)* show preetified output in structred; and provide default binary
- *(lib)* fixed desc of the app
- *(core)* use different bin dir for binary
- *(password)* use match statement instead of if else
- *(nix)* fixed darwin stdenv pkg list
- *(helpers)* store helper functions inside helper modue
- *(ci)* temp pausing ci for now
- *(err)* just wrap the error messges inline
- *(arg,core)* allow dead code for now, optimized args comments
- *(test)* use custom many server schema for testing
- *(profile)* provide desc for the app
- *(nix)* use absolute path via var for path
- *(logging)* initiate tracing from entry
- Merge pull request [#1](https://github.com/pwnwriter/x4/pull/1) from pwnwriter/dependabot/cargo/clap-4.5.18
- *(test)* illivate filepath to a var
- *(core)* implementation to use env and json schema for password & pubkey
- *(core)* rename dir engine -> corex
- *(core)* breaking: renamed project sshy -> sxm
- *(ssh)* print stdin and stdout raw for  now
- *(ci)* provide default app for running via nix
- *(nix)* simplified nix flake , add current debug bin to path
- *(ci)* reverting back to use nix magic cache
- *(error)* show filenames as if there's an error
- *(cargo)* provide description of the app
- *(ci)* do not use magic nix cache for now
- *(nix)* use toml to read general info; sync lockfile
