# LEGACY CODE ARCHIVE

## File: code_runner_secure.py

**Status:** SUPERSEDED  
**Replaced By:** code_runner.py (nsjail-based hardened version)

---

## Overview

This file implemented a Docker-based sandbox for Rust code execution. It was replaced by a more robust nsjail-based solution providing stronger isolation and faster startup times.

### Migration

To use the new implementation:
1. Install nsjail: `apt-get install nsjail`
2. Copy config: `cp deploy/nsjail/rust_exec.cfg /etc/nsjail/`
3. Update imports to use `code_runner.py`

---

**Note:** Use `code_runner.py` for all production deployments.
