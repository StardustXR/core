# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is the **stardust-xr** core library repository, providing the fundamental connection library between Stardust XR servers and clients. It consists of three main crates organized as a Cargo workspace:

- **wire** (`stardust-xr-wire`): Base library for the Stardust XR display server with client/server connection handling
- **protocol** (`stardust-xr-protocol`): Protocol schemas generated from FlatBuffers and KDL protocol definitions
- **gluon** (`stardust-xr-gluon`): D-Bus object based utilities
- **fusion** (`stardust-xr-fusion`): High-level client library with abstractions for nodes and event loops

This is an XR (extended reality) framework that uses Unix domain sockets for client-server communication and supports spatial computing with 3D transformations, zones, input handling, and drawable objects.

## Development Commands

### Building
```bash
# Build all crates in workspace
cargo build

# Build with all features
cargo build --all-features
```

### Testing
```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p stardust-xr-fusion

# Run tests with tracing output
RUST_LOG=debug cargo test
```

### Code Quality
```bash
# Format code (uses hard tabs per rustfmt.toml)
cargo fmt

# Check formatting
cargo fmt --check

# Lint
cargo clippy

# Lint all targets and features
cargo clippy --all-targets --all-features
```

### Schema Generation
The schemas crate uses FlatBuffers for protocol definitions. To regenerate schemas:

```bash
# Set environment variable to enable schema regeneration
STARDUST_REGEN_FBS=1 cargo build -p stardust-xr-schemas
```

This requires `flatc` (FlatBuffers compiler) to be installed and available in PATH.

## Architecture

### Core Library Structure
- **Client/Server Connection**: Unix domain socket communication via `runtime_dir()` (typically `/run/user/{uid}/stardust-{instance}`)
- **Messaging**: Asymmetric messenger system handling FlatBuffer/FlexBuffer serialized messages
- **Scenegraph**: Trait-based system for managing spatial hierarchies and node relationships
- **Values**: Common data structures for 3D math (using `mint` for interoperability)

### Fusion High-Level API
- **Node System**: Hierarchical spatial nodes with aspects (traits) like `Spatial`, `Drawable`, `Input`
- **Event Loop**: Sync and async event loop patterns for handling server messages
- **Client**: Connection management and resource setup
- **Aspects**: Modular functionality via trait system (similar to components/mixins)

### Protocol System
- **KDL Protocols**: Human-readable protocol definitions in `schemas/src/protocol/*.kdl`
- **Code Generation**: Procedural macros generate Rust code from protocol definitions
- **FlatBuffers**: Binary serialization for message passing
- **Aspects**: Server-side and client-side method/signal definitions

### Key Concepts
- **Spatial Nodes**: 3D transforms with parent-child relationships
- **Zones**: Areas that can capture and manipulate spatial objects across clients
- **Aspects**: Trait-like functionality that can be composed on nodes
- **Signals**: One-way communication (client→server or server→client)
- **Methods**: Request-response communication patterns

## Environment Variables

- `STARDUST_INSTANCE`: Server instance number (default: 0)
- `STARDUST_RES_PREFIXES`: Resource path prefixes (colon-separated)
- `STARDUST_REGEN_FBS`: Enable FlatBuffer schema regeneration during build

## Coding Conventions

- Use hard tabs for indentation (configured in `.editorconfig` and `rustfmt.toml`)
- Follow standard Rust naming conventions
- Protocol definitions use KDL format with snake_case identifiers
- Generated code uses PascalCase for types and snake_case for functions
- Error handling via `color-eyre` and `thiserror`

## Resource Management

Resources are resolved using prefix paths configured via:
1. Runtime environment variable `STARDUST_RES_PREFIXES`
2. Compile-time environment variable `STARDUST_RES_PREFIXES`
3. Paths provided to `Client::setup_resources()`
4. Project-local resources via `project_local_resources!("path")` macro

## Dependencies

### Core Dependencies
- `tokio`: Async runtime with Unix socket support
- `serde`: Serialization framework
- `tracing`: Structured logging
- `mint`: 3D math type interoperability
- `color-eyre`: Error reporting

### Fusion-Specific
- `glam`: 3D mathematics library
- `zbus`: D-Bus integration for system services
- `xkbcommon`: Keyboard mapping (optional feature)

### Schemas-Specific
- `flatbuffers`: Binary serialization format
- `flexbuffers`: Dynamic/schema-less serialization
- `kdl`: Human-readable data format for protocols
- `zbus`: D-Bus interface generation

## Testing

Tests can be run individually:
```bash
cargo test --test client_connect
```

The project includes unit tests and integration tests. Some tests require a running Stardust XR server instance.
