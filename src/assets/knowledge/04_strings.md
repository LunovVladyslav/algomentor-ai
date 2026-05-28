# 04. Strings — Anagram, Palindrome, Matching

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 3–4 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "anagram", "same characters", "permutation of" | Frequency Counter / Sliding Window |
| "palindrome", "reads same backwards" | Two Pointers / Expand Around Center |
| "substring", "contains pattern" | Sliding Window / KMP |
| "reverse", "rotate string" | Two Pointers in-place |
| "longest common", "edit distance" | DP (розділ 13) |
| "valid parentheses", "balanced" | Stack (розділ 07) |
| "encode/decode", "serialize" | StringBuilder + delimiter |

---

## 📐 Патерн 1: Anagram — Frequency Counter

### Ключова думка
Два рядки є анаграмами ↔ у них однакова частота кожного символу.

### Шаблон — порівняння двох рядків

```java
// Варіант A: масив частот (тільки lowercase a-z)
int[] count = new int[26];
for (char c : s.toCharArray()) count[c - 'a']++;
for (char c : t.toCharArray()) count[c - 'a']--;
// якщо всі нулі → анаграми

// Варіант B: HashMap (будь-які символи / Unicode)
Map<Character, Integer> freq = new HashMap<>();
for (char c : s.toCharArray()) freq.merge(c, 1, Integer::sum);
for (char c : t.toCharArray()) {
    freq.merge(c, -1, Integer::sum);
    if (freq.get(c) == 0) freq.remove(c);
}
// якщо map порожня → анаграми
```

### Задача 1: Valid Anagram (LeetCode #242)

```java
public boolean isAnagram(String s, String t) {
    if (s.length() != t.length()) return false;

    int[] count = new int[26];
    for (char c : s.toCharArray()) count[c - 'a']++;
    for (char c : t.toCharArray()) count[c - 'a']--;

    for (int c : count) if (c != 0) return false;
    return true;
}
```
**Складність:** O(n) time, O(1) space

---

### Задача 2: Permutation in String (LeetCode #567)
**Умова:** Чи містить s2 permutation рядка s1 як підрядок?

**Ключова думка:** Sliding Window розміру `s1.length()` по s2. Порівнюємо frequency arrays.

```java
public boolean checkInclusion(String s1, String s2) {
    if (s1.length() > s2.length()) return false;

    int[] need = new int[26];
    int[] window = new int[26];

    for (char c : s1.toCharArray()) need[c - 'a']++;

    int k = s1.length();

    // будуємо перше вікно
    for (int i = 0; i < k; i++) window[s2.charAt(i) - 'a']++;

    if (Arrays.equals(need, window)) return true;

    // слайдимо вікно
    for (int i = k; i < s2.length(); i++) {
        window[s2.charAt(i) - 'a']++;         // додаємо правий
        window[s2.charAt(i - k) - 'a']--;     // видаляємо лівий
        if (Arrays.equals(need, window)) return true;
    }

    return false;
}
```

> ⚠️ `Arrays.equals` на масиві розміру 26 — O(26) = O(1). Це ок.

**Оптимізація через лічильник `matches`** — уникаємо `Arrays.equals` кожної ітерації:

```java
public boolean checkInclusion(String s1, String s2) {
    if (s1.length() > s2.length()) return false;

    int[] count = new int[26];
    for (char c : s1.toCharArray()) count[c - 'a']++;

    // matches = кількість символів з нульовим балансом
    int matches = 0;
    for (int c : count) if (c == 0) matches++;

    int left = 0;
    for (int right = 0; right < s2.length(); right++) {
        // додаємо правий символ
        int r = s2.charAt(right) - 'a';
        count[r]--;
        if (count[r] == 0) matches++;
        else if (count[r] == -1) matches--; // баланс порушився

        // видаляємо лівий символ (після заповнення вікна)
        if (right >= s1.length()) {
            int l = s2.charAt(left++) - 'a';
            count[l]++;
            if (count[l] == 0) matches++;
            else if (count[l] == 1) matches--;
        }

        if (matches == 26) return true;
    }

    return false;
}
```
**Складність:** O(n) time, O(1) space

