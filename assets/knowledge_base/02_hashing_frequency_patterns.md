# 02. Hashing & Frequency Patterns

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 2–3 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "find if exists", "contains duplicate" | HashSet |
| "count frequency", "most frequent" | HashMap (frequency counter) |
| "two numbers sum to target" | HashMap (Two Sum) |
| "group by", "anagram", "same characters" | HashMap з ключем-сигнатурою |
| "find duplicate / missing number" | HashSet або Math/XOR |
| "longest consecutive sequence" | HashSet |
| "intersection / union of arrays" | HashSet |

---

## 📐 Патерн 1: HashSet — перевірка існування / дублікатів

### Коли використовувати
- Потрібно відповісти "чи існує X?"
- Знайти дублікат за O(n)

### Шаблон

```java
Set<Integer> seen = new HashSet<>();

for (int num : nums) {
    if (seen.contains(num)) {
        // знайшли дублікат або потрібний елемент
        return true;
    }
    seen.add(num);
}
return false;
```

### Задача 1: Contains Duplicate (LeetCode #217)

```java
public boolean containsDuplicate(int[] nums) {
    Set<Integer> seen = new HashSet<>();
    for (int num : nums) {
        if (!seen.add(num)) return true; // add() повертає false якщо вже є
    }
    return false;
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 2: Longest Consecutive Sequence (LeetCode #128)
**Умова:** Знайти довжину найдовшої послідовності consecutive чисел. O(n).

**Ключова думка:** Додаємо всі числа в Set. Починаємо рахувати послідовність тільки від **початку** (num-1 не в Set) — це уникає повторної роботи.

```java
public int longestConsecutive(int[] nums) {
    Set<Integer> numSet = new HashSet<>();
    for (int num : nums) numSet.add(num);

    int longest = 0;

    for (int num : numSet) {
        // починаємо тільки від початку послідовності
        if (!numSet.contains(num - 1)) {
            int current = num;
            int length = 1;

            while (numSet.contains(current + 1)) {
                current++;
                length++;
            }

            longest = Math.max(longest, length);
        }
    }

    return longest;
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 3: Intersection of Two Arrays (LeetCode #349)

```java
public int[] intersection(int[] nums1, int[] nums2) {
    Set<Integer> set1 = new HashSet<>();
    for (int n : nums1) set1.add(n);

    Set<Integer> result = new HashSet<>();
    for (int n : nums2) {
        if (set1.contains(n)) result.add(n);
    }

    return result.stream().mapToInt(Integer::intValue).toArray();
}
```

---

## 📐 Патерн 2: HashMap — Frequency Counter

### Коли використовувати
- Підрахунок кількості входжень
- Порівняння частот двох структур
- "Most frequent", "top K frequent"

### Шаблон

```java
Map<T, Integer> freq = new HashMap<>();
for (T item : items) {
    freq.merge(item, 1, Integer::sum); // або:
    // freq.put(item, freq.getOrDefault(item, 0) + 1);
}
```

### Задача 4: Valid Anagram (LeetCode #242)
**Умова:** Чи є два рядки анаграмами один одного?

```java
public boolean isAnagram(String s, String t) {
    if (s.length() != t.length()) return false;

    int[] count = new int[26]; // тільки для lowercase англійських літер

    for (char c : s.toCharArray()) count[c - 'a']++;
    for (char c : t.toCharArray()) count[c - 'a']--;

    for (int c : count) {
        if (c != 0) return false;
    }
    return true;
}
```
> **Примітка:** Якщо символи Unicode — використовуй `HashMap<Character, Integer>` замість масиву.

**Складність:** O(n) time, O(1) space (масив фіксованого розміру 26)

---

### Задача 5: Top K Frequent Elements (LeetCode #347)
**Умова:** Знайти k найчастіших елементів.

**Підхід 1 — Min Heap (O(n log k)):**

```java
public int[] topKFrequent(int[] nums, int k) {
    Map<Integer, Integer> freq = new HashMap<>();
    for (int num : nums) freq.merge(num, 1, Integer::sum);

    // Min-heap: зберігаємо тільки k найбільших
    PriorityQueue<Integer> minHeap = new PriorityQueue<>(
        (a, b) -> freq.get(a) - freq.get(b)
    );

    for (int num : freq.keySet()) {
        minHeap.offer(num);
        if (minHeap.size() > k) minHeap.poll(); // видаляємо найменший
    }

    int[] result = new int[k];
    for (int i = k - 1; i >= 0; i--) result[i] = minHeap.poll();
    return result;
}
```

**Підхід 2 — Bucket Sort (O(n)), краще для інтерв'ю:**

```java
public int[] topKFrequent(int[] nums, int k) {
    Map<Integer, Integer> freq = new HashMap<>();
    for (int num : nums) freq.merge(num, 1, Integer::sum);

    // bucket[i] = список чисел з частотою i
    List<Integer>[] bucket = new List[nums.length + 1];
    for (int num : freq.keySet()) {
        int f = freq.get(num);
        if (bucket[f] == null) bucket[f] = new ArrayList<>();
        bucket[f].add(num);
    }

    // збираємо з кінця (найбільша частота)
    List<Integer> result = new ArrayList<>();
    for (int i = bucket.length - 1; i >= 0 && result.size() < k; i--) {
        if (bucket[i] != null) result.addAll(bucket[i]);
    }

    return result.stream().mapToInt(Integer::intValue).toArray();
}
```
**Складність:** O(n) time, O(n) space

---

## 📐 Патерн 3: HashMap — Two Sum Family

### Коли використовувати
- Знайти пару/трійку з заданою сумою в **невідсортованому** масиві
- Відповідь потрібна за **індексами** (не значеннями)

### Шаблон — Two Sum

```java
Map<Integer, Integer> seen = new HashMap<>(); // значення → індекс

for (int i = 0; i < nums.length; i++) {
    int complement = target - nums[i];

    if (seen.containsKey(complement)) {
        return new int[]{seen.get(complement), i};
    }

    seen.put(nums[i], i);
}
```

### Задача 6: Two Sum (LeetCode #1)

```java
public int[] twoSum(int[] nums, int target) {
    Map<Integer, Integer> seen = new HashMap<>();

    for (int i = 0; i < nums.length; i++) {
        int complement = target - nums[i];
        if (seen.containsKey(complement)) {
            return new int[]{seen.get(complement), i};
        }
        seen.put(nums[i], i);
    }

    return new int[]{};
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 7: Two Sum — кількість пар (варіація)
**Умова:** Знайти кількість пар (i, j) де nums[i] + nums[j] == target.

```java
public int countPairs(int[] nums, int target) {
    Map<Integer, Integer> freq = new HashMap<>();
    int count = 0;

    for (int num : nums) {
        int complement = target - num;
        count += freq.getOrDefault(complement, 0);
        freq.merge(num, 1, Integer::sum);
    }

    return count;
}
```

---

### Задача 8: Subarray Sum Equals K — нагадування зв'язку з Prefix Sum
(Детально розібрано в розділі 01, але ключова ідея — це Two Sum на prefix sums)

```
prefix[j] - prefix[i] == k
  ↓
prefix[i] == prefix[j] - k
  ↓
HashMap: "чи бачили prefix[j] - k раніше?"
```

---

## 📐 Патерн 4: HashMap — групування за ключем-сигнатурою

### Коли використовувати
- Потрібно згрупувати елементи за якоюсь спільною властивістю
- "Group anagrams", "group by pattern"

### Шаблон

```java
Map<String, List<String>> groups = new HashMap<>();

for (String word : words) {
    String key = computeKey(word); // сигнатура групи
    groups.computeIfAbsent(key, k -> new ArrayList<>()).add(word);
}
```

### Задача 9: Group Anagrams (LeetCode #49)
**Умова:** Згрупувати рядки що є анаграмами.

**Ключ = відсортований рядок** (анаграми мають однаковий відсортований вигляд)

```java
public List<List<String>> groupAnagrams(String[] strs) {
    Map<String, List<String>> groups = new HashMap<>();

    for (String s : strs) {
        char[] chars = s.toCharArray();
        Arrays.sort(chars);
        String key = new String(chars); // "eat" → "aet", "tea" → "aet"

        groups.computeIfAbsent(key, k -> new ArrayList<>()).add(s);
    }

    return new ArrayList<>(groups.values());
}
```
**Складність:** O(n * m log m) де m — середня довжина рядка

**Альтернативний ключ — масив частот** (O(n*m), краще асимптотично):

```java
private String getFreqKey(String s) {
    int[] count = new int[26];
    for (char c : s.toCharArray()) count[c - 'a']++;
    return Arrays.toString(count); // "[1,0,0,...,1,0,...]"
}
```

---

### Задача 10: Word Pattern (LeetCode #290)
**Умова:** Чи відповідає рядок "aabb" шаблону "dog dog cat cat"?

**Ключова думка:** Bijection — двостороннє відображення (pattern→word і word→pattern).

```java
public boolean wordPattern(String pattern, String s) {
    String[] words = s.split(" ");
    if (pattern.length() != words.length) return false;

    Map<Character, String> charToWord = new HashMap<>();
    Map<String, Character> wordToChar = new HashMap<>();

    for (int i = 0; i < pattern.length(); i++) {
        char c = pattern.charAt(i);
        String w = words[i];

        if (charToWord.containsKey(c) && !charToWord.get(c).equals(w)) return false;
        if (wordToChar.containsKey(w) && wordToChar.get(w) != c) return false;

        charToWord.put(c, w);
        wordToChar.put(w, c);
    }

    return true;
}
```

---

## 📐 Патерн 5: Знайти відсутній / дублікатний елемент

### Варіант A — HashSet (простий, O(n) space)

```java
public int findDuplicate(int[] nums) {
    Set<Integer> seen = new HashSet<>();
    for (int num : nums) {
        if (!seen.add(num)) return num;
    }
    return -1;
}
```

### Варіант B — Math (O(1) space, тільки якщо числа 1..n)

```java
// Знайти відсутнє число в масиві [0..n]
public int missingNumber(int[] nums) {
    int n = nums.length;
    int expected = n * (n + 1) / 2; // сума 0+1+2+...+n
    int actual = 0;
    for (int num : nums) actual += num;
    return expected - actual;
}
```

### Варіант C — XOR (O(1) space, елегантний)

```java
// XOR всіх чисел 1..n з усіма елементами масиву
// дублікати скасовуються: a ^ a = 0
public int missingNumber(int[] nums) {
    int xor = nums.length; // починаємо з n
    for (int i = 0; i < nums.length; i++) {
        xor ^= i ^ nums[i];
    }
    return xor;
}
```

### Задача 11: Find All Duplicates in Array (LeetCode #442)
**Умова:** Масив [1..n], деякі числа зустрічаються двічі. O(n) time, O(1) extra space.

**Ключова думка:** Використовуємо знак елемента як маркер "бачили чи ні".

```java
public List<Integer> findDuplicates(int[] nums) {
    List<Integer> result = new ArrayList<>();

    for (int num : nums) {
        int idx = Math.abs(num) - 1; // число як індекс
        if (nums[idx] < 0) {
            result.add(Math.abs(num)); // вже відвідували → дублікат
        } else {
            nums[idx] = -nums[idx];   // позначаємо як відвіданий
        }
    }

    return result;
}
```

---

## 📐 Патерн 6: Sliding Window + HashMap (розширений)

### Задача 12: Longest Substring with At Most K Distinct Characters (LeetCode #340)

```java
public int lengthOfLongestSubstringKDistinct(String s, int k) {
    Map<Character, Integer> freq = new HashMap<>();
    int left = 0, result = 0;

    for (int right = 0; right < s.length(); right++) {
        char c = s.charAt(right);
        freq.merge(c, 1, Integer::sum);

        // Shrink: якщо різних символів більше k
        while (freq.size() > k) {
            char leftChar = s.charAt(left);
            freq.merge(leftChar, -1, Integer::sum);
            if (freq.get(leftChar) == 0) freq.remove(leftChar);
            left++;
        }

        result = Math.max(result, right - left + 1);
    }

    return result;
}
```

---

### Задача 13: Minimum Window Substring (LeetCode #76) ⭐ Hard але важлива
**Умова:** Найменше вікно в s що містить всі символи t.

```java
public String minWindow(String s, String t) {
    if (s.length() < t.length()) return "";

    Map<Character, Integer> need = new HashMap<>();
    for (char c : t.toCharArray()) need.merge(c, 1, Integer::sum);

    int left = 0, formed = 0, required = need.size();
    int minLen = Integer.MAX_VALUE, minLeft = 0;
    Map<Character, Integer> window = new HashMap<>();

    for (int right = 0; right < s.length(); right++) {
        char c = s.charAt(right);
        window.merge(c, 1, Integer::sum);

        // перевіряємо чи цей символ "закрив" потребу
        if (need.containsKey(c) && window.get(c).equals(need.get(c))) {
            formed++;
        }

        // Shrink: поки вікно валідне
        while (formed == required) {
            if (right - left + 1 < minLen) {
                minLen = right - left + 1;
                minLeft = left;
            }

            char leftChar = s.charAt(left);
            window.merge(leftChar, -1, Integer::sum);
            if (need.containsKey(leftChar) && window.get(leftChar) < need.get(leftChar)) {
                formed--;
            }
            left++;
        }
    }

    return minLen == Integer.MAX_VALUE ? "" : s.substring(minLeft, minLeft + minLen);
}
```
**Складність:** O(|s| + |t|) time, O(|t|) space

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача з Hashing
│
├── "exists?" / "duplicate?" / "seen before?"
│   └── HashSet
│
├── "count" / "frequency" / "most common"
│   └── HashMap (Frequency Counter)
│
├── "two numbers sum to target" + індекси потрібні
│   └── HashMap (Two Sum pattern)
│
├── "group by" / "anagram" / "same structure"
│   └── HashMap з ключем-сигнатурою
│
├── "missing" / "duplicate" числа 1..n
│   ├── O(n) space допустимо → HashSet
│   ├── числа суммуються → Math (sum formula)
│   └── O(1) space → XOR trick
│
└── Sliding window + умова на uniq символи/елементи
    └── HashMap в sliding window
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `map.get(key) == value` для Integer | `map.get(key).equals(value)` — Integer unboxing trap! |
| Не перевіряти `containsKey` перед `get` | `getOrDefault(key, 0)` або `containsKey` спочатку |
| Використовувати `==` для порівняння String ключів | `.equals()` завжди для об'єктів |
| Забути видалити з map при shrink у sliding window | `if (freq.get(c) == 0) freq.remove(c)` |
| `Arrays.sort` на char[] і потім `new String(chars)` — забути | char[] треба сортувати окремо |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #1 Two Sum
- [ ] #217 Contains Duplicate
- [ ] #242 Valid Anagram
- [ ] #49 Group Anagrams
- [ ] #128 Longest Consecutive Sequence
- [ ] #268 Missing Number

### Should Solve (Middle)
- [ ] #347 Top K Frequent Elements
- [ ] #290 Word Pattern
- [ ] #442 Find All Duplicates in Array
- [ ] #560 Subarray Sum Equals K
- [ ] #340 Longest Substring with At Most K Distinct Characters
- [ ] #76 Minimum Window Substring

### Stretch Goals
- [ ] #41 First Missing Positive (O(1) space)
- [ ] #454 4Sum II
- [ ] #149 Max Points on a Line

---

## 🔑 Quick Reference: Java HashMap / HashSet

```java
// --- HashMap ---
Map<K, V> map = new HashMap<>();

map.put(key, value);
map.get(key);                          // null якщо не існує
map.getOrDefault(key, defaultValue);   // безпечно
map.containsKey(key);
map.remove(key);
map.merge(key, 1, Integer::sum);       // increment count
map.computeIfAbsent(key, k -> new ArrayList<>()).add(item); // group by

// Ітерація
for (Map.Entry<K, V> entry : map.entrySet()) {
    entry.getKey(); entry.getValue();
}
for (K key : map.keySet()) { ... }
for (V val : map.values()) { ... }

// --- HashSet ---
Set<T> set = new HashSet<>();

set.add(item);           // повертає false якщо вже є
set.contains(item);
set.remove(item);
set.size();

// Set операції
set1.retainAll(set2);    // intersection (змінює set1!)
set1.addAll(set2);       // union
set1.removeAll(set2);    // difference

// --- LinkedHashMap (зберігає порядок вставки) ---
Map<K, V> ordered = new LinkedHashMap<>();

// --- TreeMap (відсортований за ключем) ---
Map<K, V> sorted = new TreeMap<>();
sorted.firstKey();
sorted.lastKey();
sorted.headMap(toKey);   // ключі < toKey
sorted.tailMap(fromKey); // ключі >= fromKey
```
