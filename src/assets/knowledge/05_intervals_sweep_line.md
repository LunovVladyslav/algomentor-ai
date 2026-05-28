# 05. Intervals & Sweep Line

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 2–3 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "merge overlapping intervals" | Sort + Greedy merge |
| "insert interval", "add to schedule" | Binary Search + merge |
| "minimum rooms / platforms / workers" | Sweep Line або Two Arrays Sort |
| "non-overlapping intervals", "remove minimum" | Greedy (sort by end) |
| "can attend all meetings" | Sort by start + перевірка overlap |
| "employee free time", "common free slots" | Merge All + знайти gaps |
| "event starts/ends", "count overlaps at time T" | Sweep Line (events array) |

---

## 📐 Ключова концепція: Overlap

### Коли два інтервали перекриваються?

```
A: [a1, a2]
B: [b1, b2]

Перекриваються якщо: a1 <= b2 AND b1 <= a2

НЕ перекриваються якщо: a2 < b1 (A закінчується до початку B)
                      або b2 < a1 (B закінчується до початку A)
```

```java
boolean overlaps(int[] a, int[] b) {
    return a[0] <= b[1] && b[0] <= a[1];
}
```

---

## 📐 Патерн 1: Sort + Greedy Merge

### Ключова думка
Після сортування за початком — перекриваючіся інтервали завжди **сусідні**.  
Порівнюємо кожний новий інтервал з останнім у результаті.

### Шаблон

```java
Arrays.sort(intervals, (a, b) -> a[0] - b[0]); // сортуємо за початком

List<int[]> result = new ArrayList<>();
result.add(intervals[0]);

for (int i = 1; i < intervals.length; i++) {
    int[] last = result.get(result.size() - 1);
    int[] curr = intervals[i];

    if (curr[0] <= last[1]) {
        // перекриваються → розширюємо
        last[1] = Math.max(last[1], curr[1]);
    } else {
        // не перекриваються → додаємо новий
        result.add(curr);
    }
}
```

### Задача 1: Merge Intervals (LeetCode #56)

```java
public int[][] merge(int[][] intervals) {
    Arrays.sort(intervals, (a, b) -> a[0] - b[0]);

    List<int[]> result = new ArrayList<>();
    result.add(intervals[0]);

    for (int i = 1; i < intervals.length; i++) {
        int[] last = result.get(result.size() - 1);
        int[] curr = intervals[i];

        if (curr[0] <= last[1]) {
            last[1] = Math.max(last[1], curr[1]);
        } else {
            result.add(curr);
        }
    }

    return result.toArray(new int[0][]);
}
```
**Складність:** O(n log n) time, O(n) space

---

## 📐 Патерн 2: Insert Interval

### Ключова думка
Три фази:
1. Додати всі інтервали що **закінчуються до** початку нового
2. Злити всі що **перекриваються** з новим
3. Додати решту що **починаються після** нового

### Задача 2: Insert Interval (LeetCode #57)

```java
public int[][] insert(int[][] intervals, int[] newInterval) {
    List<int[]> result = new ArrayList<>();
    int i = 0, n = intervals.length;

    // Фаза 1: інтервали що закінчуються до початку newInterval
    while (i < n && intervals[i][1] < newInterval[0]) {
        result.add(intervals[i++]);
    }

    // Фаза 2: злиття перекриваючихся інтервалів
    while (i < n && intervals[i][0] <= newInterval[1]) {
        newInterval[0] = Math.min(newInterval[0], intervals[i][0]);
        newInterval[1] = Math.max(newInterval[1], intervals[i][1]);
        i++;
    }
    result.add(newInterval);

    // Фаза 3: інтервали що починаються після newInterval
    while (i < n) {
        result.add(intervals[i++]);
    }

    return result.toArray(new int[0][]);
}
```
**Складність:** O(n) time (масив вже відсортований)

---

## 📐 Патерн 3: Greedy — мінімальна кількість (видалення / вибір)

### Ключова думка
**Сортуємо за кінцем** інтервалу.  
Жадібно вибираємо інтервал що закінчується найраніше → залишає максимум місця для наступних.

### Задача 3: Non-overlapping Intervals (LeetCode #435)
**Умова:** Мінімальна кількість інтервалів що треба видалити щоб не було перекрить.

```java
public int eraseOverlapIntervals(int[][] intervals) {
    // сортуємо за кінцем — жадібний вибір
    Arrays.sort(intervals, (a, b) -> a[1] - b[1]);

    int count = 0;          // кількість видалених
    int lastEnd = Integer.MIN_VALUE;

    for (int[] interval : intervals) {
        if (interval[0] >= lastEnd) {
            // не перекривається → залишаємо
            lastEnd = interval[1];
        } else {
            // перекривається → видаляємо (поточний, бо він закінчується пізніше)
            count++;
        }
    }

    return count;
}
```
**Складність:** O(n log n) time, O(1) space

