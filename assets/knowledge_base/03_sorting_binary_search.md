# 03. Sorting & Binary Search

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 3–4 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "sorted array", "find target" | Binary Search (класичний) |
| "rotated sorted array" | Binary Search з перевіркою половини |
| "find minimum/maximum that satisfies condition" | Binary Search on Answer |
| "first/last position", "leftmost/rightmost" | Binary Search (left/right bound) |
| "kth smallest/largest" | Binary Search on Answer або Heap |
| "matrix sorted row/col", "search in matrix" | Binary Search (2D) |
| задача виглядає як O(n) але є монотонна умова | Binary Search on Answer |

---

## 📐 Патерн 1: Класичний Binary Search

### Головне правило
> Binary Search працює коли простір пошуку **монотонний** — тобто можна чітко сказати "ліва половина не підходить" або "права половина не підходить".

### Шаблон — пошук точного значення

```java
int left = 0, right = arr.length - 1;

while (left <= right) {               // <= бо right включно
    int mid = left + (right - left) / 2; // уникаємо overflow (не (l+r)/2)

    if (arr[mid] == target) {
        return mid;
    } else if (arr[mid] < target) {
        left = mid + 1;               // target правіше
    } else {
        right = mid - 1;              // target лівіше
    }
}

return -1; // не знайдено
```

> ⚠️ **Завжди** використовуй `mid = left + (right - left) / 2`  
> `(left + right) / 2` може викликати **integer overflow** при великих значеннях!

### Задача 1: Binary Search (LeetCode #704)

```java
public int search(int[] nums, int target) {
    int left = 0, right = nums.length - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (nums[mid] == target) return mid;
        else if (nums[mid] < target) left = mid + 1;
        else right = mid - 1;
    }

    return -1;
}
```
**Складність:** O(log n) time, O(1) space

---

## 📐 Патерн 2: Binary Search — лівий та правий bound

### Коли використовувати
- "First position of target" → лівий bound
- "Last position of target" → правий bound
- Масив з дублікатами

### Шаблон — лівий bound (перша позиція)

```java
// Знайти першу позицію де arr[mid] >= target
int left = 0, right = arr.length; // right = length (не length-1!)

while (left < right) {            // строго < (не <=)
    int mid = left + (right - left) / 2;

    if (arr[mid] < target) {
        left = mid + 1;
    } else {
        right = mid;              // mid може бути відповіддю → не відкидаємо
    }
}

return left; // або right — вони рівні після циклу
```

### Шаблон — правий bound (остання позиція)

```java
// Знайти останню позицію де arr[mid] <= target
int left = 0, right = arr.length;

while (left < right) {
    int mid = left + (right - left) / 2;

    if (arr[mid] <= target) {
        left = mid + 1;
    } else {
        right = mid;
    }
}

return left - 1; // остання позиція де arr[mid] <= target
```

### Задача 2: Find First and Last Position (LeetCode #34)

```java
public int[] searchRange(int[] nums, int target) {
    return new int[]{leftBound(nums, target), rightBound(nums, target)};
}

private int leftBound(int[] nums, int target) {
    int left = 0, right = nums.length;
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] < target) left = mid + 1;
        else right = mid;
    }
    // перевіряємо чи знайшли
    if (left == nums.length || nums[left] != target) return -1;
    return left;
}

private int rightBound(int[] nums, int target) {
    int left = 0, right = nums.length;
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] <= target) left = mid + 1;
        else right = mid;
    }
    // left-1 = остання позиція target
    if (left == 0 || nums[left - 1] != target) return -1;
    return left - 1;
}
```
**Складність:** O(log n) time, O(1) space

---

## 📐 Патерн 3: Binary Search на Rotated Array

### Ключова думка
Після rotation **одна половина завжди відсортована**.  
Перевіряємо яка половина відсортована → визначаємо куди рухатись.

```
[4, 5, 6, 7, 0, 1, 2]
      ↑mid
Ліва половина [4,5,6,7] — відсортована (nums[left] <= nums[mid])
Права половина [7,0,1,2] — ні
```

### Шаблон

