# 01. Arrays, Two Pointers & Sliding Window

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 3–5 днів

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "find pair", "two numbers that sum to X" | Two Pointers або HashMap |
| "subarray with max/min/exact sum" | Sliding Window |
| "sorted array", "in-place" | Two Pointers |
| "contiguous subarray" | Sliding Window або Kadane |
| "remove duplicates", "move zeros" | Two Pointers (fast/slow) |
| "longest substring without repeating" | Sliding Window + Set |

---

## 📐 Патерн 1: Two Pointers (на відсортованому масиві)

### Коли використовувати
- Масив **відсортований** (або можна відсортувати)
- Шукаємо **пару** або **перевіряємо умову** з двох кінців

### Шаблон

```java
int left = 0, right = arr.length - 1;

while (left < right) {
    int sum = arr[left] + arr[right];

    if (sum == target) {
        // знайшли відповідь
        return new int[]{left, right};
    } else if (sum < target) {
        left++;   // потрібна більша сума → рухаємо лівий
    } else {
        right--;  // сума завелика → рухаємо правий
    }
}
```

### Задача 1: Two Sum II (LeetCode #167)
**Умова:** Відсортований масив, знайти два числа що дають target.

```java
public int[] twoSum(int[] numbers, int target) {
    int left = 0, right = numbers.length - 1;

    while (left < right) {
        int sum = numbers[left] + numbers[right];
        if (sum == target) return new int[]{left + 1, right + 1}; // 1-indexed
        else if (sum < target) left++;
        else right--;
    }

    return new int[]{-1, -1}; // не знайдено
}
```
**Складність:** O(n) time, O(1) space

---

### Задача 2: Container With Most Water (LeetCode #11)
**Умова:** Масив висот, знайти дві лінії що утворюють найбільший контейнер.

**Ключова думка:** Ширина = right - left. Якщо height[left] < height[right] → рухаємо left, бо менша висота обмежує об'єм.

```java
public int maxArea(int[] height) {
    int left = 0, right = height.length - 1;
    int maxWater = 0;

    while (left < right) {
        int water = Math.min(height[left], height[right]) * (right - left);
        maxWater = Math.max(maxWater, water);

        // рухаємо той pointer що вказує на меншу висоту
        if (height[left] < height[right]) left++;
        else right--;
    }

    return maxWater;
}
```
**Складність:** O(n) time, O(1) space

---

### Задача 3: 3Sum (LeetCode #15)
**Умова:** Знайти всі трійки що дають 0. Без дублікатів.

**Ключова думка:** Сортуємо → фіксуємо один елемент → Two Pointers на решті. Пропускаємо дублікати.

```java
public List<List<Integer>> threeSum(int[] nums) {
    Arrays.sort(nums);
    List<List<Integer>> result = new ArrayList<>();

    for (int i = 0; i < nums.length - 2; i++) {
        // пропускаємо дублікати для фіксованого елементу
        if (i > 0 && nums[i] == nums[i - 1]) continue;

        int left = i + 1, right = nums.length - 1;

        while (left < right) {
            int sum = nums[i] + nums[left] + nums[right];

            if (sum == 0) {
                result.add(Arrays.asList(nums[i], nums[left], nums[right]));
                // пропускаємо дублікати
                while (left < right && nums[left] == nums[left + 1]) left++;
                while (left < right && nums[right] == nums[right - 1]) right--;
                left++;
                right--;
            } else if (sum < 0) {
                left++;
            } else {
                right--;
            }
        }
    }

    return result;
}
```
**Складність:** O(n²) time, O(1) space (без врахування результату)

---

## 📐 Патерн 2: Two Pointers (Fast & Slow — модифікація масиву)

### Коли використовувати
- Потрібно **модифікувати масив in-place**
- Видалити елементи, перемістити нулі, прибрати дублікати

### Шаблон

```java
int slow = 0; // вказує на позицію де записати наступний "хороший" елемент

for (int fast = 0; fast < arr.length; fast++) {
    if (arr[fast] задовольняє умову) {
        arr[slow] = arr[fast];
        slow++;
    }
}
// slow = нова довжина масиву
```

### Задача 4: Remove Duplicates from Sorted Array (LeetCode #26)

```java
public int removeDuplicates(int[] nums) {
    if (nums.length == 0) return 0;

    int slow = 1; // перший елемент завжди унікальний

    for (int fast = 1; fast < nums.length; fast++) {
        if (nums[fast] != nums[fast - 1]) { // новий унікальний елемент
            nums[slow] = nums[fast];
            slow++;
        }
    }

    return slow;
}
```

