# 10. Heap / Priority Queue

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 2–3 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "kth largest / smallest" | Min-Heap розміру k |
| "top k frequent / closest" | Heap з Comparator |
| "median of a stream" | Two Heaps (max + min) |
| "merge k sorted lists/arrays" | Min-Heap (K-way Merge) |
| "task scheduling with cooldown" | Max-Heap + cooldown queue |
| "find running median" | Two Heaps |
| "minimum cost to connect / combine" | Min-Heap (Greedy) |

---

## 📐 Heap у Java — основи

```java
// Min-Heap (дефолт — мінімум на вершині)
PriorityQueue<Integer> minHeap = new PriorityQueue<>();

// Max-Heap
PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());
// або:
PriorityQueue<Integer> maxHeap = new PriorityQueue<>((a, b) -> b - a);

// Heap з масивами [value, index]
PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]); // min by value

// Основні операції: O(log n)
pq.offer(x);    // додати
pq.poll();      // видалити і повернути вершину
pq.peek();      // подивитись вершину без видалення
pq.size();
pq.isEmpty();
```

---

## 📐 Патерн 1: Top-K — Min-Heap розміру k

### Ключова думка
Підтримуємо Min-Heap розміру **k**:
- Якщо новий елемент **більший** за вершину → витісняємо мінімум, додаємо новий
- В результаті heap містить **k найбільших** елементів, вершина = k-й найбільший

```java
PriorityQueue<Integer> minHeap = new PriorityQueue<>();

for (int num : nums) {
    minHeap.offer(num);
    if (minHeap.size() > k) minHeap.poll(); // видаляємо найменший
}

return minHeap.peek(); // k-й найбільший
```

### Задача 1: Kth Largest Element in a Stream (LeetCode #703)

```java
class KthLargest {
    private final PriorityQueue<Integer> minHeap;
    private final int k;

    public KthLargest(int k, int[] nums) {
        this.k = k;
        this.minHeap = new PriorityQueue<>();
        for (int num : nums) add(num);
    }

    public int add(int val) {
        minHeap.offer(val);
        if (minHeap.size() > k) minHeap.poll();
        return minHeap.peek();
    }
}
```
**Складність:** O(log k) для кожного `add`

---

### Задача 2: Kth Largest Element in an Array (LeetCode #215)

```java
public int findKthLargest(int[] nums, int k) {
    PriorityQueue<Integer> minHeap = new PriorityQueue<>();

    for (int num : nums) {
        minHeap.offer(num);
        if (minHeap.size() > k) minHeap.poll();
    }

    return minHeap.peek();
}
```
**Складність:** O(n log k) time, O(k) space

> 💡 **QuickSelect** дає O(n) average, але складніший у реалізації.  
> На інтерв'ю — heap варіант простіший і достатній.

---

### Задача 3: Top K Frequent Elements (LeetCode #347)

```java
public int[] topKFrequent(int[] nums, int k) {
    Map<Integer, Integer> freq = new HashMap<>();
    for (int num : nums) freq.merge(num, 1, Integer::sum);

    // Min-Heap за частотою — зберігаємо k найчастіших
    PriorityQueue<Integer> minHeap = new PriorityQueue<>(
        (a, b) -> freq.get(a) - freq.get(b)
    );

    for (int num : freq.keySet()) {
        minHeap.offer(num);
        if (minHeap.size() > k) minHeap.poll();
    }

    int[] result = new int[k];
    for (int i = k - 1; i >= 0; i--) result[i] = minHeap.poll();
    return result;
}
```

---

### Задача 4: K Closest Points to Origin (LeetCode #973)

```java
public int[][] kClosest(int[][] points, int k) {
    // Max-Heap за відстанню — витісняємо найдальші
    PriorityQueue<int[]> maxHeap = new PriorityQueue<>(
        (a, b) -> (b[0]*b[0] + b[1]*b[1]) - (a[0]*a[0] + a[1]*a[1])
    );

    for (int[] point : points) {
        maxHeap.offer(point);
        if (maxHeap.size() > k) maxHeap.poll(); // видаляємо найдальший
    }

    return maxHeap.toArray(new int[0][]);
}
```