---

## 📐 Патерн 2: Palindrome

### Варіант A: Two Pointers (перевірка)

```java
// Перевірити чи є рядок паліндромом
boolean isPalindrome(String s) {
    int left = 0, right = s.length() - 1;
    while (left < right) {
        if (s.charAt(left) != s.charAt(right)) return false;
        left++;
        right--;
    }
    return true;
}
```

### Задача 3: Valid Palindrome (LeetCode #125)
**Умова:** Рядок з символами різного типу — перевірити тільки alphanumeric, ignore case.

```java
public boolean isPalindrome(String s) {
    int left = 0, right = s.length() - 1;

    while (left < right) {
        // пропускаємо не-alphanumeric
        while (left < right && !Character.isLetterOrDigit(s.charAt(left))) left++;
        while (left < right && !Character.isLetterOrDigit(s.charAt(right))) right--;

        if (Character.toLowerCase(s.charAt(left)) !=
            Character.toLowerCase(s.charAt(right))) return false;

        left++;
        right--;
    }

    return true;
}
```

---

### Варіант B: Expand Around Center

**Ключова думка:** Кожен паліндром має центр. Розширюємо з центру поки символи рівні.  
Центр може бути: один символ (непарна довжина) або між двома (парна довжина).

```java
// Розширення з центру — повертає довжину паліндрому
int expandAroundCenter(String s, int left, int right) {
    while (left >= 0 && right < s.length()
           && s.charAt(left) == s.charAt(right)) {
        left--;
        right++;
    }
    return right - left - 1; // довжина паліндрому
}

// Виклик для кожного центру:
for (int i = 0; i < s.length(); i++) {
    int odd  = expandAroundCenter(s, i, i);     // непарна довжина
    int even = expandAroundCenter(s, i, i + 1); // парна довжина
    // ...
}
```

### Задача 4: Longest Palindromic Substring (LeetCode #5)

```java
public String longestPalindrome(String s) {
    int start = 0, maxLen = 1;

    for (int i = 0; i < s.length(); i++) {
        // непарна довжина: центр = i
        int len1 = expand(s, i, i);
        // парна довжина: центр між i та i+1
        int len2 = expand(s, i, i + 1);

        int len = Math.max(len1, len2);

        if (len > maxLen) {
            maxLen = len;
            start = i - (len - 1) / 2; // обчислюємо початок
        }
    }

    return s.substring(start, start + maxLen);
}

private int expand(String s, int left, int right) {
    while (left >= 0 && right < s.length()
           && s.charAt(left) == s.charAt(right)) {
        left--;
        right++;
    }
    return right - left - 1;
}
```
**Складність:** O(n²) time, O(1) space

---

### Задача 5: Palindromic Substrings — кількість (LeetCode #647)

```java
public int countSubstrings(String s) {
    int count = 0;

    for (int i = 0; i < s.length(); i++) {
        count += countExpand(s, i, i);     // непарні
        count += countExpand(s, i, i + 1); // парні
    }

    return count;
}

private int countExpand(String s, int left, int right) {
    int count = 0;
    while (left >= 0 && right < s.length()
           && s.charAt(left) == s.charAt(right)) {
        count++;
        left--;
        right++;
    }
    return count;
}
```

---

### Задача 6: Valid Palindrome II (LeetCode #680)
**Умова:** Чи стане рядок паліндромом після видалення **одного** символу?

**Ключова думка:** Two Pointers до першої розбіжності → спробувати видалити лівий або правий символ.