### Задача 5: Move Zeroes (LeetCode #283)

```java
public void moveZeroes(int[] nums) {
    int slow = 0;

    // переміщуємо всі ненульові елементи вперед
    for (int fast = 0; fast < nums.length; fast++) {
        if (nums[fast] != 0) {
            nums[slow] = nums[fast];
            slow++;
        }
    }

    // заповнюємо решту нулями
    while (slow < nums.length) {
        nums[slow++] = 0;
    }
}
```

---

## 📐 Патерн 3: Sliding Window (фіксований розмір)

### Коли використовувати
- Підмасив **фіксованого розміру k**
- Maximum/minimum/average у вікні розміру k

### Шаблон

```java
// Крок 1: Побудувати перше вікно
int windowSum = 0;
for (int i = 0; i < k; i++) {
    windowSum += arr[i];
}

int maxSum = windowSum;

// Крок 2: Слайдити вікно
for (int i = k; i < arr.length; i++) {
    windowSum += arr[i];       // додаємо новий елемент
    windowSum -= arr[i - k];   // видаляємо старий елемент
    maxSum = Math.max(maxSum, windowSum);
}
```

### Задача 6: Maximum Average Subarray I (LeetCode #643)

```java
public double findMaxAverage(int[] nums, int k) {
    int windowSum = 0;
    for (int i = 0; i < k; i++) windowSum += nums[i];

    int maxSum = windowSum;

    for (int i = k; i < nums.length; i++) {
        windowSum += nums[i] - nums[i - k];
        maxSum = Math.max(maxSum, windowSum);
    }

    return (double) maxSum / k;
}
```

---

## 📐 Патерн 4: Sliding Window (динамічний розмір)

### Коли використовувати
- "Найдовший/найкоротший підмасив що задовольняє умову X"
- Умова може порушуватися і відновлюватися

### Шаблон

```java
int left = 0;
int result = 0;
// стан вікна (сума, частота тощо)

for (int right = 0; right < arr.length; right++) {
    // Expand: додаємо arr[right] до вікна
    // оновлюємо стан

    // Shrink: поки вікно не валідне — стягуємо зліва
    while (умова порушена) {
        // видаляємо arr[left] з вікна
        left++;
    }

    // вікно валідне → оновлюємо результат
    result = Math.max(result, right - left + 1);
}
```

### Задача 7: Longest Substring Without Repeating Characters (LeetCode #3)

```java
public int lengthOfLongestSubstring(String s) {
    Set<Character> window = new HashSet<>();
    int left = 0, result = 0;

    for (int right = 0; right < s.length(); right++) {
        char c = s.charAt(right);

        // Shrink: поки символ є у вікні — видаляємо зліва
        while (window.contains(c)) {
            window.remove(s.charAt(left));
            left++;
        }

        window.add(c);
        result = Math.max(result, right - left + 1);
    }

    return result;
}
```
**Складність:** O(n) time, O(min(n,m)) space де m — розмір алфавіту

---

### Задача 8: Minimum Size Subarray Sum (LeetCode #209)
**Умова:** Найкоротший підмасив сума якого ≥ target.

```java
public int minSubArrayLen(int target, int[] nums) {
    int left = 0, sum = 0;
    int minLen = Integer.MAX_VALUE;

    for (int right = 0; right < nums.length; right++) {
        sum += nums[right];

        // Shrink: поки сума достатня → намагаємось зменшити вікно
        while (sum >= target) {
            minLen = Math.min(minLen, right - left + 1);
            sum -= nums[left];
            left++;
        }
    }

    return minLen == Integer.MAX_VALUE ? 0 : minLen;
}
```

---

### Задача 9: Longest Subarray with At Most K Zeros (LeetCode #1004 — Flip Zeros)

```java
public int longestOnes(int[] nums, int k) {
    int left = 0, zeros = 0, result = 0;

    for (int right = 0; right < nums.length; right++) {
        if (nums[right] == 0) zeros++;

        // Shrink: якщо нулів більше ніж k
        while (zeros > k) {
            if (nums[left] == 0) zeros--;
            left++;
        }

        result = Math.max(result, right - left + 1);
    }

    return result;
}
```

---

## 📐 Патерн 5: Prefix Sum

### Коли використовувати
- Багато запитів "сума від i до j"
- Підмасив із заданою сумою (в комбінації з HashMap)

### Шаблон

```java
// Побудова prefix sum
int[] prefix = new int[n + 1];
for (int i = 0; i < n; i++) {
    prefix[i + 1] = prefix[i] + nums[i];
}

// Сума від l до r включно:
int rangeSum = prefix[r + 1] - prefix[l];
```

