## ADDED Requirements

### Requirement: Integration test script builds and runs all notification types
The script SHALL build the example binary with `cargo build --example showcase` and then run it with each notification type argument, reporting pass/fail for each.

#### Scenario: Successful build and run
- **WHEN** the script is executed on a macOS system with Rust installed
- **THEN** it builds the example binary and runs each notification type, printing status for each

#### Scenario: Build failure
- **WHEN** the cargo build fails
- **THEN** the script exits immediately with an error message

### Requirement: Integration test script runs notifications concurrently
The script SHALL launch notification commands in the background using `&` and `wait` for all to complete, to test that multiple concurrent notifications work.

#### Scenario: Concurrent execution
- **WHEN** the script runs
- **THEN** multiple notification types are launched in parallel background processes

### Requirement: Integration test script reports results
The script SHALL print a summary showing how many tests passed and failed, and exit with code 0 if all passed or code 1 if any failed.

#### Scenario: All pass
- **WHEN** all notification commands exit successfully
- **THEN** the script prints a success summary and exits with code 0

#### Scenario: Some fail
- **WHEN** one or more notification commands fail
- **THEN** the script prints which ones failed and exits with code 1

### Requirement: Integration test script is executable
The script SHALL have a shebang line (`#!/usr/bin/env bash`) and be marked executable.

#### Scenario: Direct execution
- **WHEN** a user runs `./integration-tests.sh`
- **THEN** the script executes without needing to prefix with `bash`
