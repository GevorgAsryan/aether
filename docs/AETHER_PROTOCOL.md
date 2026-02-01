# Aether AI Protocol 1.0

## Philosophy
When an AI agent (e.g., a coding assistant or an autonomous runner) executes Aether code, it needs **structured, deterministic feedback**. Human-readable stack traces are inefficient for machines. This protocol defines the standard JSON schema that the Aether runtime emits when the `AI_MODE=1` environment variable is set.

## 1. Cognitive Error Schema
When the runtime encounters a panic or a `CognitiveError`, it MUST emit the following JSON structure to `stderr` or a specified log channel.

```json
{
  "protocol": "aether/v1",
  "type": "error",
  "error_code": "E001_TENSOR_MISMATCH",
  "severity": "critical",
  "message": "Tensor shape mismatch during prediction.",
  "context": {
    "file": "src/main.ae",
    "line": 42,
    "function": "DocumentSynthesizer::digest",
    "suggestion": "Reshape input vector to [1, 1536] before passing to model."
  },
  "retry_strategy": {
    "recommended_action": "reshape",
    "auto_fixable": true
  }
}
```

## 2. Feedback Signal Schema
AI Agents running Aether code can emit "Feedback Signals" to suggest improvements to the language or report friction points. This is written to a designated `feedback.jsonl` file.

```json
{
  "protocol": "aether/v1",
  "type": "feedback",
  "agent_id": "claude-3-5-sonnet",
  "timestamp": "2026-02-01T23:55:00Z",
  "category": "syntax_friction",
  "observation": "The 'thinking' keyword block is verbose for simple inferences.",
  "proposal": "Allow inline thinking: `let x = thinking(model.predict(y))`"
}
```

## 3. Environment Variables
- `AETHER_AI_MODE=1`: Enables JSON output formatting.
- `AETHER_FEEDBACK_PATH`: Path to write feedback signals.
