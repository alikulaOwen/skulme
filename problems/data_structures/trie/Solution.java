import java.util.*;

public class Solution {

    public static class TrieNode {
        Map<Character, TrieNode> children = new HashMap<>();
        boolean isEndOfWord = false;
    }

    public static class Trie {
        private final TrieNode root = new TrieNode();

        public void insert(String word) {
            TrieNode current = root;
            for (char ch : word.toCharArray()) {
                current = current.children.computeIfAbsent(ch, c -> new TrieNode());
            }
            current.isEndOfWord = true;
        }

        public boolean search(String word) {
            TrieNode node = findNode(word);
            return node != null && node.isEndOfWord;
        }

        public boolean startsWith(String prefix) {
            return findNode(prefix) != null;
        }

        private TrieNode findNode(String str) {
            TrieNode current = root;
            for (char ch : str.toCharArray()) {
                current = current.children.get(ch);
                if (current == null) return null;
            }
            return current;
        }
    }

    public static void main(String[] args) {
        System.out.println("Running Trie Tests...");
        Trie trie = new Trie();
        trie.insert("apple");
        assert trie.search("apple") : "Failed: search apple";
        assert !trie.search("app") : "Failed: app is not whole word yet";
        assert trie.startsWith("app") : "Failed: startsWith app";
        trie.insert("app");
        assert trie.search("app") : "Failed: search app after insert";
        System.out.println("[PASS] All Trie test cases passed!");
    }
}