---

### Задача 4: Minimum Number of Arrows to Burst Balloons (LeetCode #452)
**Умова:** Кулі — відрізки на осі X. Стріла пронизує всі кулі на своїй X-позиції. Мінімальна кількість стріл.

**Ключова думка:** Та сама ідея — сортуємо за кінцем, жадібно вибираємо стрілу на кінці поточного інтервалу.

```java
public int findMinArrowShots(int[][] points) {
    Arrays.sort(points, (a, b) -> Integer.compare(a[1], b[1])); // compare щоб уникнути overflow

    int arrows = 1;
    int arrowPos = points[0][1]; // стріла на кінці першої кулі

    for (int i = 1; i < points.length; i++) {
        if (points[i][0] > arrowPos) {
            // куля починається після поточної стріли → потрібна нова
            arrows++;
            arrowPos = points[i][1];
        }
        // інакше поточна стріла вже пронизує цю кулю
    }

    return arrows;
}
```

> ⚠️ Використовуй `Integer.compare(a[1], b[1])` замість `a[1] - b[1]` коли значення можуть бути від `Integer.MIN_VALUE` до `Integer.MAX_VALUE` — різниця може переповнитися!

---

## 📐 Патерн 4: Sweep Line — підрахунок максимального overlap

### Ключова думка
Замість порівняння пар інтервалів — генеруємо **події**:
- `+1` на початку інтервалу
- `-1` на кінці інтервалу

Сортуємо події → проходимо і знаходимо максимальне накопичення.

### Шаблон

```java
// Генеруємо події
List<int[]> events = new ArrayList<>();
for (int[] interval : intervals) {
    events.add(new int[]{interval[0], 1});  // початок → +1
    events.add(new int[]{interval[1], -1}); // кінець → -1
}

// Сортуємо: спочатку за часом, при рівному — кінець (-1) перед початком (+1)
events.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);

int current = 0, maxOverlap = 0;
for (int[] event : events) {
    current += event[1];
    maxOverlap = Math.max(maxOverlap, current);
}
```

### Задача 5: Meeting Rooms II (LeetCode #253)
**Умова:** Мінімальна кількість кімнат для всіх зустрічей.

**Підхід 1 — Two Sorted Arrays (простіший для пояснення на інтерв'ю):**

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
            rooms++; // нова зустріч починається до закінчення найранішої → +1 кімната
        } else {
            endPtr++; // найраніша зустріч закінчилась → кімната звільнилась
        }
    }

    return rooms;
}
```

**Підхід 2 — Sweep Line:**

```java
public int minMeetingRooms(int[][] intervals) {
    List<int[]> events = new ArrayList<>();
    for (int[] i : intervals) {
        events.add(new int[]{i[0], 1});
        events.add(new int[]{i[1], -1});
    }
    // при рівному часі: спочатку закінчення (-1), потім початок (1)
    events.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);

    int rooms = 0, maxRooms = 0;
    for (int[] e : events) {
        rooms += e[1];
        maxRooms = Math.max(maxRooms, rooms);
    }

    return maxRooms;
}
```
**Складність:** O(n log n) time, O(n) space

---

### Задача 6: Car Pooling (LeetCode #1094)
**Умова:** Пасажири сідають і виходять на зупинках. Чи вміщує автобус capacity пасажирів?

```java
public boolean carPooling(int[][] trips, int capacity) {
    // зупинки від 0 до 1000
    int[] stops = new int[1001];

    for (int[] trip : trips) {
        stops[trip[1]] += trip[0]; // пасажири сідають
        stops[trip[2]] -= trip[0]; // пасажири виходять
    }

    int current = 0;
    for (int passengers : stops) {
        current += passengers;
        if (current > capacity) return false;
    }

    return true;
}
```

> 💡 Коли діапазон значень обмежений (тут 0–1000) — замість List подій використовуємо **масив-різницевий** (difference array). Швидше і простіше.

---

## 📐 Патерн 5: Difference Array (різницевий масив)

### Коли використовувати
- Range update: додати значення до всіх елементів на відрізку `[l, r]`
- Потрібно відповісти на запити після всіх оновлень

### Шаблон

```java
int[] diff = new int[n + 1];

// Додати val до всіх елементів [l, r]
void rangeUpdate(int l, int r, int val) {
    diff[l] += val;
    diff[r + 1] -= val;
}