```java
public boolean validPalindrome(String s) {
    int left = 0, right = s.length() - 1;

    while (left < right) {
        if (s.charAt(left) != s.charAt(right)) {
            // спробуємо пропустити лівий або правий
            return isPalin(s, left + 1, right) || isPalin(s, left, right - 1);
        }
        left++;
        right--;
    }

    return true;
}

private boolean isPalin(String s, int left, int right) {
    while (left < right) {
        if (s.charAt(left) != s.charAt(right)) return false;
        left++;
        right--;
    }
    return true;
}
```

---

## 📐 Патерн 3: String Matching — Sliding Window

### Задача 7: Find All Anagrams in a String (LeetCode #438)
**Умова:** Знайти всі початкові індекси anagram рядка p у рядку s.

```java
public List<Integer> findAnagrams(String s, String p) {
    List<Integer> result = new ArrayList<>();
    if (s.length() < p.length()) return result;

    int[] need = new int[26];
    int[] window = new int[26];

    for (char c : p.toCharArray()) need[c - 'a']++;
    int k = p.length();

    for (int i = 0; i < k; i++) window[s.charAt(i) - 'a']++;
    if (Arrays.equals(need, window)) result.add(0);

    for (int i = k; i < s.length(); i++) {
        window[s.charAt(i) - 'a']++;
        window[s.charAt(i - k) - 'a']--;
        if (Arrays.equals(need, window)) result.add(i - k + 1);
    }

    return result;
}
```

---

## 📐 Патерн 4: String Matching — KMP Algorithm

### Коли використовувати
- Знайти всі входження pattern у text за **O(n + m)**
- Brute force — O(n * m), KMP — O(n + m)

### Крок 1: Побудова LPS (Longest Proper Prefix which is also Suffix)

```
pattern = "ABABC"
lps    = [0, 0, 1, 2, 0]

"A"     → 0 (немає префіксу = суфіксу)
"AB"    → 0
"ABA"   → 1 ("A" є і префіксом і суфіксом)
"ABAB"  → 2 ("AB" є і префіксом і суфіксом)
"ABABC" → 0
```

```java
private int[] buildLPS(String pattern) {
    int m = pattern.length();
    int[] lps = new int[m];
    int len = 0; // довжина попереднього найдовшого prefix-suffix
    int i = 1;

    while (i < m) {
        if (pattern.charAt(i) == pattern.charAt(len)) {
            lps[i++] = ++len;
        } else {
            if (len != 0) {
                len = lps[len - 1]; // відкат
            } else {
                lps[i++] = 0;
            }
        }
    }

    return lps;
}
```

### Крок 2: KMP Search

```java
public List<Integer> kmpSearch(String text, String pattern) {
    List<Integer> result = new ArrayList<>();
    int n = text.length(), m = pattern.length();
    int[] lps = buildLPS(pattern);

    int i = 0; // індекс у text
    int j = 0; // індекс у pattern

    while (i < n) {
        if (text.charAt(i) == pattern.charAt(j)) {
            i++;
            j++;
        }

        if (j == m) {
            result.add(i - j); // знайшли входження
            j = lps[j - 1];    // шукаємо наступне
        } else if (i < n && text.charAt(i) != pattern.charAt(j)) {
            if (j != 0) j = lps[j - 1]; // відкат по lps
            else i++;
        }
    }

    return result;
}
```

### Задача 8: Repeated Substring Pattern (LeetCode #459)
**Умова:** Чи складається рядок з повторень підрядка?

**KMP трюк:** Якщо s складається з повторень → `(s + s).indexOf(s, 1) != s.length()`

```java
public boolean repeatedSubstringPattern(String s) {
    String doubled = s + s;
    // шукаємо s у doubled, але не з початку і не з кінця
    return doubled.indexOf(s, 1) != s.length();
}
```

---

## 📐 Патерн 5: Reverse / Rotate — Two Pointers in-place

### Шаблон — reverse підрядка

