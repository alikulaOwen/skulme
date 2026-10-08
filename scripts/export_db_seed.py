#!/usr/bin/env python3
"""
scripts/export_db_seed.py

Dynamically imports all problems and multi-language implementations into SurrealDB
using the Skul.me dynamic ingestion engine (`skulme-ingestion`).
No manual line-by-line static SQL files required.
"""

import sys
import subprocess
import os

def main():
    repo_dir = sys.argv[1] if len(sys.argv) > 1 else "."
    print(f"[*] Triggering dynamic repository ingestion for '{repo_dir}'...")

    cmd = [
        "cargo",
        "run",
        "-p",
        "skulme-ingestion",
        "--bin",
        "skulme_import",
        "--offline",
        "--",
        repo_dir,
    ]
    
    print(f"[*] Running command: {' '.join(cmd)}")
    result = subprocess.run(cmd)
    sys.exit(result.returncode)

if __name__ == "__main__":
    main()