> 💡 Не потрібно рахувати `sqrt` — відстані монотонні відносно квадрату.

---

## 📐 Патерн 2: Two Heaps — медіана потоку

### Ключова думка
- **Max-Heap** (lower) — зберігає меншу половину. Вершина = максимум лівої половини
- **Min-Heap** (upper) — зберігає більшу половину. Вершина = мінімум правої половини
- Медіана = середнє двох вершин (парна кількість) або вершина більшого heap (непарна)

```
lower (Max-Heap) | upper (Min-Heap)
[1, 2, 3, 4]    | [5, 6, 7, 8]
      4 ← медіана між 4 і 5 = 4.5
```

### Задача 5: Find Median from Data Stream (LeetCode #295)

```java
class MedianFinder {
    private PriorityQueue<Integer> lower; // Max-Heap — менша половина
    private PriorityQueue<Integer> upper; // Min-Heap — більша половина

    public MedianFinder() {
        lower = new PriorityQueue<>(Collections.reverseOrder());
        upper = new PriorityQueue<>();
    }

    public void addNum(int num) {
        // Крок 1: додаємо в lower
        lower.offer(num);

        // Крок 2: балансуємо — максимум lower має бути <= мінімуму upper
        if (!upper.isEmpty() && lower.peek() > upper.peek()) {
            upper.offer(lower.poll());
        }

        // Крок 3: балансуємо розміри (різниця не більше 1)
        if (lower.size() > upper.size() + 1) {
            upper.offer(lower.poll());
        } else if (upper.size() > lower.size()) {
            lower.offer(upper.poll());
        }
    }

    public double findMedian() {
        if (lower.size() == upper.size()) {
            return (lower.peek() + upper.peek()) / 2.0;
        }
        return lower.peek(); // lower завжди більший або рівний
    }
}
```
**Складність:** O(log n) addNum, O(1) findMedian

---

## 📐 Патерн 3: K-way Merge

### Ключова думка
Маємо k відсортованих масивів/списків. Min-Heap завжди зберігає **мінімальний елемент** серед поточних голів кожного списку.

### Шаблон

```java
// heap зберігає [значення, індекс_масиву, індекс_в_масиві]
PriorityQueue<int[]> minHeap = new PriorityQueue<>((a, b) -> a[0] - b[0]);

// Ініціалізація — додаємо перший елемент кожного масиву
for (int i = 0; i < arrays.length; i++) {
    if (arrays[i].length > 0) {
        minHeap.offer(new int[]{arrays[i][0], i, 0});
    }
}

while (!minHeap.isEmpty()) {
    int[] curr = minHeap.poll();
    int val = curr[0], arrIdx = curr[1], elemIdx = curr[2];

    // обробляємо val

    // додаємо наступний елемент з того ж масиву
    if (elemIdx + 1 < arrays[arrIdx].length) {
        minHeap.offer(new int[]{arrays[arrIdx][elemIdx + 1], arrIdx, elemIdx + 1});
    }
}
```

### Задача 6: Merge K Sorted Arrays
**Умова:** Злити k відсортованих масивів в один відсортований.

```java
public int[] mergeKSortedArrays(int[][] arrays) {
    PriorityQueue<int[]> minHeap = new PriorityQueue<>((a, b) -> a[0] - b[0]);
    int totalSize = 0;

    for (int i = 0; i < arrays.length; i++) {
        totalSize += arrays[i].length;
        if (arrays[i].length > 0) {
            minHeap.offer(new int[]{arrays[i][0], i, 0});
        }
    }

    int[] result = new int[totalSize];
    int idx = 0;

    while (!minHeap.isEmpty()) {
        int[] curr = minHeap.poll();
        result[idx++] = curr[0];

        int arrIdx = curr[1], elemIdx = curr[2];
        if (elemIdx + 1 < arrays[arrIdx].length) {
            minHeap.offer(new int[]{arrays[arrIdx][elemIdx + 1], arrIdx, elemIdx + 1});
        }
    }

    return result;
}
```
**Складність:** O(n log k) де n — загальна кількість елементів

