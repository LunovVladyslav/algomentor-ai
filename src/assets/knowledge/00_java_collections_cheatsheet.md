# Бонус: Java Collections Cheatsheet

> Швидкий довідник — коли яку колекцію використовувати на інтерв'ю

---

## 🗺️ Вибір колекції — дерево рішень

```
Потрібен список елементів?
├── Довільний доступ за індексом → ArrayList
├── Часті вставки/видалення з середини → LinkedList
└── Стек або Черга → ArrayDeque

Потрібен унікальний набір?
├── Порядок не важливий → HashSet       O(1) get/add
├── Відсортований порядок → TreeSet     O(log n)
└── Порядок вставки → LinkedHashSet    O(1)

Потрібна пара ключ-значення?
├── Порядок не важливий → HashMap       O(1) get/put
├── Відсортований за ключем → TreeMap  O(log n)
└── Порядок вставки → LinkedHashMap   O(1)

Потрібна черга з пріоритетом?
└── PriorityQueue (min-heap за замовч.)
```

---

## 📐 ArrayList vs ArrayDeque vs LinkedList

| Операція | ArrayList | ArrayDeque | LinkedList |
|---|---|---|---|
| get(i) | O(1) ✅ | O(n) | O(n) |
| add(end) | O(1) | O(1) | O(1) |
| add(front) | O(n) | O(1) ✅ | O(1) |
| remove(middle) | O(n) | O(n) | O(1) якщо є ref |
| Stack/Queue | ❌ | ✅ | ✅ |

**Висновок:** `ArrayDeque` — найкращий вибір для стеку і черги.  
`ArrayList` — для списку з доступом за індексом.

---

## 📐 HashMap — повний довідник

```java
Map<K, V> map = new HashMap<>();

// Базові операції
map.put(key, value);
map.get(key);                              // null якщо немає
map.getOrDefault(key, defaultValue);      // ✅ безпечно
map.containsKey(key);
map.containsValue(value);                 // O(n)
map.remove(key);
map.size();
map.isEmpty();
map.clear();

// Умовні операції
map.putIfAbsent(key, value);              // тільки якщо немає
map.computeIfAbsent(key, k -> new ArrayList<>()); // створити якщо немає
map.computeIfPresent(key, (k, v) -> v + 1);       // оновити якщо є

// Increment pattern (підрахунок частоти)
map.merge(key, 1, Integer::sum);
// еквівалентно: map.put(key, map.getOrDefault(key, 0) + 1)

// Ітерація
for (Map.Entry<K, V> entry : map.entrySet()) {
    K k = entry.getKey();
    V v = entry.getValue();
}
map.forEach((k, v) -> System.out.println(k + "=" + v));

// Перетворення
map.keySet()    // Set<K>
map.values()    // Collection<V>
map.entrySet()  // Set<Map.Entry<K,V>>
```

---

## 📐 TreeMap — відсортований за ключем

```java
TreeMap<K, V> treeMap = new TreeMap<>();

treeMap.firstKey();           // найменший ключ
treeMap.lastKey();            // найбільший ключ
treeMap.floorKey(key);        // найбільший ключ <= key
treeMap.ceilingKey(key);      // найменший ключ >= key
treeMap.lowerKey(key);        // найбільший ключ < key
treeMap.higherKey(key);       // найменший ключ > key
treeMap.headMap(toKey);       // ключі < toKey
treeMap.tailMap(fromKey);     // ключі >= fromKey
treeMap.subMap(from, to);     // ключі [from, to)
treeMap.descendingMap();      // у зворотному порядку

// Корисно для задач типу:
// - "знайти найближчу дату до X"
// - Sweep Line з відсортованими подіями
// - "count elements in range [a, b]"
```

---

## 📐 HashSet / TreeSet

```java
// HashSet — O(1) add/remove/contains
Set<T> set = new HashSet<>();
set.add(x);
set.remove(x);
set.contains(x);

// Set операції (змінюють перший set!)
Set<T> a = new HashSet<>(Arrays.asList(1,2,3));
Set<T> b = new HashSet<>(Arrays.asList(2,3,4));
a.retainAll(b);   // intersection: {2,3}
a.addAll(b);      // union: {1,2,3,4}
a.removeAll(b);   // difference: {1}

// TreeSet — O(log n), відсортований
TreeSet<Integer> ts = new TreeSet<>();
ts.first();       // мінімум
ts.last();        // максимум
ts.floor(x);      // найбільший <= x
ts.ceiling(x);    // найменший >= x
ts.headSet(x);    // елементи < x
ts.tailSet(x);    // елементи >= x

// LinkedHashSet — зберігає порядок вставки
Set<T> linked = new LinkedHashSet<>();
```

