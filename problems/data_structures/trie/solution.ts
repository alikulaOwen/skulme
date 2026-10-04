class Trie {
    root: Record<string, any> = {};

    insert(word: string): void {
        let cur = this.root;
        for (const ch of word) {
            cur[ch] = cur[ch] || {};
            cur = cur[ch];
        }
        cur["#"] = true;
    }

    search(word: string): boolean {
        let cur = this.root;
        for (const ch of word) {
            if (!cur[ch]) return false;
            cur = cur[ch];
        }
        return !!cur["#"];
    }

    startsWith(prefix: string): boolean {
        let cur = this.root;
        for (const ch of prefix) {
            if (!cur[ch]) return false;
            cur = cur[ch];
        }
        return true;
    }
}

const t = new Trie();
t.insert("apple");
if (!t.search("apple") || t.search("app") || !t.startsWith("app")) throw new Error("Failed");
console.log("[PASS] TypeScript Trie passed!");