---

### Задача 7: Kth Smallest Element in a Sorted Matrix (LeetCode #378)
**Умова:** Matrix де кожен рядок і стовпець відсортовані. Знайти k-й найменший елемент.

```java
public int kthSmallest(int[][] matrix, int k) {
    int n = matrix.length;
    // Min-Heap: [значення, рядок, стовпець]
    PriorityQueue<int[]> minHeap = new PriorityQueue<>((a, b) -> a[0] - b[0]);

    // Додаємо перший елемент кожного рядка
    for (int r = 0; r < n; r++) {
        minHeap.offer(new int[]{matrix[r][0], r, 0});
    }

    int result = 0;
    for (int i = 0; i < k; i++) {
        int[] curr = minHeap.poll();
        result = curr[0];
        int row = curr[1], col = curr[2];

        if (col + 1 < n) {
            minHeap.offer(new int[]{matrix[row][col + 1], row, col + 1});
        }
    }

    return result;
}
```

---

## 📐 Патерн 4: Greedy + Heap

### Задача 8: Task Scheduler (LeetCode #621)
**Умова:** Задачі з cooldown n між однаковими типами. Мінімальний час.

```java
public int leastInterval(char[] tasks, int n) {
    int[] freq = new int[26];
    for (char task : tasks) freq[task - 'A']++;

    // Max-Heap за частотою
    PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());
    for (int f : freq) if (f > 0) maxHeap.offer(f);

    // Queue для задач на cooldown: [залишкова_частота, час_коли_знову_доступна]
    Queue<int[]> cooldown = new ArrayDeque<>();
    int time = 0;

    while (!maxHeap.isEmpty() || !cooldown.isEmpty()) {
        time++;

        if (!maxHeap.isEmpty()) {
            int remaining = maxHeap.poll() - 1;
            if (remaining > 0) cooldown.offer(new int[]{remaining, time + n});
        }

        // повертаємо задачі з cooldown що стали доступні
        if (!cooldown.isEmpty() && cooldown.peek()[1] == time) {
            maxHeap.offer(cooldown.poll()[0]);
        }
    }

    return time;
}
```

---

### Задача 9: Find the Most Competitive Subsequence (LeetCode #1673)
**Умова:** Знайти найменшу лексикографічно підпослідовність довжини k.

```java
public int[] mostCompetitive(int[] nums, int k) {
    Deque<Integer> stack = new ArrayDeque<>();
    int n = nums.length;

    for (int i = 0; i < n; i++) {
        // видаляємо більші елементи якщо можемо ще набрати k елементів
        while (!stack.isEmpty()
               && stack.peek() > nums[i]
               && stack.size() + (n - i) > k) {
            stack.pop();
        }
        if (stack.size() < k) stack.push(nums[i]);
    }

    int[] result = new int[k];
    for (int i = k - 1; i >= 0; i--) result[i] = stack.pop();
    return result;
}
```

---

### Задача 10: Reorganize String (LeetCode #767)
**Умова:** Переставити символи рядка щоб однакові не стояли поруч.