---

## 📐 ArrayDeque — Stack і Queue

```java
Deque<T> deque = new ArrayDeque<>();

// Як STACK (LIFO)
deque.push(x);       // = addFirst
deque.pop();         // = removeFirst
deque.peek();        // = peekFirst

// Як QUEUE (FIFO)
deque.offer(x);      // = addLast
deque.poll();        // = removeFirst
deque.peek();        // = peekFirst

// Як DEQUE (обидва кінці)
deque.addFirst(x);   deque.addLast(x);
deque.removeFirst(); deque.removeLast();
deque.peekFirst();   deque.peekLast();
deque.offerFirst(x); deque.offerLast(x);
deque.pollFirst();   deque.pollLast();

// Розмір
deque.size();
deque.isEmpty();
```

---

## 📐 PriorityQueue — Heap

```java
// Min-Heap (дефолт)
PriorityQueue<Integer> minPQ = new PriorityQueue<>();

// Max-Heap
PriorityQueue<Integer> maxPQ = new PriorityQueue<>(Collections.reverseOrder());

// Кастомний компаратор
PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> {
    if (a[0] != b[0]) return a[0] - b[0]; // за першим елементом
    return a[1] - b[1];                    // потім за другим
});

// Операції
pq.offer(x);    // O(log n)
pq.poll();      // O(log n) — видалити і повернути мінімум
pq.peek();      // O(1) — подивитись мінімум
pq.size();
pq.contains(x); // O(n) — повільно!

// ⚠️ PriorityQueue НЕ підтримує:
// - decrease-key → видали і додай заново
// - швидкий contains → використовуй окремий Set
```

---

## 📐 Arrays утиліти

```java
// Сортування
Arrays.sort(arr);                              // примітиви: O(n log n) dual-pivot quicksort
Arrays.sort(arr, fromIdx, toIdx);              // діапазон [from, to)
Arrays.sort(objArr, comparator);               // об'єкти з comparator

// Пошук (тільки у відсортованому масиві)
int idx = Arrays.binarySearch(arr, target);    // < 0 якщо не знайдено

// Заповнення
Arrays.fill(arr, value);
Arrays.fill(arr, from, to, value);

// Копіювання
int[] copy = Arrays.copyOf(arr, newLength);
int[] slice = Arrays.copyOfRange(arr, from, to); // [from, to)

// Порівняння
Arrays.equals(arr1, arr2);
Arrays.deepEquals(matrix1, matrix2);  // для 2D

// Конвертація
Arrays.asList(arr);              // List (фіксований розмір!)
Arrays.stream(arr).boxed()...    // IntStream → Stream<Integer>
String.join(",", strArr);        // String[]  → "a,b,c"

// 2D масив
int[][] matrix = new int[rows][cols];
Arrays.stream(matrix).forEach(row -> Arrays.fill(row, -1));
```

---

## 📐 Collections утиліти

```java
// Сортування
Collections.sort(list);
Collections.sort(list, comparator);
Collections.sort(list, Collections.reverseOrder());
list.sort((a, b) -> a - b); // Java 8+

// Пошук
Collections.binarySearch(list, key);

// Min / Max
Collections.min(list);
Collections.max(list);

// Shuffle / Reverse / Fill
Collections.shuffle(list);
Collections.reverse(list);
Collections.fill(list, value);

// Частота
Collections.frequency(list, element);

// Незмінні колекції
List<T> immutable = Collections.unmodifiableList(list);
Map<K,V> immutable = Collections.unmodifiableMap(map);

// Singleton
List<T> single = Collections.singletonList(x);
Set<T> single = Collections.singleton(x);

// Empty
List<T> empty = Collections.emptyList();
```

---

## 📐 Stream API — корисне для інтерв'ю

```java
// Базові операції
int[] arr = {3, 1, 4, 1, 5};

Arrays.stream(arr).max().getAsInt();     // 5
Arrays.stream(arr).min().getAsInt();     // 1
Arrays.stream(arr).sum();                // 14
Arrays.stream(arr).average().getAsDouble();

// Фільтрація і збір
List<Integer> evens = list.stream()
    .filter(x -> x % 2 == 0)
    .collect(Collectors.toList());

// Mapping
List<String> strings = list.stream()
    .map(Object::toString)
    .collect(Collectors.toList());

// Sorted
list.stream().sorted().collect(Collectors.toList());
list.stream().sorted(Comparator.reverseOrder())...

// GroupBy
Map<Integer, List<String>> grouped = list.stream()
    .collect(Collectors.groupingBy(String::length));

// Joining
String joined = list.stream()
    .map(Object::toString)
    .collect(Collectors.joining(", "));

// int[] ↔ List<Integer>
int[] arr = list.stream().mapToInt(Integer::intValue).toArray();
List<Integer> list = Arrays.stream(arr).boxed().collect(Collectors.toList());
```

