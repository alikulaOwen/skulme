#!/usr/bin/env python3
"""
scripts/export_db_seed.py

Reads all problems in `problems/<category>/<problem>/` and exports:
1. `database/seed_surreal.surql`: SurrealQL statements creating `concept`, `implementation`,
   and `RELATE implementation->IMPLEMENTS->concept` records according to the restored schema.
"""

import os
import glob
import re

PROBLEMS_DIR = "problems"
OUT_SURQL = "database/seed_surreal.surql"

def escape_str(s: str) -> str:
    return s.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n")

def main():
    os.makedirs("database", exist_ok=True)
    statements = [
        "-- Skul.me Seed Data for SurrealDB",
        "USE NAMESPACE skulme;",
        "USE DATABASE skulme_core;",
        "",
        "-- Create Upstream Repositories",
        "CREATE repo:rust SET name = 'Rust', url = 'https://github.com/TheAlgorithms/Rust', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "CREATE repo:python SET name = 'Python', url = 'https://github.com/TheAlgorithms/Python', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "CREATE repo:java SET name = 'Java', url = 'https://github.com/TheAlgorithms/Java', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "CREATE repo:cpp SET name = 'C-Plus-Plus', url = 'https://github.com/TheAlgorithms/C-Plus-Plus', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "CREATE repo:go SET name = 'Go', url = 'https://github.com/TheAlgorithms/Go', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "CREATE repo:typescript SET name = 'TypeScript', url = 'https://github.com/TheAlgorithms/TypeScript', default_branch = 'master', last_commit_sha = 'HEAD', license_spdx = 'MIT';",
        "",
    ]

    count = 0
    categories = sorted(os.listdir(PROBLEMS_DIR))
    for cat in categories:
        cat_path = os.path.join(PROBLEMS_DIR, cat)
        if not os.path.isdir(cat_path):
            continue

        for prob in sorted(os.listdir(cat_path)):
            prob_path = os.path.join(cat_path, prob)
            if not os.path.isdir(prob_path):
                continue

            readme_file = os.path.join(prob_path, "README.md")
            if not os.path.exists(readme_file):
                continue

            slug = f"{cat}_{prob}"
            title = prob.replace("_", " ").title()

            # Read README
            with open(readme_file, "r", encoding="utf-8", errors="ignore") as f:
                desc = f.read()

            esc_desc = escape_str(desc[:500]) # First 500 chars summary
            statements.append(
                f"CREATE concept:{slug} SET slug = '{slug}', title = '{escape_str(title)}', category = '{escape_str(cat)}', description = '{esc_desc}', status = 'published';"
            )

            # Check for implementations
            for lang, ext, repo in [
                ("rust", "solution.rs", "repo:rust"),
                ("python", "solution.py", "repo:python"),
                ("java", "Solution.java", "repo:java"),
                ("typescript", "solution.ts", "repo:typescript"),
            ]:
                code_file = os.path.join(prob_path, ext)
                if os.path.exists(code_file):
                    impl_id = f"{slug}_{lang}"
                    with open(code_file, "r", encoding="utf-8", errors="ignore") as f:
                        source = f.read()
                    esc_source = escape_str(source[:2000]) # Sample code for seed
                    statements.append(
                        f"CREATE implementation:{impl_id} SET language = '{lang}', source_code = '{esc_source}', ast_hash = 'seed_hash', commit_sha = 'HEAD', original_authors = ['TheAlgorithms Community'], keywords = ['{cat}', '{prob}'], license_spdx = 'MIT', origin_repo = '{repo}', origin_path = '{prob_path}';"
                    )
                    statements.append(
                        f"RELATE implementation:{impl_id}->IMPLEMENTS->concept:{slug};"
                    )

            count += 1

    with open(OUT_SURQL, "w", encoding="utf-8") as f:
        f.write("\n".join(statements) + "\n")

    print(f"Exported {count} canonical concepts and implementations to {OUT_SURQL}")

if __name__ == "__main__":
    main()
