class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word: str) -> None:
        cur = self.root
        for ch in word:
            cur = cur.setdefault(ch, {})
        cur["#"] = True

    def search(self, word: str) -> bool:
        cur = self.root
        for ch in word:
            if ch not in cur: return False
            cur = cur[ch]
        return "#" in cur

    def starts_with(self, prefix: str) -> bool:
        cur = self.root
        for ch in prefix:
            if ch not in cur: return False
            cur = cur[ch]
        return True

if __name__ == "__main__":
    t = Trie()
    t.insert("apple")
    assert t.search("apple") is True
    assert t.search("app") is False
    assert t.starts_with("app") is True
    t.insert("app")
    assert t.search("app") is True
    print("[PASS] Python Trie passed!")