```java
int left = 0, right = nums.length - 1;

while (left <= right) {
    int mid = left + (right - left) / 2;

    if (nums[mid] == target) return mid;

    // визначаємо яка половина відсортована
    if (nums[left] <= nums[mid]) {
        // ліва половина відсортована
        if (nums[left] <= target && target < nums[mid]) {
            right = mid - 1; // target у лівій половині
        } else {
            left = mid + 1;  // target у правій половині
        }
    } else {
        // права половина відсортована
        if (nums[mid] < target && target <= nums[right]) {
            left = mid + 1;  // target у правій половині
        } else {
            right = mid - 1; // target у лівій половині
        }
    }
}

return -1;
```

### Задача 3: Search in Rotated Sorted Array (LeetCode #33)

```java
public int search(int[] nums, int target) {
    int left = 0, right = nums.length - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] == target) return mid;

        if (nums[left] <= nums[mid]) {
            // ліва половина відсортована
            if (nums[left] <= target && target < nums[mid]) right = mid - 1;
            else left = mid + 1;
        } else {
            // права половина відсортована
            if (nums[mid] < target && target <= nums[right]) left = mid + 1;
            else right = mid - 1;
        }
    }

    return -1;
}
```

### Задача 4: Find Minimum in Rotated Sorted Array (LeetCode #153)
**Ключова думка:** Мінімум — точка де починається rotation.  
Якщо `nums[mid] > nums[right]` → мінімум у правій половині.

```java
public int findMin(int[] nums) {
    int left = 0, right = nums.length - 1;

    while (left < right) {        // строго < (зупиняємось на одному елементі)
        int mid = left + (right - left) / 2;

        if (nums[mid] > nums[right]) {
            left = mid + 1;       // мінімум правіше
        } else {
            right = mid;          // mid може бути мінімумом
        }
    }

    return nums[left];
}
```

---

## 📐 Патерн 4: Binary Search on Answer

### Ключова думка
> Замість пошуку в масиві — **шукаємо в просторі відповідей**.  
> Якщо можна перевірити "чи можливо досягти результату X?" за O(n) → бінарний пошук по X дає O(n log n).

### Як розпізнати
1. Питання: "мінімальний максимум" або "максимальний мінімум"
2. Є монотонна умова: якщо X можливо → X-1 теж можливо (або навпаки)
3. Відповідь — числове значення в певному діапазоні

### Шаблон

```java
int left = minPossibleAnswer;
int right = maxPossibleAnswer;

while (left < right) {
    int mid = left + (right - left) / 2;

    if (canAchieve(mid)) {
        right = mid;       // шукаємо менше (мінімізація)
        // або: left = mid + 1; (максимізація)
    } else {
        left = mid + 1;    // mid замалий
    }
}

return left;

// Функція перевірки — чи можна досягти результату mid
boolean canAchieve(int mid) {
    // O(n) перевірка
}
```

### Задача 5: Koko Eating Bananas (LeetCode #875)
**Умова:** Кoko їсть банани зі швидкістю k бананів/год. Знайти мінімальну k щоб з'їсти всі за h годин.

```java
public int minEatingSpeed(int[] piles, int h) {
    int left = 1;
    int right = Arrays.stream(piles).max().getAsInt(); // макс можлива швидкість

    while (left < right) {
        int mid = left + (right - left) / 2;

        if (canFinish(piles, mid, h)) {
            right = mid;       // можемо їсти повільніше
        } else {
            left = mid + 1;    // занадто повільно
        }
    }

    return left;
}

private boolean canFinish(int[] piles, int speed, int h) {
    int hours = 0;
    for (int pile : piles) {
        hours += (pile + speed - 1) / speed; // ceil division
    }
    return hours <= h;
}
```
**Складність:** O(n log m) де m — максимальна купа

---

### Задача 6: Minimum Number of Days to Make m Bouquets (LeetCode #1482)
**Умова:** Квіти розцвітають на день[i]-й день. Потрібно m букетів по k сусідніх квіток. Знайти мінімальний день.

