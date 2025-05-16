**`gguf-info`**

* Simple, clear, UNIX-style
* Easily searchable and guessable by others
* Aligns with tools like `ffprobe`, `pdfinfo`, etc.

---

### ✅ Short Description

> A CLI tool to inspect and extract metadata from GGUF language model files.

---

### ✅ `README.md`

````markdown
# gguf-info

**gguf-info** is a command-line tool for inspecting [GGUF](https://github.com/ggerganov/ggml/blob/master/docs/gguf.md) (GGML Unified Format) language model files.

It extracts and prints essential metadata like model architecture, parameter count, context size, embedding size, and more — all in JSON format.

## 🔧 Features

- Supports all GGUF-compatible models (LLaMA, Qwen, Mistral, etc.)
- JSON output for easy scripting and integration
- Smart fallback to known metadata prefixes (`qwen3`, `llama`, `mistral`, etc.)
- Optional `--metadata` flag to output full raw metadata

## 📦 Installation

```bash
git clone https://github.com/rbehzadan/gguf-info.git
cd gguf-info
cargo build --release
````

The resulting binary will be in `./target/release/gguf-info`.

## 🚀 Usage

```bash
gguf-info --model /path/to/model.gguf
```

Default output:

```json
{
  "model_architecture": "llama",
  "number_of_parameters": "7B",
  "context_length": 8192,
  "embedding_length": 4096,
  ...
}
```

Print full metadata:

```bash
gguf-info --model /path/to/model.gguf --metadata
```

## 🧠 Why GGUF?

GGUF is a modern file format for quantized large language models, used in `llama.cpp`, `ggml`, and tools like `llm` and `ollama`.

## 📜 License

MIT

---

Created by [Reza Behzadan](https://github.com/rbehzadan)

