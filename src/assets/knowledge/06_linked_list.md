# 06. Linked List

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 3–4 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "middle of linked list" | Fast & Slow Pointers |
| "detect cycle", "has cycle" | Fast & Slow Pointers |
| "find cycle start" | Fast & Slow + math |
| "reverse linked list" | Iterative або Recursive Reverse |
| "merge two sorted lists" | Merge патерн |
| "remove nth from end" | Fast & Slow з gap |
| "reorder list", "rotate list" | Комбінація: find middle + reverse + merge |
| "palindrome linked list" | Find middle + reverse half |
| "LRU Cache" | HashMap + Doubly Linked List |

---

## 📐 Базова структура Node

```java
public class ListNode {
    int val;
    ListNode next;
    ListNode(int val) { this.val = val; }
}
```

---

## 📐 Патерн 1: Fast & Slow Pointers

### Ключова думка
- `slow` рухається на 1 крок, `fast` — на 2
- Коли `fast` досягає кінця → `slow` на середині
- Якщо є цикл → `fast` і `slow` зустрінуться

### Задача 1: Middle of the Linked List (LeetCode #876)

```java
public ListNode middleNode(ListNode head) {
    ListNode slow = head, fast = head;

    while (fast != null && fast.next != null) {
        slow = slow.next;
        fast = fast.next.next;
    }

    return slow; // slow вказує на середину
}
```

> 💡 Для **парної** довжини (1→2→3→4) slow зупиниться на **другій** середині (node 3).  
> Якщо потрібна перша середина — зупиняємось коли `fast.next == null || fast.next.next == null`.

**Складність:** O(n) time, O(1) space

---

### Задача 2: Linked List Cycle (LeetCode #141)

```java
public boolean hasCycle(ListNode head) {
    ListNode slow = head, fast = head;

    while (fast != null && fast.next != null) {
        slow = slow.next;
        fast = fast.next.next;

        if (slow == fast) return true; // зустрілись → є цикл
    }

    return false;
}
```

---

### Задача 3: Linked List Cycle II — знайти початок циклу (LeetCode #142)

**Математика за патерном:**
```
Нехай:
  F = відстань від head до початку циклу
  C = довжина циклу
  a = відстань від початку циклу до точки зустрічі

Коли зустрілись:
  slow пройшов: F + a
  fast пройшов: F + a + C (або кілька циклів, але спростимо)
  fast = 2 * slow → F + a + C = 2(F + a) → C = F + a → F = C - a

Висновок: якщо один pointer рухається з head,
а інший з точки зустрічі — вони зустрінуться на початку циклу!
```

```java
public ListNode detectCycle(ListNode head) {
    ListNode slow = head, fast = head;

    // Крок 1: знайти точку зустрічі
    while (fast != null && fast.next != null) {
        slow = slow.next;
        fast = fast.next.next;
        if (slow == fast) break;
    }

    // немає циклу
    if (fast == null || fast.next == null) return null;

    // Крок 2: знайти початок циклу
    slow = head;
    while (slow != fast) {
        slow = slow.next;
        fast = fast.next; // обидва по одному кроку
    }

    return slow;
}
```
**Складність:** O(n) time, O(1) space

---

### Задача 4: Remove Nth Node From End (LeetCode #19)
**Ключова думка:** Два pointer з gap = n. Коли fast досягає кінця → slow вказує на вузол **перед** тим що треба видалити.

```java
public ListNode removeNthFromEnd(ListNode head, int n) {
    ListNode dummy = new ListNode(0);
    dummy.next = head;

    ListNode fast = dummy, slow = dummy;

    // fast робить n+1 кроків вперед
    for (int i = 0; i <= n; i++) fast = fast.next;

    // рухаємо обидва до кінця
    while (fast != null) {
        slow = slow.next;
        fast = fast.next;
    }

    // slow.next — вузол що треба видалити
    slow.next = slow.next.next;

    return dummy.next;
}
```

> 💡 **Dummy node** — стандартний прийом для linked list задач.  
> Спрощує edge cases: видалення head, порожній список.

---

## 📐 Патерн 2: Reverse Linked List

### Ітеративний підхід (рекомендований)