```java
public int minDays(int[] bloomDay, int m, int k) {
    if ((long) m * k > bloomDay.length) return -1;

    int left = 1, right = Arrays.stream(bloomDay).max().getAsInt();

    while (left < right) {
        int mid = left + (right - left) / 2;
        if (canMake(bloomDay, m, k, mid)) right = mid;
        else left = mid + 1;
    }

    return left;
}

private boolean canMake(int[] bloomDay, int m, int k, int day) {
    int bouquets = 0, consecutive = 0;
    for (int bloom : bloomDay) {
        if (bloom <= day) {
            consecutive++;
            if (consecutive == k) {
                bouquets++;
                consecutive = 0;
            }
        } else {
            consecutive = 0;
        }
    }
    return bouquets >= m;
}
```

---

### Задача 7: Split Array Largest Sum (LeetCode #410) ⭐
**Умова:** Розбити масив на m підмасивів так щоб мінімізувати максимальну суму підмасиву.

**Ключова думка:** "мінімізувати максимум" → Binary Search on Answer.

```java
public int splitArray(int[] nums, int m) {
    int left = Arrays.stream(nums).max().getAsInt(); // мінімально можлива відповідь
    int right = Arrays.stream(nums).sum();            // максимально можлива відповідь

    while (left < right) {
        int mid = left + (right - left) / 2;
        if (canSplit(nums, m, mid)) right = mid;
        else left = mid + 1;
    }

    return left;
}

private boolean canSplit(int[] nums, int m, int maxSum) {
    int parts = 1, currentSum = 0;
    for (int num : nums) {
        if (currentSum + num > maxSum) {
            parts++;
            currentSum = num;
            if (parts > m) return false;
        } else {
            currentSum += num;
        }
    }
    return true;
}
```

---

## 📐 Патерн 5: Binary Search у 2D Matrix

### Задача 8: Search a 2D Matrix (LeetCode #74)
**Умова:** Matrix де кожен рядок відсортований і перший елемент рядка > останнього попереднього.

**Ключова думка:** Трактуємо matrix як плоский відсортований масив.

```java
public boolean searchMatrix(int[][] matrix, int target) {
    int m = matrix.length, n = matrix[0].length;
    int left = 0, right = m * n - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;
        int val = matrix[mid / n][mid % n]; // перетворення індексу

        if (val == target) return true;
        else if (val < target) left = mid + 1;
        else right = mid - 1;
    }

    return false;
}
```

