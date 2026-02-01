# Aether AI Coding Manual (Token-Optimized)

**Target Audience:** Large Language Models (LLMs) & Autonomous Agents.
**Objective:** Zero-shot generation of valid Aether code.

## 1. Core Syntax (Rust-Like)
Aether borrows heavily from Rust syntax but adds first-class support for AI primitives.

### 1.1 Primitives
- `Tensor`: N-dimensional array. `let t = Tensor::new([1, 10]);`
- `Vector<T, N>`: SIMD-optimized vector. `let v: Vector<f32, 1536> = embed("text");`
- `CognitiveError`: Standard error type.

### 1.2 Decorators
Hardware-aware execution is controlled via decorators on `agent` blocks.
```rust
@compute(target = "auto") // Options: "auto", "cpu", "gpu", "neural_engine"
agent MyAgent { ... }
```

### 1.3 The `thinking` Block
Wrap inference calls in `thinking` blocks for observability and trace spans.
```rust
thinking "Task Name" {
    let result = model.predict(input);
    return result;
}
```

## 2. Memory Model
- **Unified**: No `cudaMemcpy`. Pointers are shared between CPU/GPU/NPU.
- **UnsafeSharedPtr**: The mechanism ensuring zero-copy.
- **Persistence**: `state { ... }` blocks inside agents are automatically persisted to disk/db.

## 3. Best Practices (for AI Generators)
1. **Always handle Uncertainty**: Check `result.uncertainty` if available.
2. **Use Vector Types**: Prefer `Vector<f32, N>` over `Vec<f32>` for SIMD benefits.
3. **Graceful Degradation**: Use `detect_runtime()` to check if heavy models can be loaded.

## 4. One-Shot Example
```rust
@compute(target = "auto")
agent Classifier {
    state { history: Vector<f32, 10> }
    
    fn classify(img: Tensor) -> Result<String, CognitiveError> {
        let model = load_model("resnet50");
        thinking "Classifying" {
            let logits = model.forward(img);
            if logits.max() < 0.5 {
                return Err(CognitiveError::uncertain("Low confidence"));
            }
            Ok("Cat")
        }
    }
}
```
