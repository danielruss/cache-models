# cache-models

`cache-models` downloads and caches the model files used by the SOCcerNET and
CLIPS occupational-classification applications. It retrieves the classifier
ONNX model and the associated Hugging Face embedding model ahead of time, so an
application can start without downloading models on its first run.

The program does not run inference and does not depend on ONNX Runtime.

## Install

Prebuilt binaries are available for Apple Silicon and Intel macOS, x86-64
Linux, and x86-64 Windows.

### macOS and Linux

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/danielruss/cache-models/releases/latest/download/cache-models-installer.sh | sh
```

### Windows PowerShell

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/danielruss/cache-models/releases/latest/download/cache-models-installer.ps1 | iex"
```

### From source

A current Rust toolchain is required:

```sh
cargo install --git https://github.com/danielruss/cache-models --tag v0.1.0
```

Confirm the installation with:

```sh
cache-models --version
```

## Usage

Running without a classifier caches the default SOCcerNET models:

```sh
cache-models
```

You can also select a classifier explicitly:

```sh
cache-models soccernet
cache-models clips
```

Each classifier currently supports version `1.0.0`:

```sh
cache-models soccernet --version 1.0.0
cache-models clips --version 1.0.0
```

The command is safe to run repeatedly. Files that are already cached are not
downloaded again. Download failures are reported on standard error and return a
nonzero exit status.

Run `cache-models --help` to see all available options.

## Clear the cache

```sh
cache-models --clear
```

This removes the cached SOCcerNET and CLIPS classifier files and the current
Hugging Face embedding model. Missing cache directories are treated as a
successful no-op.

## Cache locations

Classifier models are stored in a `soccernet` directory beneath the operating
system's standard user cache directory. Embedding-model files use the standard
cache managed by Hugging Face.

Downloads are written to temporary files and moved into place only after they
complete successfully, preventing interrupted downloads from being mistaken for
valid cached models.
