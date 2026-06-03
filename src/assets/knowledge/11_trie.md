# 11. Trie (Prefix Tree)

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 2 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "autocomplete", "search suggestions" | Trie |
| "word search", "prefix search" | Trie |
| "implement dictionary", "add/search words" | Trie |
| "longest common prefix" | Trie |
| "replace words with root" | Trie |
| "starts with prefix" | Trie |
| "word squares", "palindrome pairs" | Trie (advanced) |

---

## 📐 Структура Trie

```
Trie для слів: ["cat", "car", "card", "care", "dog"]

        root
       /    \
      c      d
      |      |
      a      o
     / \     |
    t*  r    g*
        |
        d*  e*
```

- Кожен вузол = символ
- `*` = кінець слова (`isEnd = true`)
- Кожен вузол має до 26 дітей (для lowercase a-z)

---

## 📐 Базова реалізація Trie

```java
class TrieNode {
    TrieNode[] children = new TrieNode[26];
    boolean isEnd = false;
}

class Trie {
    private TrieNode root = new TrieNode();

    // Insert: O(m) де m — довжина слова
    public void insert(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            int idx = c - 'a';
            if (curr.children[idx] == null) {
                curr.children[idx] = new TrieNode();
            }
            curr = curr.children[idx];
        }
        curr.isEnd = true;
    }

    // Search: O(m)
    public boolean search(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            int idx = c - 'a';
            if (curr.children[idx] == null) return false;
            curr = curr.children[idx];
        }
        return curr.isEnd; // слово має завершуватись тут
    }

    // StartsWith: O(m)
    public boolean startsWith(String prefix) {
        TrieNode curr = root;
        for (char c : prefix.toCharArray()) {
            int idx = c - 'a';
            if (curr.children[idx] == null) return false;
            curr = curr.children[idx];
        }
        return true; // не перевіряємо isEnd — достатньо що prefix існує
    }
}
```

---

## 📐 Патерн 1: Implement Trie

### Задача 1: Implement Trie (Prefix Tree) (LeetCode #208)

```java
class Trie {
    private TrieNode root;

    public Trie() {
        root = new TrieNode();
    }

    public void insert(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) curr.children[i] = new TrieNode();
            curr = curr.children[i];
        }
        curr.isEnd = true;
    }

    public boolean search(String word) {
        TrieNode node = traverse(word);
        return node != null && node.isEnd;
    }

    public boolean startsWith(String prefix) {
        return traverse(prefix) != null;
    }

    // Допоміжний метод — повертає вузол після обходу рядка
    private TrieNode traverse(String s) {
        TrieNode curr = root;
        for (char c : s.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) return null;
            curr = curr.children[i];
        }
        return curr;
    }
}

class TrieNode {
    TrieNode[] children = new TrieNode[26];
    boolean isEnd = false;
}
```
**Складність:** O(m) insert/search/startsWith де m — довжина слова

---

## 📐 Патерн 2: Search with Wildcards

### Задача 2: Design Add and Search Words (LeetCode #211)
**Умова:** Додавати слова і шукати з wildcard `.` (будь-який символ).

**Ключова думка:** При `.` — рекурсивно перевіряємо **всіх** дітей.

```java
class WordDictionary {
    private TrieNode root = new TrieNode();

    public void addWord(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) curr.children[i] = new TrieNode();
            curr = curr.children[i];
        }
        curr.isEnd = true;
    }

    public boolean search(String word) {
        return dfs(word, 0, root);
    }

    private boolean dfs(String word, int idx, TrieNode node) {
        if (idx == word.length()) return node.isEnd;

        char c = word.charAt(idx);

        if (c == '.') {
            // перевіряємо всіх дітей
            for (TrieNode child : node.children) {
                if (child != null && dfs(word, idx + 1, child)) return true;
            }
            return false;
        } else {
            TrieNode child = node.children[c - 'a'];
            return child != null && dfs(word, idx + 1, child);
        }
    }
}
```

---

## 📐 Патерн 3: Word Search у Grid + Trie

### Задача 3: Word Search II (LeetCode #212)
**Умова:** Grid з літерами. Знайти всі слова зі словника що можна скласти з сусідніх клітинок.

**Ключова думка:** Будуємо Trie зі словника → DFS по grid перевіряє prefix у Trie одночасно.  
Набагато ефективніше ніж DFS для кожного слова окремо.

```java
public List<String> findWords(char[][] board, String[] words) {
    // Будуємо Trie
    TrieNode root = new TrieNode();
    for (String word : words) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) curr.children[i] = new TrieNode();
            curr = curr.children[i];
        }
        curr.word = word; // зберігаємо слово у кінцевому вузлі
    }

    List<String> result = new ArrayList<>();
    int rows = board.length, cols = board[0].length;

    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            dfs(board, r, c, root, result);
        }
    }

    return result;
}

private void dfs(char[][] board, int r, int c, TrieNode node, List<String> result) {
    if (r < 0 || r >= board.length || c < 0 || c >= board[0].length) return;

    char ch = board[r][c];
    if (ch == '#' || node.children[ch - 'a'] == null) return; // '#' = відвіданий

    node = node.children[ch - 'a'];

    if (node.word != null) {
        result.add(node.word);
        node.word = null; // уникаємо дублікатів
    }

    board[r][c] = '#'; // позначаємо як відвіданий

    dfs(board, r + 1, c, node, result);
    dfs(board, r - 1, c, node, result);
    dfs(board, r, c + 1, node, result);
    dfs(board, r, c - 1, node, result);

    board[r][c] = ch; // відновлюємо
}

// TrieNode з полем word
class TrieNode {
    TrieNode[] children = new TrieNode[26];
    String word = null; // не null якщо тут закінчується слово
}
```
**Складність:** O(M * 4 * 3^(L-1)) де M — клітинки, L — довжина слова

