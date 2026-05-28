# 07. Stack, Queue, Deque & Monotonic

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 3–4 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "valid parentheses", "balanced brackets" | Stack |
| "next greater / smaller element" | Monotonic Stack |
| "daily temperatures", "stock span" | Monotonic Stack |
| "largest rectangle in histogram" | Monotonic Stack |
| "sliding window maximum / minimum" | Monotonic Deque |
| "implement queue using stacks" | Two Stacks |
| "evaluate expression", "calculator" | Stack (operator precedence) |
| "BFS", "level order traversal" | Queue |
| "decode string", "nested structure" | Stack |

---

## 📐 Патерн 1: Stack — задачі на дужки

### Ключова думка
Stack ідеально підходить для **вкладених структур** — відкриваюча дужка входить у стек, закриваюча — перевіряє вершину.

### Задача 1: Valid Parentheses (LeetCode #20)

```java
public boolean isValid(String s) {
    Deque<Character> stack = new ArrayDeque<>();

    for (char c : s.toCharArray()) {
        if (c == '(' || c == '{' || c == '[') {
            stack.push(c);
        } else {
            if (stack.isEmpty()) return false;

            char top = stack.pop();
            if (c == ')' && top != '(') return false;
            if (c == '}' && top != '{') return false;
            if (c == ']' && top != '[') return false;
        }
    }

    return stack.isEmpty(); // стек має бути порожній
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 2: Minimum Remove to Make Valid Parentheses (LeetCode #1249)
**Умова:** Видалити мінімальну кількість дужок щоб зробити рядок валідним.

```java
public String minRemoveToMakeValid(String s) {
    Deque<Integer> stack = new ArrayDeque<>(); // індекси '('
    Set<Integer> toRemove = new HashSet<>();

    for (int i = 0; i < s.length(); i++) {
        char c = s.charAt(i);
        if (c == '(') {
            stack.push(i);
        } else if (c == ')') {
            if (stack.isEmpty()) {
                toRemove.add(i); // зайва ')' — видалити
            } else {
                stack.pop(); // знайшли пару
            }
        }
    }

    // залишки у стеку — непарні '('
    while (!stack.isEmpty()) toRemove.add(stack.pop());

    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
        if (!toRemove.contains(i)) sb.append(s.charAt(i));
    }

    return sb.toString();
}
```

---

### Задача 3: Decode String (LeetCode #394)
**Умова:** `"3[a2[bc]]"` → `"abcbcabcbcabcbc"`

**Ключова думка:** Два стеки — один для чисел, один для рядків.

```java
public String decodeString(String s) {
    Deque<Integer> countStack = new ArrayDeque<>();
    Deque<StringBuilder> strStack = new ArrayDeque<>();
    StringBuilder current = new StringBuilder();
    int k = 0;

    for (char c : s.toCharArray()) {
        if (Character.isDigit(c)) {
            k = k * 10 + (c - '0'); // число може бути багатозначним
        } else if (c == '[') {
            countStack.push(k);
            strStack.push(current);
            current = new StringBuilder();
            k = 0;
        } else if (c == ']') {
            int count = countStack.pop();
            StringBuilder prev = strStack.pop();
            // повторюємо current count разів і додаємо до prev
            for (int i = 0; i < count; i++) prev.append(current);
            current = prev;
        } else {
            current.append(c);
        }
    }

    return current.toString();
}
```
**Складність:** O(n * max_k) time де max_k — максимальний множник

---

### Задача 4: Basic Calculator II (LeetCode #227)
**Умова:** Обчислити вираз з `+`, `-`, `*`, `/` (без дужок).

**Ключова думка:** Застосовуємо операцію **попереднього** знаку до поточного числа. `*` і `/` мають вищий пріоритет → виконуємо одразу.

```java
public int calculate(String s) {
    Deque<Integer> stack = new ArrayDeque<>();
    int num = 0;
    char op = '+'; // починаємо з '+'

    for (int i = 0; i < s.length(); i++) {
        char c = s.charAt(i);

        if (Character.isDigit(c)) {
            num = num * 10 + (c - '0');
        }

        // обробляємо коли зустрічаємо оператор або кінець рядка
        if ((!Character.isDigit(c) && c != ' ') || i == s.length() - 1) {
            if (op == '+') stack.push(num);
            else if (op == '-') stack.push(-num);
            else if (op == '*') stack.push(stack.pop() * num);
            else if (op == '/') stack.push(stack.pop() / num);

            op = c;
            num = 0;
        }
    }

    int result = 0;
    while (!stack.isEmpty()) result += stack.pop();
    return result;
}
```

---

## 📐 Патерн 2: Monotonic Stack

### Ключова думка
Monotonic Stack — стек де елементи завжди у **зростаючому або спадаючому** порядку.

- **Monotonic Increasing Stack** (знизу до верху зростає) → знаходить **Next Smaller Element**
- **Monotonic Decreasing Stack** (знизу до верху спадає) → знаходить **Next Greater Element**

### Шаблон — Next Greater Element

```java
int[] result = new int[n];
Arrays.fill(result, -1); // дефолт: немає більшого
Deque<Integer> stack = new ArrayDeque<>(); // зберігаємо індекси