```java
void reverse(char[] arr, int left, int right) {
    while (left < right) {
        char temp = arr[left];
        arr[left++] = arr[right];
        arr[right--] = temp;
    }
}
```

### Задача 9: Reverse Words in a String (LeetCode #151)
**Умова:** Розвернути порядок слів у рядку (видалити зайві пробіли).

```java
public String reverseWords(String s) {
    // Варіант зі split (простий, але O(n) extra space)
    String[] words = s.trim().split("\\s+");
    StringBuilder sb = new StringBuilder();

    for (int i = words.length - 1; i >= 0; i--) {
        sb.append(words[i]);
        if (i > 0) sb.append(" ");
    }

    return sb.toString();
}
```

**In-place варіант (на char[]):**
1. Reverse весь рядок
2. Reverse кожне слово окремо
3. Видалити зайві пробіли

```java
public String reverseWords(String s) {
    char[] arr = s.toCharArray();
    int n = arr.length;

    reverse(arr, 0, n - 1);          // 1. Reverse all
    reverseWords(arr, n);             // 2. Reverse each word
    return cleanSpaces(arr, n);       // 3. Clean spaces
}

private void reverseWords(char[] arr, int n) {
    int i = 0, j = 0;
    while (i < n) {
        while (i < j || (i < n && arr[i] == ' ')) i++; // skip spaces
        while (j < i || (j < n && arr[j] != ' ')) j++; // find end of word
        reverse(arr, i, j - 1);
    }
}

private String cleanSpaces(char[] arr, int n) {
    int i = 0, j = 0;
    while (j < n) {
        while (j < n && arr[j] == ' ') j++;             // skip spaces
        while (j < n && arr[j] != ' ') arr[i++] = arr[j++]; // copy word
        while (j < n && arr[j] == ' ') j++;             // skip spaces
        if (j < n) arr[i++] = ' ';
    }
    return new String(arr, 0, i);
}

private void reverse(char[] arr, int left, int right) {
    while (left < right) {
        char tmp = arr[left];
        arr[left++] = arr[right];
        arr[right--] = tmp;
    }
}
```

---

### Задача 10: Rotate String (LeetCode #796)
**Умова:** Чи можна отримати goal з s шляхом rotation?

```java
public boolean rotateString(String s, String goal) {
    if (s.length() != goal.length()) return false;
    return (s + s).contains(goal); // усі rotation містяться у s+s
}
```

---

## 📐 Патерн 6: StringBuilder — encode / decode / побудова

### Задача 11: Encode and Decode Strings (LeetCode #271)
**Умова:** Закодувати список рядків в один і декодувати назад.

**Ключова думка:** Зберігаємо довжину рядка перед кожним словом як заголовок.

```java
// Encode: "4#word3#foo" → ["word", "foo"]
public String encode(List<String> strs) {
    StringBuilder sb = new StringBuilder();
    for (String s : strs) {
        sb.append(s.length()).append('#').append(s);
    }
    return sb.toString();
}

public List<String> decode(String s) {
    List<String> result = new ArrayList<>();
    int i = 0;

    while (i < s.length()) {
        int j = s.indexOf('#', i);           // знаходимо роздільник
        int len = Integer.parseInt(s.substring(i, j));
        result.add(s.substring(j + 1, j + 1 + len));
        i = j + 1 + len;
    }

    return result;
}
```

---

### Задача 12: Longest Common Prefix (LeetCode #14)

