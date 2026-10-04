import os
import re
import sys

def slug_to_title(slug):
    # Special acronyms
    acronyms = {
        "aes": "AES", "rsa": "RSA", "bfs": "BFS", "dfs": "DFS", "dsu": "DSU",
        "bst": "BST", "avl": "AVL", "lcs": "LCS", "lis": "LIS", "kmp": "KMP",
        "dp": "DP", "gcd": "GCD", "lcm": "LCM", "fft": "FFT", "sha256": "SHA-256",
        "md5": "MD5", "rot13": "ROT13", "cmyk": "CMYK", "hsv": "HSV", "rgb": "RGB",
        "ipv4": "IPv4", "ipv6": "IPv6", "lz77": "LZ77", "bwt": "BWT"
    }
    parts = slug.split("_")
    title_parts = []
    for p in parts:
        lower = p.lower()
        if lower in acronyms:
            title_parts.append(acronyms[lower])
        else:
            title_parts.append(p.capitalize())
    return " ".join(title_parts)

def extract_problem_data(rs_path):
    with open(rs_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()

    lines = content.splitlines()

    # 1. Extract module doc or leading comments
    doc_lines = []
    for l in lines:
        s = l.strip()
        if s.startswith("//!"):
            doc_lines.append(s[3:].strip())
        elif doc_lines and not s.startswith("//!"):
            break
            
    if not doc_lines:
        pub_idx = -1
        for i, l in enumerate(lines):
            if re.match(r"^\s*pub\s+(?:fn|struct|enum)\b", l):
                pub_idx = i
                break
        if pub_idx > 0:
            for i in range(pub_idx - 1, -1, -1):
                s = lines[i].strip()
                if s.startswith("///"):
                    doc_lines.append(s[3:].strip())
                elif s.startswith("//") and not s.startswith("//#"):
                    doc_lines.append(s[2:].strip())
                elif s == "" and doc_lines:
                    doc_lines.append("")
                else:
                    break
            doc_lines = list(reversed(doc_lines))

    if not doc_lines:
        for l in lines:
            s = l.strip()
            if s.startswith("//"):
                doc_lines.append(s[2:].strip())
            elif doc_lines and not s.startswith("//"):
                break

    doc = "\n".join(doc_lines).strip()

    # Extract pub fns
    pub_fns = re.findall(r"pub\s+fn\s+([a-zA-Z0-9_]+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)\s*(?:->\s*([^{]+))?", content)

    # Extract pub structs
    pub_structs = re.findall(r"pub\s+(?:struct|enum)\s+([a-zA-Z0-9_]+)", content)

    # Extract time/space complexity if mentioned
    time_comp = "O(N)"
    space_comp = "O(1)"
    m_time = re.search(r"[Tt]ime\s*(?:complexity)?\s*[:=-]\s*([^\n\r]+)", content)
    if m_time:
        time_comp = m_time.group(1).strip()
    m_space = re.search(r"[Ss]pace\s*(?:complexity)?\s*[:=-]\s*([^\n\r]+)", content)
    if m_space:
        space_comp = m_space.group(1).strip()

    return {
        "doc": doc,
        "pub_fns": pub_fns,
        "pub_structs": pub_structs,
        "time_complexity": time_comp,
        "space_complexity": space_comp,
        "raw_content": content
    }

print("Extractor module compiled.")
