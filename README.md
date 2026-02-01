# Aether 

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-experimental-orange.svg)]()
[![AI-Native](https://img.shields.io/badge/AI-Native-purple.svg)](README-AI.md)

> **"Local First. Swarm Ready."**

Aether is a polymorphic programming language designed for the post-GPT era. It compiles into a single binary that exhibits "tri-state" behavior, optimizing itself for the hardware it wakes up on—whether that's a MacBook Pro, a Linux Server, or a Web Browser.

## 🧬 Philosophical DNA

### 1. The Tri-State Runtime
Code written in Aether adapts its execution path at runtime:
- **macOS (Neural Native)**: Bypasses Python/Torch and talks directly to **Metal** and the **Apple Neural Engine (ANE)**. Zero-copy tensors via Unified Memory.
- **Linux (Hyper-Container)**: Uses `io_uring` for massive async throughput and **CUDA/ROCm** for discrete GPU acceleration. Pinned memory pipelines.
- **Browser (Hologram)**: Compiles to Wasm/WebGPU for client-side inference.

### 2. The Cognitive Field
Memory is treated as a single, shared field. We eliminate the 50% energy waste of copying data between CPU RAM and GPU VRAM. In Aether, a `Tensor` is just a pointer to a shared memory region.

## 💻 Example

```rust
// Hardware-aware dispatch
@compute(target = "auto")
agent DocumentSynthesizer {

    // Persistent "Cognitive State"
    state {
        last_read_doc: Hash,
        embedding: Vector<f32, 1536>
    }

    fn digest(content: String) -> Result<Summary, CognitiveError> {
        // Native Vectorization (SIMD / NEON)
        let embedding = embed(content); 

        // Automatically routes to Neural Engine on Mac, CUDA on Linux
        let model = load_model("llama-7-optimize");

        thinking "Summarizing" {
            return model.predict(embedding);
        }
    }
}
```

## 🚀 Getting Started

### Prerequisites
- Rust Toolchain (1.70+)
- macOS (Apple Silicon recommended) OR Linux (x86_64/ARM64)

### Building
```bash
# Clone the repository
git clone https://github.com/GevorgAsryan/aether.git
cd aether

# Build the runtime
cargo build --release -p aether-runtime

# Run the example agent
cargo run --example document_synthesizer
```

## 🧠 AI-First Ecosystem
Aether is built to be written *by* AIs.
- **Protocol**: See [`docs/AETHER_PROTOCOL.md`](docs/AETHER_PROTOCOL.md) for our structured error reporting schema.
- **Manual**: Providing [`docs/AI_CODING_MANUAL.md`](docs/AI_CODING_MANUAL.md) to an LLM teaches it Aether syntax instantly.

## 📜 License
MIT License. See [LICENSE](LICENSE) for details.