---

## 📐 Патерн 4: Replace Words (Trie для пошуку кореня)

### Задача 4: Replace Words (LeetCode #648)
**Умова:** Замінити слова у реченні їхніми найкоротшими коренями зі словника.

```java
public String replaceWords(List<String> dictionary, String sentence) {
    // Будуємо Trie з коренів
    TrieNode root = new TrieNode();
    for (String root_word : dictionary) {
        TrieNode curr = root;
        for (char c : root_word.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) curr.children[i] = new TrieNode();
            curr = curr.children[i];
            if (curr.isEnd) break; // вже є коротший корінь
        }
        curr.isEnd = true;
    }

    StringBuilder sb = new StringBuilder();
    for (String word : sentence.split(" ")) {
        if (sb.length() > 0) sb.append(" ");
        sb.append(findRoot(root, word));
    }

    return sb.toString();
}

private String findRoot(TrieNode root, String word) {
    TrieNode curr = root;
    for (int i = 0; i < word.length(); i++) {
        int idx = word.charAt(i) - 'a';
        if (curr.children[idx] == null) break;
        curr = curr.children[idx];
        if (curr.isEnd) return word.substring(0, i + 1); // знайшли корінь
    }
    return word; // корінь не знайдено — повертаємо оригінал
}
```

---

## 📐 Патерн 5: Trie з HashMap (для не-ASCII символів)

### Коли використовувати
- Символи не тільки a-z (Unicode, цифри, спеціальні символи)
- Великий алфавіт де масив `[26]` неефективний

```java
class TrieNode {
    Map<Character, TrieNode> children = new HashMap<>();
    boolean isEnd = false;
    // або: String word = null;
}

class Trie {
    private TrieNode root = new TrieNode();

    public void insert(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            curr.children.putIfAbsent(c, new TrieNode());
            curr = curr.children.get(c);
        }
        curr.isEnd = true;
    }

    public boolean search(String word) {
        TrieNode curr = root;
        for (char c : word.toCharArray()) {
            if (!curr.children.containsKey(c)) return false;
            curr = curr.children.get(c);
        }
        return curr.isEnd;
    }
}
```

---

## 📐 Патерн 6: Autocomplete — отримати всі слова з prefix

### Задача 5: Search Suggestions System (LeetCode #1268)
**Умова:** Для кожного prefix введеного рядка повернути 3 найменші лексикографічно відповідні продукти.

```java
public List<List<String>> suggestedProducts(String[] products, String searchWord) {
    Arrays.sort(products); // лексикографічне сортування
    TrieNode root = buildTrie(products);

    List<List<String>> result = new ArrayList<>();
    TrieNode curr = root;

    for (char c : searchWord.toCharArray()) {
        List<String> suggestions = new ArrayList<>();

        if (curr != null) {
            curr = curr.children[c - 'a'];
            if (curr != null) collectWords(curr, suggestions);
        }

        result.add(suggestions);
    }

    return result;
}

private TrieNode buildTrie(String[] products) {
    TrieNode root = new TrieNode();
    for (String product : products) {
        TrieNode curr = root;
        for (char c : product.toCharArray()) {
            int i = c - 'a';
            if (curr.children[i] == null) curr.children[i] = new TrieNode();
            curr = curr.children[i];
        }
        curr.word = product;
    }
    return root;
}

// DFS для збору до 3 слів
private void collectWords(TrieNode node, List<String> result) {
    if (result.size() == 3) return;
    if (node.word != null) result.add(node.word);

    for (TrieNode child : node.children) {
        if (child != null) collectWords(child, result);
        if (result.size() == 3) return;
    }
}

class TrieNode {
    TrieNode[] children = new TrieNode[26];
    String word = null;
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Trie
│
├── "insert + search + startsWith"?
│   └── Базовий Trie (TrieNode з children[26] + isEnd)
│
├── "search з wildcards" (. або *)
│   └── Trie + DFS рекурсія при '.'
│
├── "знайти слова у grid зі словника"?
│   └── Trie + DFS по grid одночасно
│       (ефективніше ніж окремий DFS для кожного слова)
│
├── "замінити на найкоротший корінь"?
│   └── Trie → знайти перший isEnd при traversal
│
├── "autocomplete / suggestions"?
│   └── Trie → traverse до prefix → DFS для збору слів
│
└── Символи не a-z?
    └── TrieNode з HashMap замість children[26]
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `search` повертає `true` якщо traversal успішний | Перевіряти `node.isEnd` — prefix ≠ слово |
| `startsWith` перевіряє `isEnd` | `startsWith` повертає `true` якщо вузол існує, незалежно від `isEnd` |
| Не відновлювати `board[r][c]` після DFS у Word Search | `board[r][c] = ch` після рекурсивних викликів (backtrack) |
| Дублікати у Word Search II | `node.word = null` після знаходження слова |
| Зберігати слово у кожному вузлі замість кінцевого | Зберігай `word` тільки у вузлі де `isEnd = true` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #208 Implement Trie (Prefix Tree)
- [ ] #211 Design Add and Search Words Data Structure
- [ ] #648 Replace Words

### Should Solve (Middle)
- [ ] #212 Word Search II
- [ ] #1268 Search Suggestions System
- [ ] #677 Map Sum Pairs

### Stretch Goals
- [ ] #336 Palindrome Pairs
- [ ] #745 Prefix and Suffix Search
- [ ] #1032 Stream of Characters