// Відновити оригінальний масив з префіксними сумами
int[] result = new int[n];
int current = 0;
for (int i = 0; i < n; i++) {
    current += diff[i];
    result[i] = current;
}
```

### Задача 7: Corporate Flight Bookings (LeetCode #1109)
**Умова:** Для кожного бронювання `[first, last, seats]` — додати `seats` до рейсів `first..last`. Повернути загальну кількість місць для кожного рейсу.

```java
public int[] corpFlightBookings(int[][] bookings, int n) {
    int[] diff = new int[n + 1];

    for (int[] b : bookings) {
        diff[b[0] - 1] += b[2];  // 1-indexed → 0-indexed
        diff[b[1]] -= b[2];
    }

    int[] result = new int[n];
    int current = 0;
    for (int i = 0; i < n; i++) {
        current += diff[i];
        result[i] = current;
    }

    return result;
}
```
**Складність:** O(n + m) time де m — кількість бронювань

---

## 📐 Патерн 6: Interval Scheduling — вибір максимальної кількості

### Задача 8: Can Attend All Meetings (LeetCode #252)
**Умова:** Чи може людина відвідати всі зустрічі?

```java
public boolean canAttendMeetings(int[][] intervals) {
    Arrays.sort(intervals, (a, b) -> a[0] - b[0]);

    for (int i = 1; i < intervals.length; i++) {
        if (intervals[i][0] < intervals[i - 1][1]) {
            return false; // перекриття
        }
    }

    return true;
}
```

---

### Задача 9: Task Scheduler (LeetCode #621)
**Умова:** Задачі з cooldown n між однаковими. Мінімальний час виконання всіх задач.

**Ключова думка:** Найчастіша задача визначає мінімальний frame.

```java
public int leastInterval(char[] tasks, int n) {
    int[] freq = new int[26];
    for (char task : tasks) freq[task - 'A']++;

    int maxFreq = Arrays.stream(freq).max().getAsInt();

    // кількість задач з максимальною частотою
    int maxCount = 0;
    for (int f : freq) if (f == maxFreq) maxCount++;

    // формула: (maxFreq - 1) * (n + 1) + maxCount
    int minTime = (maxFreq - 1) * (n + 1) + maxCount;

    // але якщо задач багато → просто tasks.length
    return Math.max(minTime, tasks.length);
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на інтервали
│
├── "злити / об'єднати інтервали"?
│   └── Sort by start + Greedy merge
│
├── "вставити новий інтервал"?
│   └── 3 фази: до / overlap / після
│
├── "мінімум видалень / максимум невідповідних"?
│   └── Sort by END + Greedy (залишаємо що закінчується раніше)
│
├── "мінімум кімнат / ресурсів"?
│   ├── Two sorted arrays (starts + ends)
│   └── Sweep Line (events +1/-1)
│
├── "range update" на фіксованому діапазоні?
│   └── Difference Array
│
└── "перекриваються чи ні"?
    └── Sort by start + порівняти curr[0] з prev[1]
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `a[1] - b[1]` у Comparator з великими числами | `Integer.compare(a[1], b[1])` — уникаємо overflow |
| Сортувати за початком для задачі на мінімум видалень | Сортувати за **кінцем** для greedy вибору |
| У Sweep Line: кінець і початок в одну точку — порядок важливий | Кінець (-1) обробляти **перед** початком (+1) при однаковому часі |
| `last[1] = curr[1]` при merge | `last[1] = Math.max(last[1], curr[1])` — curr може бути всередині last |
| 1-indexed задачі: не конвертувати індекс | Конвертувати: `diff[b[0] - 1]` для 1-indexed |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #56 Merge Intervals
- [ ] #57 Insert Interval
- [ ] #252 Can Attend All Meetings
- [ ] #253 Meeting Rooms II
- [ ] #435 Non-overlapping Intervals

### Should Solve (Middle)
- [ ] #452 Minimum Number of Arrows to Burst Balloons
- [ ] #1094 Car Pooling
- [ ] #1109 Corporate Flight Bookings
- [ ] #621 Task Scheduler
- [ ] #986 Interval List Intersections

### Stretch Goals
- [ ] #759 Employee Free Time
- [ ] #732 My Calendar III (Sweep Line + TreeMap)
- [ ] #218 The Skyline Problem

---

## 🔑 Quick Reference: Intervals у Java

```java
// Сортування інтервалів
Arrays.sort(intervals, (a, b) -> a[0] - b[0]);          // за початком
Arrays.sort(intervals, (a, b) -> a[1] - b[1]);          // за кінцем
Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1])); // безпечно

// Перевірка overlap
boolean overlaps = a[0] <= b[1] && b[0] <= a[1];
boolean noOverlap = a[1] < b[0] || b[1] < a[0];

// Злиття двох інтервалів
int mergedStart = Math.min(a[0], b[0]);
int mergedEnd = Math.max(a[1], b[1]);

// List<int[]> → int[][]
result.toArray(new int[0][]);

// TreeMap для Sweep Line (коли потрібен відсортований порядок подій)
TreeMap<Integer, Integer> events = new TreeMap<>();
events.merge(start, 1, Integer::sum);
events.merge(end, -1, Integer::sum);
for (Map.Entry<Integer, Integer> e : events.entrySet()) {
    current += e.getValue();
}
```