```java
public String reorganizeString(String s) {
    int[] freq = new int[26];
    for (char c : s.toCharArray()) freq[c - 'a']++;

    // Max-Heap за частотою
    PriorityQueue<int[]> maxHeap = new PriorityQueue<>((a, b) -> b[1] - a[1]);
    for (int i = 0; i < 26; i++) {
        if (freq[i] > 0) maxHeap.offer(new int[]{i, freq[i]});
    }

    StringBuilder sb = new StringBuilder();

    while (maxHeap.size() >= 2) {
        int[] first = maxHeap.poll();
        int[] second = maxHeap.poll();

        sb.append((char)('a' + first[0]));
        sb.append((char)('a' + second[0]));

        if (--first[1] > 0) maxHeap.offer(first);
        if (--second[1] > 0) maxHeap.offer(second);
    }

    if (!maxHeap.isEmpty()) {
        int[] last = maxHeap.poll();
        if (last[1] > 1) return ""; // неможливо reorganize
        sb.append((char)('a' + last[0]));
    }

    return sb.toString();
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Heap
│
├── "Kth largest / smallest"?
│   ├── Kth largest → Min-Heap розміру k
│   └── Kth smallest → Max-Heap розміру k
│
├── "Top K" елементів за якоюсь метрикою?
│   └── Heap розміру k з кастомним Comparator
│
├── "Running median" / "median of stream"?
│   └── TWO HEAPS (Max lower + Min upper)
│       → балансуємо розміри після кожного додавання
│
├── "Merge k sorted" структур?
│   └── K-WAY MERGE (Min-Heap з [val, listIdx, elemIdx])
│
└── "Scheduling" / "Greedy мінімальна вартість"?
    └── Max-Heap + Greedy (завжди обираємо найвигідніший)
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `(a, b) -> a - b` для великих чисел | `Integer.compare(a, b)` або перевірити на overflow |
| Max-Heap через `(a,b) -> b - a` з об'єктами | `Collections.reverseOrder()` або `(a,b) -> b.compareTo(a)` |
| Не оновлювати heap після зміни значення | Heap не підтримує decrease-key — видали і додай заново |
| Two Heaps: не балансувати після кожного `addNum` | Завжди перевіряй і переноси елементи після вставки |
| `pq.toArray()` повертає не відсортований масив | Heap не сортований — тільки вершина гарантована |
| K-way merge: забути додати наступний елемент після poll | `minHeap.offer(next element from same list)` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #215 Kth Largest Element in an Array
- [ ] #703 Kth Largest Element in a Stream
- [ ] #347 Top K Frequent Elements
- [ ] #295 Find Median from Data Stream
- [ ] #973 K Closest Points to Origin

### Should Solve (Middle)
- [ ] #378 Kth Smallest Element in Sorted Matrix
- [ ] #621 Task Scheduler
- [ ] #767 Reorganize String
- [ ] #23 Merge K Sorted Lists (розділ 06)
- [ ] #1673 Find the Most Competitive Subsequence

### Stretch Goals
- [ ] #480 Sliding Window Median
- [ ] #502 IPO
- [ ] #632 Smallest Range Covering Elements from K Lists

---

## 🔑 Quick Reference: PriorityQueue у Java

```java
// Min-Heap (дефолт)
PriorityQueue<Integer> minPQ = new PriorityQueue<>();

// Max-Heap
PriorityQueue<Integer> maxPQ = new PriorityQueue<>(Collections.reverseOrder());

// За першим елементом масиву
PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);

// За другим елементом, descending
PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> b[1] - a[1]);

// За довжиною рядка
PriorityQueue<String> pq = new PriorityQueue<>(Comparator.comparingInt(String::length));

// Ітерація (не в порядку!)
for (int x : pq) { } // порядок не гарантований

// Отримати всі елементи у відсортованому порядку
List<Integer> sorted = new ArrayList<>();
while (!pq.isEmpty()) sorted.add(pq.poll());

// Початковий розмір (опціонально, для оптимізації)
PriorityQueue<Integer> pq = new PriorityQueue<>(k);

// Two Heaps — шаблон балансування
void balance() {
    if (lower.size() > upper.size() + 1) upper.offer(lower.poll());
    else if (upper.size() > lower.size()) lower.offer(upper.poll());
}
```