```java
public ListNode reverse(ListNode head) {
    ListNode prev = null;
    ListNode curr = head;

    while (curr != null) {
        ListNode next = curr.next; // зберігаємо наступний
        curr.next = prev;          // розвертаємо pointer
        prev = curr;               // рухаємо prev
        curr = next;               // рухаємо curr
    }

    return prev; // новий head
}
```

### Рекурсивний підхід

```java
public ListNode reverseList(ListNode head) {
    // base case
    if (head == null || head.next == null) return head;

    ListNode newHead = reverseList(head.next); // розвертаємо хвіст
    head.next.next = head; // наступний вузол вказує назад на head
    head.next = null;      // head стає хвостом

    return newHead;
}
```

### Задача 5: Reverse Linked List (LeetCode #206)
```java
public ListNode reverseList(ListNode head) {
    ListNode prev = null, curr = head;
    while (curr != null) {
        ListNode next = curr.next;
        curr.next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}
```

---

### Задача 6: Reverse Linked List II — reverse частини (LeetCode #92)
**Умова:** Reverse від позиції left до right включно.

```java
public ListNode reverseBetween(ListNode head, int left, int right) {
    ListNode dummy = new ListNode(0);
    dummy.next = head;
    ListNode prev = dummy;

    // Крок 1: дійти до вузла перед left
    for (int i = 0; i < left - 1; i++) prev = prev.next;

    // Крок 2: reverse (right - left) разів
    ListNode curr = prev.next;
    for (int i = 0; i < right - left; i++) {
        ListNode next = curr.next;
        curr.next = next.next;
        next.next = prev.next;
        prev.next = next;
    }

    return dummy.next;
}
```

> 💡 Це "insertion at front" трюк — кожен наступний вузол вставляємо одразу після `prev`.

---

### Задача 7: Palindrome Linked List (LeetCode #234)
**Підхід:** Find middle → Reverse second half → Compare

```java
public boolean isPalindrome(ListNode head) {
    // Крок 1: знайти середину
    ListNode slow = head, fast = head;
    while (fast != null && fast.next != null) {
        slow = slow.next;
        fast = fast.next.next;
    }

    // Крок 2: reverse другої половини
    ListNode secondHalf = reverse(slow);
    ListNode copy = secondHalf; // зберігаємо для відновлення

    // Крок 3: порівняти
    ListNode first = head;
    boolean result = true;
    while (secondHalf != null) {
        if (first.val != secondHalf.val) {
            result = false;
            break;
        }
        first = first.next;
        secondHalf = secondHalf.next;
    }

    // Крок 4: відновити список (good practice)
    reverse(copy);

    return result;
}

private ListNode reverse(ListNode head) {
    ListNode prev = null, curr = head;
    while (curr != null) {
        ListNode next = curr.next;
        curr.next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}
```
**Складність:** O(n) time, O(1) space

---

## 📐 Патерн 3: Merge

### Задача 8: Merge Two Sorted Lists (LeetCode #21)

```java
public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
    ListNode dummy = new ListNode(0);
    ListNode curr = dummy;

    while (l1 != null && l2 != null) {
        if (l1.val <= l2.val) {
            curr.next = l1;
            l1 = l1.next;
        } else {
            curr.next = l2;
            l2 = l2.next;
        }
        curr = curr.next;
    }

    // приєднуємо залишок
    curr.next = (l1 != null) ? l1 : l2;

    return dummy.next;
}
```
**Складність:** O(n + m) time, O(1) space

---

### Задача 9: Merge K Sorted Lists (LeetCode #23)
**Підхід 1 — Min Heap (Priority Queue):**

```java
public ListNode mergeKLists(ListNode[] lists) {
    PriorityQueue<ListNode> minHeap = new PriorityQueue<>(
        (a, b) -> a.val - b.val
    );

    // Додаємо head кожного списку
    for (ListNode node : lists) {
        if (node != null) minHeap.offer(node);
    }

    ListNode dummy = new ListNode(0);
    ListNode curr = dummy;

    while (!minHeap.isEmpty()) {
        ListNode node = minHeap.poll(); // найменший
        curr.next = node;
        curr = curr.next;

        if (node.next != null) minHeap.offer(node.next); // наступний з того ж списку
    }

    return dummy.next;
}
```
**Складність:** O(n log k) time де n — загальна кількість вузлів, k — кількість списків

