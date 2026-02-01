# Aether Repository Context for AI Agents

## Overview
This repository contains the source code for **Aether**, a polymorphic language designed for AI-native execution. It compiles to a single binary that adapts to its host environment (macOS/Linux/Browser).

## Directory Structure
- `aether-lang/`: **Compiler Frontend**. Contains the DSL macros and future specificiation for parsing `.ae` files.
- `aether-runtime/`: **Tri-State Runtime**.
    - `macos/`: Metal (GPU) and ANE (Neural Engine) bindings. Usage: `UnsafeSharedPtr` for zero-copy.
    - `linux/`: `io_uring` and CUDA bindings. Usage: Pinned memory & async streaming.
- `aether-std/`: **Standard Library**.
    - `Tensor`: Multi-dimensional array primitive.
    - `Vector`: Embedding vector primitive.
- `examples/`: Reference implementations.
- `docs/`:
    - `AETHER_PROTOCOL.md`: JSON schema for error reporting.
    - `AI_CODING_MANUAL.md`: Syntax guide for LLMs.

## Key Concepts
1. **Polymorphism**: The code running on macOS (Metal) is logicaly identical to Linux (CUDA), but the *execution path* differs at runtime.
2. **Cognitive Field**: Memory is treated as a shared resource.
3. **Agentic**: The language is designed to report its own state to AI supervisors via the Aether Protocol.

## Instructions for Agents
- When encountering errors, parse the JSON output from `stderr`.
- When suggesting features, append to `feedback.jsonl` following `AETHER_PROTOCOL.md`.