```java
public String longestCommonPrefix(String[] strs) {
    if (strs.length == 0) return "";

    String prefix = strs[0];

    for (int i = 1; i < strs.length; i++) {
        // скорочуємо prefix поки не збігається з початком strs[i]
        while (!strs[i].startsWith(prefix)) {
            prefix = prefix.substring(0, prefix.length() - 1);
            if (prefix.isEmpty()) return "";
        }
    }

    return prefix;
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на рядки
│
├── "anagram" / "permutation" / "same characters"?
│   ├── Порівняти два рядки → Frequency array (int[26])
│   └── Знайти у рядку → Sliding Window + frequency array
│
├── "palindrome"?
│   ├── Перевірити → Two Pointers (left/right)
│   ├── Найдовший паліндромний підрядок → Expand Around Center
│   └── Можна видалити 1 символ → Two Pointers + рекурсія
│
├── "find pattern in text"?
│   ├── Один пошук, простота важливіша → contains() / indexOf()
│   └── Всі входження / великий текст → KMP O(n+m)
│
├── "reverse" / "rotate"?
│   └── Two Pointers на char[] (in-place)
│
└── "encode" / "serialize" / "побудувати рядок"?
    └── StringBuilder + delimiter або length-prefix
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `s1 == s2` порівняння рядків | `s1.equals(s2)` |
| `s + char` у циклі → O(n²) | `StringBuilder.append(char)` |
| `s.substring(i, j)` — j включно? | j **не включно** (exclusive) |
| `split(" ")` не обробляє кілька пробілів | `split("\\s+")` |
| char арифметика: `c - 'a'` для не-lowercase | Перевірити `Character.isLowerCase(c)` |
| `indexOf` повертає -1 якщо не знайдено | Завжди перевіряти `!= -1` |
| Мутувати String напряму | String immutable — використовуй `char[]` або `StringBuilder` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #242 Valid Anagram
- [ ] #125 Valid Palindrome
- [ ] #5 Longest Palindromic Substring
- [ ] #567 Permutation in String
- [ ] #438 Find All Anagrams in a String
- [ ] #151 Reverse Words in a String
- [ ] #14 Longest Common Prefix

### Should Solve (Middle)
- [ ] #3 Longest Substring Without Repeating Characters (розділ 01)
- [ ] #647 Palindromic Substrings
- [ ] #680 Valid Palindrome II
- [ ] #459 Repeated Substring Pattern
- [ ] #271 Encode and Decode Strings
- [ ] #796 Rotate String

### Stretch Goals
- [ ] #76 Minimum Window Substring (розділ 02)
- [ ] #28 Find the Index of the First Occurrence (KMP)
- [ ] #336 Palindrome Pairs

---

## 🔑 Quick Reference: Java String операції

```java
// Основні операції
s.length()
s.charAt(i)
s.substring(start, end)        // [start, end) — end не включно
s.indexOf(ch)                  // -1 якщо не знайдено
s.indexOf(str, fromIndex)
s.contains(str)                // true/false
s.startsWith(prefix)
s.endsWith(suffix)
s.equals(other)                // порівняння (не ==)
s.equalsIgnoreCase(other)
s.toLowerCase() / toUpperCase()
s.trim()                       // видалити пробіли з країв
s.strip()                      // як trim() але Unicode-aware (Java 11+)
s.split("\\s+")                // split по whitespace
s.replace('a', 'b')            // char → char
s.replace("old", "new")        // String → String
s.toCharArray()                // String → char[]
String.valueOf(charArray)      // char[] → String

// Character утиліти
Character.isLetterOrDigit(c)
Character.isLetter(c)
Character.isDigit(c)
Character.isUpperCase(c)
Character.isLowerCase(c)
Character.toLowerCase(c)
Character.toUpperCase(c)
(int)(c - 'a')                 // char → index 0-25

// StringBuilder (мутабельний рядок)
StringBuilder sb = new StringBuilder();
sb.append(str / char / int);
sb.insert(index, str);
sb.delete(start, end);
sb.reverse();
sb.charAt(i);
sb.setCharAt(i, c);
sb.toString();
sb.length();

// String.format (для складного форматування)
String.format("%d#%s", len, word);

// Java 11+ корисні методи
s.isBlank()                    // true якщо тільки whitespace
s.repeat(n)                    // "ab".repeat(3) → "ababab"
s.strip() / stripLeading() / stripTrailing()
```