for (int i = 0; i < n; i++) {
    // поки поточний елемент більший за вершину стеку
    while (!stack.isEmpty() && nums[i] > nums[stack.peek()]) {
        int idx = stack.pop();
        result[idx] = nums[i]; // nums[i] — next greater для idx
    }
    stack.push(i);
}
// елементи що залишились у стеку — не мають next greater
```

### Задача 5: Daily Temperatures (LeetCode #739)
**Умова:** Для кожного дня знайти кількість днів до наступного теплішого дня.

```java
public int[] dailyTemperatures(int[] temperatures) {
    int n = temperatures.length;
    int[] result = new int[n]; // дефолт 0
    Deque<Integer> stack = new ArrayDeque<>(); // індекси

    for (int i = 0; i < n; i++) {
        while (!stack.isEmpty() && temperatures[i] > temperatures[stack.peek()]) {
            int idx = stack.pop();
            result[idx] = i - idx; // різниця індексів = кількість днів
        }
        stack.push(i);
    }

    return result;
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 6: Next Greater Element I (LeetCode #496)
**Умова:** Для елементів nums1 (підмножина nums2) знайти next greater у nums2.

```java
public int[] nextGreaterElement(int[] nums1, int[] nums2) {
    // Будуємо map: значення → next greater у nums2
    Map<Integer, Integer> nextGreater = new HashMap<>();
    Deque<Integer> stack = new ArrayDeque<>();

    for (int num : nums2) {
        while (!stack.isEmpty() && num > stack.peek()) {
            nextGreater.put(stack.pop(), num);
        }
        stack.push(num);
    }

    int[] result = new int[nums1.length];
    for (int i = 0; i < nums1.length; i++) {
        result[i] = nextGreater.getOrDefault(nums1[i], -1);
    }

    return result;
}
```

---

### Задача 7: Largest Rectangle in Histogram (LeetCode #84) ⭐
**Умова:** Знайти найбільший прямокутник у гістограмі.

**Ключова думка:** Для кожного стовпця знайти межі — як далеко вліво і вправо може розтягнутися прямокутник його висоти. Monotonic Increasing Stack — коли знаходимо менший елемент, рахуємо площу для вершини стеку.

```java
public int largestRectangleArea(int[] heights) {
    Deque<Integer> stack = new ArrayDeque<>(); // monotonic increasing (індекси)
    int maxArea = 0;
    int n = heights.length;

    for (int i = 0; i <= n; i++) {
        int currHeight = (i == n) ? 0 : heights[i]; // sentinel 0 в кінці

        while (!stack.isEmpty() && currHeight < heights[stack.peek()]) {
            int height = heights[stack.pop()];
            // ширина: від поточного i до наступного у стеку
            int width = stack.isEmpty() ? i : i - stack.peek() - 1;
            maxArea = Math.max(maxArea, height * width);
        }

        stack.push(i);
    }

    return maxArea;
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 8: Trapping Rain Water (LeetCode #42) ⭐
**Підхід 1 — Monotonic Stack:**

```java
public int trap(int[] height) {
    Deque<Integer> stack = new ArrayDeque<>();
    int water = 0;

    for (int i = 0; i < height.length; i++) {
        while (!stack.isEmpty() && height[i] > height[stack.peek()]) {
            int bottom = stack.pop();

            if (stack.isEmpty()) break;

            int left = stack.peek();
            int width = i - left - 1;
            int boundedHeight = Math.min(height[left], height[i]) - height[bottom];
            water += width * boundedHeight;
        }
        stack.push(i);
    }

    return water;
}
```

**Підхід 2 — Two Pointers (O(1) space, простіший):**

```java
public int trap(int[] height) {
    int left = 0, right = height.length - 1;
    int leftMax = 0, rightMax = 0;
    int water = 0;

    while (left < right) {
        if (height[left] < height[right]) {
            if (height[left] >= leftMax) leftMax = height[left];
            else water += leftMax - height[left];
            left++;
        } else {
            if (height[right] >= rightMax) rightMax = height[right];
            else water += rightMax - height[right];
            right--;
        }
    }

    return water;
}
```
**Складність:** O(n) time, O(1) space

---

## 📐 Патерн 3: Monotonic Deque (Sliding Window Maximum)

### Ключова думка
Deque зберігає індекси у **спадаючому** порядку значень.  
- Спереду (front) завжди **максимальний** елемент поточного вікна
- Нові елементи додаємо ззаду, видаляючи менші за них
- Застарілі елементи видаляємо спереду

### Задача 9: Sliding Window Maximum (LeetCode #239)
**Умова:** Максимум у кожному вікні розміру k.

```java
public int[] maxSlidingWindow(int[] nums, int k) {
    int n = nums.length;
    int[] result = new int[n - k + 1];
    Deque<Integer> deque = new ArrayDeque<>(); // індекси, спадаючі значення

    for (int i = 0; i < n; i++) {
        // видаляємо застарілі елементи (за межами вікна)
        while (!deque.isEmpty() && deque.peekFirst() < i - k + 1) {
            deque.pollFirst();
        }

        // видаляємо елементи менші за поточний (вони ніколи не будуть максимумом)
        while (!deque.isEmpty() && nums[deque.peekLast()] < nums[i]) {
            deque.pollLast();
        }

        deque.offerLast(i);

        // записуємо результат (коли вікно повністю заповнене)
        if (i >= k - 1) {
            result[i - k + 1] = nums[deque.peekFirst()];
        }
    }

    return result;
}
```
**Складність:** O(n) time, O(k) space

---

## 📐 Патерн 4: Two Stacks — реалізація Queue

### Задача 10: Implement Queue using Stacks (LeetCode #232)
**Ключова думка:** `inStack` — для push, `outStack` — для pop/peek. Переливаємо з in до out тільки коли out порожній.

```java
class MyQueue {
    private Deque<Integer> inStack = new ArrayDeque<>();
    private Deque<Integer> outStack = new ArrayDeque<>();

    public void push(int x) {
        inStack.push(x);
    }

    public int pop() {
        move();
        return outStack.pop();
    }

    public int peek() {
        move();
        return outStack.peek();
    }

    public boolean empty() {
        return inStack.isEmpty() && outStack.isEmpty();
    }

    // переливаємо тільки коли outStack порожній → амортизована O(1)
    private void move() {
        if (outStack.isEmpty()) {
            while (!inStack.isEmpty()) {
                outStack.push(inStack.pop());
            }
        }
    }
}
```
**Складність:** Амортизована O(1) для всіх операцій

---

### Задача 11: Implement Stack using Queues (LeetCode #225)

```java
class MyStack {
    private Queue<Integer> queue = new LinkedList<>();

    public void push(int x) {
        queue.offer(x);
        // rotate: переміщуємо всі елементи крім нового в кінець
        for (int i = 0; i < queue.size() - 1; i++) {
            queue.offer(queue.poll());
        }
    }

    public int pop() { return queue.poll(); }
    public int top() { return queue.peek(); }
    public boolean empty() { return queue.isEmpty(); }
}
```

---

## 📐 Патерн 5: Stack для обходу (ітеративний DFS)

### Задача 12: Min Stack (LeetCode #155)
**Умова:** Stack з O(1) `getMin()`.

```java
class MinStack {
    private Deque<Integer> stack = new ArrayDeque<>();
    private Deque<Integer> minStack = new ArrayDeque<>(); // паралельний стек мінімумів

    public void push(int val) {
        stack.push(val);
        // зберігаємо мінімум: або новий val, або попередній мінімум
        int currentMin = minStack.isEmpty() ? val : Math.min(val, minStack.peek());
        minStack.push(currentMin);
    }

    public void pop() {
        stack.pop();
        minStack.pop();
    }

    public int top() { return stack.peek(); }

    public int getMin() { return minStack.peek(); }
}
```
**Складність:** O(1) для всіх операцій

---

### Задача 13: Asteroid Collision (LeetCode #735)
**Умова:** Астероїди рухаються вправо (>0) або вліво (<0). Зіткнення знищують менший. Знайти що залишиться.

```java
public int[] asteroidCollision(int[] asteroids) {
    Deque<Integer> stack = new ArrayDeque<>();

    for (int ast : asteroids) {
        boolean destroyed = false;

        // зіткнення: верхівка стеку летить вправо, ast летить вліво
        while (!stack.isEmpty() && ast < 0 && stack.peek() > 0) {
            if (stack.peek() < -ast) {
                stack.pop(); // верхівка менша → знищується
            } else if (stack.peek() == -ast) {
                stack.pop(); // рівні → обидва знищуються
                destroyed = true;
                break;
            } else {
                destroyed = true; // ast менший → знищується
                break;
            }
        }

        if (!destroyed) stack.push(ast);
    }

    // конвертуємо стек у масив (в правильному порядку)
    int[] result = new int[stack.size()];
    for (int i = result.length - 1; i >= 0; i--) {
        result[i] = stack.pop();
    }

    return result;
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Stack / Queue / Deque
│
├── "дужки", "вкладені структури", "decode"?
│   └── STACK (push відкриваючу, перевіряй при закриваючій)
│
├── "вираз", "calculator"?
│   └── STACK (зберігай числа, застосовуй попередній оператор)
│
├── "next greater/smaller element"?
│   └── MONOTONIC STACK
│       ├── next greater → decreasing stack
│       └── next smaller → increasing stack
│
├── "largest rectangle", "trapping rain water"?
│   └── MONOTONIC STACK (площа між границями)
│
├── "sliding window max/min"?
│   └── MONOTONIC DEQUE
│       ├── front → поточний max/min
│       ├── видаляй спереду застарілі
│       └── видаляй ззаду гірші кандидати
│
├── "O(1) min/max у stack"?
│   └── TWO STACKS (основний + допоміжний для min/max)
│
└── "queue через stacks" або "stack через queue"?
    └── TWO STACKS / QUEUE з rotate
```

---

## ⚠️ Типові помилки

| Помилка | Правилько |
|---|---|
| `Stack<Integer> stack = new Stack<>()` | Використовуй `Deque<Integer> stack = new ArrayDeque<>()` — Stack застарілий клас |
| `stack.peek()` без перевірки на порожність | `!stack.isEmpty() && stack.peek() ...` |
| У Monotonic Stack: зберігати значення замість індексів | Зберігай **індекси** — вони дають і значення і позицію |
| Забути sentinel у Histogram (`i == n ? 0 : heights[i]`) | Без sentinel останні елементи у стеку не обробляються |
| `deque.peek()` — який кінець? | `peekFirst()` = front, `peekLast()` = back — будь точним |
| У MinStack: `min(val, minStack.peek())` при push | Якщо minStack порожній → `val` є мінімумом |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #20 Valid Parentheses
- [ ] #155 Min Stack
- [ ] #739 Daily Temperatures
- [ ] #496 Next Greater Element I
- [ ] #232 Implement Queue using Stacks
- [ ] #394 Decode String

### Should Solve (Middle)
- [ ] #84 Largest Rectangle in Histogram
- [ ] #42 Trapping Rain Water
- [ ] #239 Sliding Window Maximum
- [ ] #227 Basic Calculator II
- [ ] #1249 Minimum Remove to Make Valid Parentheses
- [ ] #735 Asteroid Collision
- [ ] #225 Implement Stack using Queues

### Stretch Goals
- [ ] #85 Maximal Rectangle
- [ ] #907 Sum of Subarray Minimums
- [ ] #224 Basic Calculator (з дужками)
- [ ] #862 Shortest Subarray with Sum at Least K

---

## 🔑 Quick Reference: Java Stack / Queue / Deque

```java
// ✅ Рекомендований спосіб — ArrayDeque
Deque<Integer> stack = new ArrayDeque<>();
stack.push(x);        // додати на вершину
stack.pop();          // видалити з вершини
stack.peek();         // подивитись вершину
stack.isEmpty();

// Як Queue (FIFO)
Queue<Integer> queue = new ArrayDeque<>();
queue.offer(x);       // додати в кінець
queue.poll();         // видалити з початку
queue.peek();         // подивитись початок

// Deque — обидва кінці
Deque<Integer> deque = new ArrayDeque<>();
deque.offerFirst(x);  // додати спереду
deque.offerLast(x);   // додати ззаду
deque.pollFirst();    // видалити спереду
deque.pollLast();     // видалити ззаду
deque.peekFirst();    // подивитись спереду
deque.peekLast();     // подивитись ззаду

// PriorityQueue (Heap) — для задач з пріоритетом
PriorityQueue<Integer> minHeap = new PriorityQueue<>();
PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());
minHeap.offer(x);
minHeap.poll();       // видаляє мінімум
minHeap.peek();       // мінімум без видалення

// ❌ Не використовуй ці застарілі класи
// Stack<Integer> stack = new Stack<>();   // synchronized, повільний
// Queue<Integer> queue = new LinkedList<>(); // зайві витрати пам'яті
```