---

## 📐 Comparator — швидкий довідник

```java
// Один критерій
Comparator.comparingInt(x -> x[0])

// Декілька критеріїв
Comparator.comparingInt((int[] x) -> x[0])
          .thenComparingInt(x -> x[1])

// Reverse
Comparator.comparingInt(x -> x[0]).reversed()

// Рядки
Comparator.comparing(String::length)
Comparator.naturalOrder()  // лексикографічний
Comparator.reverseOrder()

// Null-safe
Comparator.nullsFirst(Comparator.naturalOrder())
Comparator.nullsLast(Comparator.naturalOrder())

// Приклади для Arrays.sort
Arrays.sort(intervals, (a, b) -> a[0] - b[0]);             // за початком
Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1])); // безпечно
Arrays.sort(words, (a, b) -> a.length() - b.length());     // за довжиною
Arrays.sort(words, Comparator.comparingInt(String::length)
                              .thenComparing(Comparator.naturalOrder()));
```

---

## 📐 Типові перетворення

```java
// int[] → List<Integer>
List<Integer> list = Arrays.stream(arr).boxed().collect(Collectors.toList());

// List<Integer> → int[]
int[] arr = list.stream().mapToInt(Integer::intValue).toArray();

// String → char[]
char[] chars = s.toCharArray();

// char[] → String
String s = new String(chars);
String s = String.valueOf(chars);

// int → String
String s = String.valueOf(num);
String s = Integer.toString(num);

// String → int
int n = Integer.parseInt(s);

// List → Set (видалення дублікатів)
Set<T> set = new HashSet<>(list);

// Set → List
List<T> list = new ArrayList<>(set);

// Map → sorted by value
map.entrySet().stream()
   .sorted(Map.Entry.comparingByValue())
   .collect(Collectors.toMap(
       Map.Entry::getKey,
       Map.Entry::getValue,
       (e1, e2) -> e1,
       LinkedHashMap::new
   ));
```

---

## 📐 Складність операцій — зведена таблиця

| Колекція | get | add | remove | contains | notes |
|---|---|---|---|---|---|
| ArrayList | O(1) | O(1)* | O(n) | O(n) | *амортизована |
| LinkedList | O(n) | O(1) | O(1)** | O(n) | **якщо є ref |
| ArrayDeque | O(n) | O(1) | O(1) ends | O(n) | стек/черга |
| HashMap | O(1) | O(1) | O(1) | O(1) | avg case |
| TreeMap | O(log n) | O(log n) | O(log n) | O(log n) | відсортований |
| HashSet | — | O(1) | O(1) | O(1) | avg case |
| TreeSet | — | O(log n) | O(log n) | O(log n) | відсортований |
| PriorityQueue | O(1) peek | O(log n) | O(log n) | O(n) | heap |

---

## 📐 Java-специфічні пастки на інтерв'ю

```java
// 1. Integer == vs .equals()
Integer a = 127, b = 127;
a == b;       // true (кешується -128..127)
Integer x = 200, y = 200;
x == y;       // false! Використовуй .equals()

// 2. int overflow
int a = Integer.MAX_VALUE;
a + 1;        // → Integer.MIN_VALUE (overflow!)
long result = (long) a + 1; // правильно

// 3. char арифметика
char c = 'a';
int idx = c - 'a';  // 0
c + 1;              // int, не char!
(char)(c + 1);      // 'b'

// 4. String immutability
String s = "hello";
s += " world";     // створює новий об'єкт!
// у циклі: завжди StringBuilder

// 5. Arrays.asList — фіксований розмір
List<Integer> list = Arrays.asList(1, 2, 3);
list.add(4);  // UnsupportedOperationException!
// Правильно:
List<Integer> list = new ArrayList<>(Arrays.asList(1, 2, 3));

// 6. Comparator overflow
(a, b) -> a - b   // небезпечно якщо a=-2^31, b=2^31-1
Integer.compare(a, b) // безпечно завжди

// 7. Modifying collection під час ітерації
for (int x : list) {
    list.remove(x); // ConcurrentModificationException!
}
// Правильно: Iterator або removeIf
list.removeIf(x -> x % 2 == 0);

// 8. null у HashMap
map.get("key") == null — або ключа немає, або значення null
// Перевіряй: map.containsKey("key")
```
