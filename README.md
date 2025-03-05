# README

This is the repo for Pulse. An AI driven personal health coach.

## Project Overview

This project is my personal solution to transform complex biometric information into clear, actionable insights that will help me optimize my health, performance, and daily life.

## Features (MVP)

- Securely store session tokens for Wearable API.
- Fetch and process biometric data (HRV, steps, sleep, etc.).
- AI-driven health coaching via CLI.
- Local data storage for quick retrieval.
- Design for future scalability into a full web API.

## Getting Started

1. Clone the Repository:

```
git clone https://github.com/bernardmuller/pulse.git
cd your-repo
```

2. Install Rust (if not installed):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Run the CLI App:

```
cargo run
```

## Roadmap

- Phase 1: CLI Prototype (Data ingestion, processing, AI insights)
- Phase 2: Web API (Rust-based backend)
- Phase 3: Cross-platform app (React with Tauri or Flutter)