**Підхід 2 — Divide & Conquer (елегантний):**

```java
public ListNode mergeKLists(ListNode[] lists) {
    if (lists.length == 0) return null;
    return mergeRange(lists, 0, lists.length - 1);
}

private ListNode mergeRange(ListNode[] lists, int left, int right) {
    if (left == right) return lists[left];

    int mid = left + (right - left) / 2;
    ListNode l1 = mergeRange(lists, left, mid);
    ListNode l2 = mergeRange(lists, mid + 1, right);
    return mergeTwoLists(l1, l2);
}
```

---

## 📐 Патерн 4: Комбінований (Find Middle + Reverse + Merge)

### Задача 10: Reorder List (LeetCode #143)
**Умова:** `L0→L1→...→Ln` перетворити на `L0→Ln→L1→Ln-1→...`

**Три кроки:**
1. Знайти середину
2. Reverse другу половину
3. Merge дві половини по черзі

```java
public void reorderList(ListNode head) {
    if (head == null || head.next == null) return;

    // Крок 1: знайти середину
    ListNode slow = head, fast = head;
    while (fast.next != null && fast.next.next != null) {
        slow = slow.next;
        fast = fast.next.next;
    }

    // Крок 2: reverse другої половини
    ListNode second = reverse(slow.next);
    slow.next = null; // розрізаємо список

    // Крок 3: merge по черзі
    ListNode first = head;
    while (second != null) {
        ListNode tmp1 = first.next;
        ListNode tmp2 = second.next;

        first.next = second;
        second.next = tmp1;

        first = tmp1;
        second = tmp2;
    }
}

private ListNode reverse(ListNode head) {
    ListNode prev = null, curr = head;
    while (curr != null) {
        ListNode next = curr.next;
        curr.next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}
```
**Складність:** O(n) time, O(1) space

---

## 📐 Патерн 5: LRU Cache

### Задача 11: LRU Cache (LeetCode #146)
**Умова:** Реалізувати LRU Cache з `get` і `put` за O(1).

**Ключова думка:**
- `HashMap` → O(1) доступ за ключем
- `Doubly Linked List` → O(1) видалення/вставка будь-якого вузла
- Найновіший — біля `head`, найстаріший — біля `tail`

```java
class LRUCache {
    // Doubly Linked List Node
    class Node {
        int key, val;
        Node prev, next;
        Node(int key, int val) {
            this.key = key;
            this.val = val;
        }
    }

    private final int capacity;
    private final Map<Integer, Node> map;
    private final Node head, tail; // dummy nodes

    public LRUCache(int capacity) {
        this.capacity = capacity;
        this.map = new HashMap<>();

        // dummy head і tail спрощують операції
        head = new Node(0, 0);
        tail = new Node(0, 0);
        head.next = tail;
        tail.prev = head;
    }

    public int get(int key) {
        if (!map.containsKey(key)) return -1;

        Node node = map.get(key);
        moveToFront(node); // recently used → front
        return node.val;
    }

    public void put(int key, int value) {
        if (map.containsKey(key)) {
            Node node = map.get(key);
            node.val = value;
            moveToFront(node);
        } else {
            Node node = new Node(key, value);
            map.put(key, node);
            addToFront(node);

            if (map.size() > capacity) {
                Node lru = removeLast(); // видаляємо найстаріший
                map.remove(lru.key);
            }
        }
    }

    private void addToFront(Node node) {
        node.next = head.next;
        node.prev = head;
        head.next.prev = node;
        head.next = node;
    }

    private void remove(Node node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    private void moveToFront(Node node) {
        remove(node);
        addToFront(node);
    }

    private Node removeLast() {
        Node lru = tail.prev;
        remove(lru);
        return lru;
    }
}
```
**Складність:** O(1) get і put

---

## 📐 Патерн 6: Додаткові корисні задачі

### Задача 12: Add Two Numbers (LeetCode #2)
**Умова:** Числа зберігаються у reversed linked list. Знайти суму.

```java
public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
    ListNode dummy = new ListNode(0);
    ListNode curr = dummy;
    int carry = 0;

    while (l1 != null || l2 != null || carry != 0) {
        int sum = carry;
        if (l1 != null) { sum += l1.val; l1 = l1.next; }
        if (l2 != null) { sum += l2.val; l2 = l2.next; }

        carry = sum / 10;
        curr.next = new ListNode(sum % 10);
        curr = curr.next;
    }

    return dummy.next;
}
```