### Задача 9: Search a 2D Matrix II (LeetCode #240)
**Умова:** Кожен рядок і стовпець відсортовані (але рядки не пов'язані між собою).

**Ключова думка:** Починаємо з правого верхнього кута — рухаємось вниз або вліво.

```java
public boolean searchMatrix(int[][] matrix, int target) {
    int row = 0, col = matrix[0].length - 1; // правий верхній кут

    while (row < matrix.length && col >= 0) {
        if (matrix[row][col] == target) return true;
        else if (matrix[row][col] < target) row++;    // потрібно більше → йдемо вниз
        else col--;                                    // занадто велике → йдемо вліво
    }

    return false;
}
```
**Складність:** O(m + n) time

---

## 📐 Патерн 6: Коли сортувати перед вирішенням

### Коли сортування допомагає
- Задача на пари/трійки → сортуємо + Two Pointers
- Задача на intervals → сортуємо за початком
- Задача де порядок не важливий але важлива впорядкованість
- Greedy задачі

### Задача 10: Merge Intervals (LeetCode #56)
**Умова:** Злити перекриваючіся інтервали.

```java
public int[][] merge(int[][] intervals) {
    Arrays.sort(intervals, (a, b) -> a[0] - b[0]); // сортуємо за початком

    List<int[]> result = new ArrayList<>();
    result.add(intervals[0]);

    for (int i = 1; i < intervals.length; i++) {
        int[] last = result.get(result.size() - 1);
        int[] curr = intervals[i];

        if (curr[0] <= last[1]) {
            // перекриваються → розширюємо останній інтервал
            last[1] = Math.max(last[1], curr[1]);
        } else {
            result.add(curr);
        }
    }

    return result.toArray(new int[0][]);
}
```

### Задача 11: Meeting Rooms II (LeetCode #253)
**Умова:** Мінімальна кількість кімнат для всіх зустрічей.

```java
public int minMeetingRooms(int[][] intervals) {
    int n = intervals.length;
    int[] starts = new int[n];
    int[] ends = new int[n];

    for (int i = 0; i < n; i++) {
        starts[i] = intervals[i][0];
        ends[i] = intervals[i][1];
    }

    Arrays.sort(starts);
    Arrays.sort(ends);

    int rooms = 0, endPtr = 0;

    for (int i = 0; i < n; i++) {
        if (starts[i] < ends[endPtr]) {
            rooms++; // нова зустріч починається до закінчення попередньої
        } else {
            endPtr++; // звільнилась кімната
        }
    }

    return rooms;
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Binary Search / Sorting
│
├── Відсортований масив, знайти точне значення?
│   └── КЛАСИЧНИЙ Binary Search (left <= right)
│
├── Знайти першу/останню позицію?
│   └── LEFT / RIGHT BOUND (left < right)
│
├── Rotated sorted array?
│   └── Binary Search + перевірка якої половини відсортована
│
├── "Мінімальний максимум" / "максимальний мінімум"?
│   └── BINARY SEARCH ON ANSWER
│       → визначити діапазон відповідей
│       → написати canAchieve(mid) за O(n)
│
├── Відсортована матриця?
│   ├── Рядки пов'язані → flatten → Binary Search
│   └── Тільки рядки/стовпці → правий верхній кут
│
└── Пари/трійки / інтервали?
    └── СОРТУВАННЯ + Two Pointers або Greedy
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `mid = (left + right) / 2` | `mid = left + (right - left) / 2` (overflow!) |
| `while (left <= right)` у bound search | `while (left < right)` для left/right bound |
| `right = arr.length - 1` у bound search | `right = arr.length` (включаємо позицію "за масивом") |
| Не перевіряти результат leftBound | `if (left == n \|\| nums[left] != target) return -1` |
| Неправильний діапазон у Binary Search on Answer | `left = мін. можлива`, `right = макс. можлива` |
| `pile / speed` замість ceil ділення | `(pile + speed - 1) / speed` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #704 Binary Search
- [ ] #34 Find First and Last Position
- [ ] #33 Search in Rotated Sorted Array
- [ ] #153 Find Minimum in Rotated Sorted Array
- [ ] #56 Merge Intervals
- [ ] #74 Search a 2D Matrix

### Should Solve (Middle)
- [ ] #875 Koko Eating Bananas
- [ ] #1482 Minimum Number of Days to Make Bouquets
- [ ] #410 Split Array Largest Sum
- [ ] #240 Search a 2D Matrix II
- [ ] #253 Meeting Rooms II
- [ ] #162 Find Peak Element

### Stretch Goals
- [ ] #4 Median of Two Sorted Arrays (Hard)
- [ ] #302 Smallest Rectangle Enclosing Black Pixels
- [ ] #1231 Divide Chocolate

---

## 🔑 Quick Reference: Java Sorting

```java
// Масив примітивів
Arrays.sort(arr);                          // O(n log n)
Arrays.sort(arr, fromIndex, toIndex);      // сортує діапазон [from, to)

// Масив об'єктів з Comparator
Arrays.sort(intervals, (a, b) -> a[0] - b[0]);         // за першим елементом
Arrays.sort(intervals, (a, b) -> a[1] - b[1]);         // за другим елементом
Arrays.sort(strs, (a, b) -> a.length() - b.length());  // за довжиною

// List
Collections.sort(list);
Collections.sort(list, Comparator.reverseOrder());
list.sort((a, b) -> b - a); // descending

// Comparator composing
Arrays.sort(arr, Comparator.comparingInt((int[] a) -> a[0])
                            .thenComparingInt(a -> a[1]));

// Власний клас
Arrays.sort(people, (a, b) -> {
    if (a.age != b.age) return a.age - b.age;
    return a.name.compareTo(b.name);
});

// Ceil division (важливо для Binary Search on Answer)
int ceil = (a + b - 1) / b;  // або Math.ceilDiv(a, b) у Java 18+
```