### Задача 10: Subarray Sum Equals K (LeetCode #560)
**Умова:** Знайти кількість підмасивів із сумою рівно k.

**Ключова думка:** prefix[j] - prefix[i] = k → prefix[i] = prefix[j] - k.  
Зберігаємо кількість входжень кожного prefix sum у HashMap.

```java
public int subarraySum(int[] nums, int k) {
    Map<Integer, Integer> prefixCount = new HashMap<>();
    prefixCount.put(0, 1); // порожній підмасив

    int sum = 0, count = 0;

    for (int num : nums) {
        sum += num;

        // якщо (sum - k) вже бачили → знайшли підмасив
        count += prefixCount.getOrDefault(sum - k, 0);

        prefixCount.merge(sum, 1, Integer::sum);
    }

    return count;
}
```
**Складність:** O(n) time, O(n) space

---

## 📐 Патерн 6: Kadane's Algorithm (Maximum Subarray)

### Коли використовувати
- "Maximum sum subarray" (contiguous)

### Шаблон + Задача 11: Maximum Subarray (LeetCode #53)

**Ключова думка:** На кожному кроці вирішуємо — продовжувати попередній підмасив чи почати новий.

```java
public int maxSubArray(int[] nums) {
    int currentSum = nums[0];
    int maxSum = nums[0];

    for (int i = 1; i < nums.length; i++) {
        // або продовжуємо підмасив, або починаємо новий з nums[i]
        currentSum = Math.max(nums[i], currentSum + nums[i]);
        maxSum = Math.max(maxSum, currentSum);
    }

    return maxSum;
}
```
**Складність:** O(n) time, O(1) space

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на масив/рядок
│
├── Відсортований масив + пара/трійка?
│   └── TWO POINTERS (left/right від країв)
│
├── Модифікація масиву in-place (видалити, перемістити)?
│   └── TWO POINTERS (fast/slow)
│
├── Підмасив фіксованого розміру k?
│   └── SLIDING WINDOW (фіксований)
│
├── Найдовший/найкоротший підмасив з умовою?
│   └── SLIDING WINDOW (динамічний)
│
├── Сума діапазону / кількість підмасивів з сумою k?
│   └── PREFIX SUM (+ HashMap)
│
└── Maximum sum contiguous subarray?
    └── KADANE'S ALGORITHM
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `while (left <= right)` у sliding window | `while (left < right)` або `for right + while shrink` |
| Не пропускати дублікати у 3Sum | `if (i > 0 && nums[i] == nums[i-1]) continue` |
| `prefix[r] - prefix[l]` | `prefix[r+1] - prefix[l]` (off-by-one) |
| Починати Kadane з 0 | Починати з `nums[0]` (масив може бути весь від'ємний) |
| Забути `prefixCount.put(0, 1)` | Без нього не рахуємо підмасиви що починаються з index 0 |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #1 Two Sum
- [ ] #167 Two Sum II
- [ ] #26 Remove Duplicates from Sorted Array
- [ ] #283 Move Zeroes
- [ ] #3 Longest Substring Without Repeating Characters
- [ ] #53 Maximum Subarray
- [ ] #209 Minimum Size Subarray Sum

### Should Solve (Middle)
- [ ] #11 Container With Most Water
- [ ] #15 3Sum
- [ ] #560 Subarray Sum Equals K
- [ ] #1004 Max Consecutive Ones III
- [ ] #424 Longest Repeating Character Replacement
- [ ] #76 Minimum Window Substring

### Stretch Goals
- [ ] #42 Trapping Rain Water
- [ ] #239 Sliding Window Maximum (Monotonic Deque)
- [ ] #992 Subarrays with K Different Integers

---

## 🔑 Quick Reference: Java Array Tricks

```java
// Сортування
Arrays.sort(arr);

// Копія масиву
int[] copy = Arrays.copyOf(arr, arr.length);
int[] slice = Arrays.copyOfRange(arr, left, right); // [left, right)

// Заповнення
Arrays.fill(arr, 0);

// Перетворення int[] → List
// (напряму не працює, потрібен Integer[])
Integer[] boxed = Arrays.stream(arr).boxed().toArray(Integer[]::new);

// Максимум/мінімум
int max = Arrays.stream(arr).max().getAsInt();

// HashMap часті операції
map.getOrDefault(key, 0);
map.merge(key, 1, Integer::sum);          // збільшити на 1
map.put(key, map.getOrDefault(key, 0) + 1); // те саме
```