---

### Задача 13: Copy List with Random Pointer (LeetCode #138)
**Умова:** Linked list з `next` і `random` pointer. Зробити deep copy.

```java
public Node copyRandomList(Node head) {
    if (head == null) return null;

    Map<Node, Node> map = new HashMap<>(); // original → copy

    // Прохід 1: створюємо копії всіх вузлів
    Node curr = head;
    while (curr != null) {
        map.put(curr, new Node(curr.val));
        curr = curr.next;
    }

    // Прохід 2: встановлюємо next і random
    curr = head;
    while (curr != null) {
        map.get(curr).next = map.get(curr.next);
        map.get(curr).random = map.get(curr.random);
        curr = curr.next;
    }

    return map.get(head);
}
```
**Складність:** O(n) time, O(n) space

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Linked List
│
├── "middle" / "cycle" / "nth from end"?
│   └── FAST & SLOW POINTERS
│       ├── middle → slow зупиняється на середині
│       ├── cycle detect → зустріч = цикл
│       ├── cycle start → math trick (F = C - a)
│       └── nth from end → gap = n між fast і slow
│
├── "reverse" частини або всього списку?
│   └── ITERATIVE REVERSE (prev/curr/next)
│       └── partial reverse → "insertion at front" трюк
│
├── "merge" відсортованих списків?
│   ├── 2 списки → dummy + порівняння
│   └── k списків → Min Heap або Divide & Conquer
│
├── "reorder" / "palindrome"?
│   └── КОМБІНОВАНИЙ: find middle + reverse + merge/compare
│
├── "O(1) get/put cache"?
│   └── LRU: HashMap + Doubly Linked List
│
└── "copy" / "deep clone"?
    └── HashMap: original → copy (два проходи)
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| Не використовувати dummy node | Завжди додавай dummy для спрощення edge cases |
| Забути зберегти `curr.next` перед зміною pointer | `ListNode next = curr.next;` перший рядок у reverse |
| `fast.next.next` без перевірки `fast.next != null` | `while (fast != null && fast.next != null)` |
| Не розрізати список після find middle у Reorder | `slow.next = null` після знаходження середини |
| У LRU: не оновлювати map при евікції | `map.remove(lru.key)` після `removeLast()` |
| Порівнювати вузли через `==` замість `.val` | `node1.val == node2.val` для порівняння значень |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #206 Reverse Linked List
- [ ] #21 Merge Two Sorted Lists
- [ ] #141 Linked List Cycle
- [ ] #876 Middle of the Linked List
- [ ] #19 Remove Nth Node From End
- [ ] #234 Palindrome Linked List

### Should Solve (Middle)
- [ ] #142 Linked List Cycle II
- [ ] #92 Reverse Linked List II
- [ ] #143 Reorder List
- [ ] #23 Merge K Sorted Lists
- [ ] #146 LRU Cache
- [ ] #2 Add Two Numbers
- [ ] #138 Copy List with Random Pointer

### Stretch Goals
- [ ] #25 Reverse Nodes in k-Group
- [ ] #432 All O`one Data Structure
- [ ] #460 LFU Cache

---

## 🔑 Quick Reference: Linked List прийоми

```java
// Dummy node — завжди використовуй при модифікації head
ListNode dummy = new ListNode(0);
dummy.next = head;
// ... операції ...
return dummy.next;

// Fast & Slow — умова циклу
while (fast != null && fast.next != null) {
    slow = slow.next;
    fast = fast.next.next;
}

// Reverse — три змінні
ListNode prev = null, curr = head;
while (curr != null) {
    ListNode next = curr.next;
    curr.next = prev;
    prev = curr;
    curr = next;
}
return prev; // новий head

// Gap між двома pointers (nth from end)
for (int i = 0; i <= n; i++) fast = fast.next; // gap = n+1 від dummy
while (fast != null) { slow = slow.next; fast = fast.next; }
slow.next = slow.next.next; // видалення

// Перевірка довжини (якщо треба знати)
int length = 0;
ListNode curr = head;
while (curr != null) { length++; curr = curr.next; }
```
