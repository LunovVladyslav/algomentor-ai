# 08. Trees — DFS & BFS

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 4–5 днів

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "max depth", "min depth", "diameter" | DFS (recursion) |
| "path sum", "root to leaf" | DFS (recursion з accumulator) |
| "level order", "zigzag", "right side view" | BFS (Queue) |
| "validate BST", "kth smallest in BST" | DFS Inorder (BST property) |
| "lowest common ancestor" | DFS (post-order) |
| "serialize / deserialize" | BFS або DFS |
| "symmetric", "same tree", "invert" | DFS (структурне порівняння) |
| "count nodes", "complete binary tree" | DFS або Binary Search |

---

## 📐 Базова структура TreeNode

```java
public class TreeNode {
    int val;
    TreeNode left, right;
    TreeNode(int val) { this.val = val; }
}
```

---

## 📐 Патерн 1: DFS — базові рекурсивні шаблони

### Три порядки обходу

```java
// Pre-order: root → left → right (корінь першим)
void preOrder(TreeNode root) {
    if (root == null) return;
    process(root);          // ← тут обробка
    preOrder(root.left);
    preOrder(root.right);
}

// In-order: left → root → right (для BST дає відсортований порядок)
void inOrder(TreeNode root) {
    if (root == null) return;
    inOrder(root.left);
    process(root);          // ← тут обробка
    inOrder(root.right);
}

// Post-order: left → right → root (спочатку діти, потім батько)
void postOrder(TreeNode root) {
    if (root == null) return;
    postOrder(root.left);
    postOrder(root.right);
    process(root);          // ← тут обробка
}
```

> 💡 **Як вибрати порядок:**
> - Pre-order → коли потрібен **батько перед дітьми** (копіювання, серіалізація)
> - In-order → **BST задачі** (дає відсортований порядок)
> - Post-order → коли потрібні **результати дітей для батька** (висота, діаметр, LCA)

---

### Задача 1: Maximum Depth of Binary Tree (LeetCode #104)

```java
public int maxDepth(TreeNode root) {
    if (root == null) return 0;

    int leftDepth = maxDepth(root.left);
    int rightDepth = maxDepth(root.right);

    return Math.max(leftDepth, rightDepth) + 1;
}
```
**Складність:** O(n) time, O(h) space де h — висота дерева

---

### Задача 2: Invert Binary Tree (LeetCode #226)

```java
public TreeNode invertTree(TreeNode root) {
    if (root == null) return null;

    // спочатку інвертуємо піддерева
    TreeNode left = invertTree(root.left);
    TreeNode right = invertTree(root.right);

    // міняємо місцями
    root.left = right;
    root.right = left;

    return root;
}
```

---

### Задача 3: Symmetric Tree (LeetCode #101)

```java
public boolean isSymmetric(TreeNode root) {
    return isMirror(root.left, root.right);
}

private boolean isMirror(TreeNode left, TreeNode right) {
    if (left == null && right == null) return true;
    if (left == null || right == null) return false;

    return left.val == right.val
        && isMirror(left.left, right.right)   // зовнішні
        && isMirror(left.right, right.left);  // внутрішні
}
```

---

### Задача 4: Diameter of Binary Tree (LeetCode #543)
**Умова:** Найдовший шлях між будь-якими двома вузлами (не обов'язково через корінь).

**Ключова думка:** Діаметр через вузол = висота лівого + висота правого піддерева.  
Оновлюємо глобальний максимум під час обчислення висоти (post-order).

```java
private int diameter = 0;

public int diameterOfBinaryTree(TreeNode root) {
    height(root);
    return diameter;
}

private int height(TreeNode node) {
    if (node == null) return 0;

    int leftH = height(node.left);
    int rightH = height(node.right);

    // оновлюємо діаметр через поточний вузол
    diameter = Math.max(diameter, leftH + rightH);

    return Math.max(leftH, rightH) + 1;
}
```

> ⚠️ Типова помилка: повертати `diameter` з рекурсивного методу напряму.  
> Правильно: рекурсія повертає **висоту**, діаметр оновлюємо як side effect.

---

## 📐 Патерн 2: DFS — Path Sum задачі

### Шаблон — пошук шляху root-to-leaf

```java
// Передаємо залишок суми вниз (top-down)
boolean hasPathSum(TreeNode node, int remaining) {
    if (node == null) return false;

    remaining -= node.val;

    // листок: перевіряємо чи вичерпали суму
    if (node.left == null && node.right == null) return remaining == 0;

    return hasPathSum(node.left, remaining)
        || hasPathSum(node.right, remaining);
}
```

### Задача 5: Path Sum (LeetCode #112)

```java
public boolean hasPathSum(TreeNode root, int targetSum) {
    if (root == null) return false;

    targetSum -= root.val;

    if (root.left == null && root.right == null) return targetSum == 0;

    return hasPathSum(root.left, targetSum)
        || hasPathSum(root.right, targetSum);
}
```

---

### Задача 6: Path Sum II — всі шляхи (LeetCode #113)

```java
public List<List<Integer>> pathSum(TreeNode root, int targetSum) {
    List<List<Integer>> result = new ArrayList<>();
    dfs(root, targetSum, new ArrayList<>(), result);
    return result;
}

private void dfs(TreeNode node, int remaining,
                 List<Integer> path, List<List<Integer>> result) {
    if (node == null) return;

    path.add(node.val);
    remaining -= node.val;

    if (node.left == null && node.right == null && remaining == 0) {
        result.add(new ArrayList<>(path)); // копія!
    }

    dfs(node.left, remaining, path, result);
    dfs(node.right, remaining, path, result);

    path.remove(path.size() - 1); // backtrack
}
```

> 💡 `new ArrayList<>(path)` — обов'язкова копія, інакше всі результати будуть посилатися на один список.

---

### Задача 7: Binary Tree Maximum Path Sum (LeetCode #124) ⭐
**Умова:** Максимальна сума шляху між будь-якими двома вузлами.

**Ключова думка:** Схожа на Diameter — рахуємо "gain" від кожного піддерева.  
Якщо gain від'ємний — ігноруємо (`Math.max(gain, 0)`).

```java
private int maxSum = Integer.MIN_VALUE;

public int maxPathSum(TreeNode root) {
    gain(root);
    return maxSum;
}

private int gain(TreeNode node) {
    if (node == null) return 0;

    // якщо gain від'ємний → краще не включати це піддерево
    int leftGain = Math.max(gain(node.left), 0);
    int rightGain = Math.max(gain(node.right), 0);

    // шлях через поточний вузол
    maxSum = Math.max(maxSum, node.val + leftGain + rightGain);

    // повертаємо тільки одну гілку (шлях не може розгалужуватись)
    return node.val + Math.max(leftGain, rightGain);
}
```

---

## 📐 Патерн 3: BFS — Level Order Traversal

### Шаблон

```java
Queue<TreeNode> queue = new ArrayDeque<>();
queue.offer(root);

while (!queue.isEmpty()) {
    int levelSize = queue.size(); // кількість вузлів на поточному рівні

    for (int i = 0; i < levelSize; i++) {
        TreeNode node = queue.poll();
        // обробка node

        if (node.left != null) queue.offer(node.left);
        if (node.right != null) queue.offer(node.right);
    }
    // тут: закінчили рівень
}
```

### Задача 8: Binary Tree Level Order Traversal (LeetCode #102)

```java
public List<List<Integer>> levelOrder(TreeNode root) {
    List<List<Integer>> result = new ArrayList<>();
    if (root == null) return result;

    Queue<TreeNode> queue = new ArrayDeque<>();
    queue.offer(root);

    while (!queue.isEmpty()) {
        int levelSize = queue.size();
        List<Integer> level = new ArrayList<>();

        for (int i = 0; i < levelSize; i++) {
            TreeNode node = queue.poll();
            level.add(node.val);

            if (node.left != null) queue.offer(node.left);
            if (node.right != null) queue.offer(node.right);
        }

        result.add(level);
    }

    return result;
}
```
**Складність:** O(n) time, O(n) space

---

### Задача 9: Binary Tree Right Side View (LeetCode #199)
**Ключова думка:** Останній елемент кожного рівня = правий вид.

```java
public List<Integer> rightSideView(TreeNode root) {
    List<Integer> result = new ArrayList<>();
    if (root == null) return result;

    Queue<TreeNode> queue = new ArrayDeque<>();
    queue.offer(root);

    while (!queue.isEmpty()) {
        int levelSize = queue.size();

        for (int i = 0; i < levelSize; i++) {
            TreeNode node = queue.poll();

            if (i == levelSize - 1) result.add(node.val); // останній на рівні

            if (node.left != null) queue.offer(node.left);
            if (node.right != null) queue.offer(node.right);
        }
    }

    return result;
}
```

---

### Задача 10: Binary Tree Zigzag Level Order (LeetCode #103)

```java
public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
    List<List<Integer>> result = new ArrayList<>();
    if (root == null) return result;

    Queue<TreeNode> queue = new ArrayDeque<>();
    queue.offer(root);
    boolean leftToRight = true;

    while (!queue.isEmpty()) {
        int levelSize = queue.size();
        LinkedList<Integer> level = new LinkedList<>();

        for (int i = 0; i < levelSize; i++) {
            TreeNode node = queue.poll();

            // додаємо в початок або кінець залежно від напрямку
            if (leftToRight) level.addLast(node.val);
            else level.addFirst(node.val);

            if (node.left != null) queue.offer(node.left);
            if (node.right != null) queue.offer(node.right);
        }

        result.add(level);
        leftToRight = !leftToRight;
    }

    return result;
}
```

---

## 📐 Патерн 4: BST задачі

### Властивість BST
```
Для кожного вузла:
  всі вузли лівого піддерева < node.val
  всі вузли правого піддерева > node.val
  In-order обхід → відсортований масив
```

### Задача 11: Validate Binary Search Tree (LeetCode #98)
**Ключова думка:** Передаємо допустимий діапазон `[min, max]` зверху вниз.

```java
public boolean isValidBST(TreeNode root) {
    return validate(root, Long.MIN_VALUE, Long.MAX_VALUE);
}

private boolean validate(TreeNode node, long min, long max) {
    if (node == null) return true;

    if (node.val <= min || node.val >= max) return false;

    return validate(node.left, min, node.val)   // ліве піддерево: max = node.val
        && validate(node.right, node.val, max); // праве піддерево: min = node.val
}
```

> ⚠️ Використовуй `Long.MIN_VALUE / Long.MAX_VALUE` — значення вузлів можуть бути `Integer.MIN/MAX_VALUE`.

---

### Задача 12: Kth Smallest Element in BST (LeetCode #230)
**Ключова думка:** In-order BST = відсортований порядок. k-й елемент in-order = відповідь.

```java
private int count = 0;
private int result = 0;

public int kthSmallest(TreeNode root, int k) {
    inOrder(root, k);
    return result;
}

private void inOrder(TreeNode node, int k) {
    if (node == null) return;

    inOrder(node.left, k);

    count++;
    if (count == k) {
        result = node.val;
        return;
    }

    inOrder(node.right, k);
}
```

**Ітеративний варіант (частіше питають на інтерв'ю):**

```java
public int kthSmallest(TreeNode root, int k) {
    Deque<TreeNode> stack = new ArrayDeque<>();
    TreeNode curr = root;

    while (curr != null || !stack.isEmpty()) {
        while (curr != null) {
            stack.push(curr);
            curr = curr.left;
        }

        curr = stack.pop();
        k--;
        if (k == 0) return curr.val;

        curr = curr.right;
    }

    return -1;
}
```

---

### Задача 13: Lowest Common Ancestor of BST (LeetCode #235)
**Ключова думка:** Якщо обидва вузли менші за root → LCA у лівому піддереві. Якщо більші → у правому. Інакше → root і є LCA.

```java
public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
    if (p.val < root.val && q.val < root.val) {
        return lowestCommonAncestor(root.left, p, q);
    }
    if (p.val > root.val && q.val > root.val) {
        return lowestCommonAncestor(root.right, p, q);
    }
    return root; // root між p і q (або рівний одному з них)
}
```

---

## 📐 Патерн 5: LCA для звичайного дерева

### Задача 14: Lowest Common Ancestor of Binary Tree (LeetCode #236)
**Ключова думка:** Post-order — якщо знайшли p або q повертаємо їх вгору. Якщо обидва піддерева повернули не null → поточний вузол і є LCA.

```java
public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
    // base case: не знайшли або знайшли один з вузлів
    if (root == null || root == p || root == q) return root;

    TreeNode left = lowestCommonAncestor(root.left, p, q);
    TreeNode right = lowestCommonAncestor(root.right, p, q);

    // p і q з різних сторін → root є LCA
    if (left != null && right != null) return root;

    // обидва в одному піддереві
    return left != null ? left : right;
}
```

---

## 📐 Патерн 6: Serialize / Deserialize

### Задача 15: Serialize and Deserialize Binary Tree (LeetCode #297)

```java
public class Codec {
    private static final String NULL = "N";
    private static final String SEP = ",";

    // Pre-order serialize
    public String serialize(TreeNode root) {
        StringBuilder sb = new StringBuilder();
        serializeHelper(root, sb);
        return sb.toString();
    }

    private void serializeHelper(TreeNode node, StringBuilder sb) {
        if (node == null) {
            sb.append(NULL).append(SEP);
            return;
        }
        sb.append(node.val).append(SEP);
        serializeHelper(node.left, sb);
        serializeHelper(node.right, sb);
    }

    // Pre-order deserialize
    public TreeNode deserialize(String data) {
        Deque<String> queue = new ArrayDeque<>(Arrays.asList(data.split(SEP)));
        return deserializeHelper(queue);
    }

    private TreeNode deserializeHelper(Deque<String> queue) {
        String val = queue.poll();
        if (NULL.equals(val)) return null;

        TreeNode node = new TreeNode(Integer.parseInt(val));
        node.left = deserializeHelper(queue);
        node.right = deserializeHelper(queue);
        return node;
    }
}
```

---

## 📐 Патерн 7: Iterative DFS зі Stack

### Коли використовувати
- Глибоке дерево → stack overflow при рекурсії
- Інтерв'юер явно просить iterative

```java
// Iterative Pre-order
public List<Integer> preorderTraversal(TreeNode root) {
    List<Integer> result = new ArrayList<>();
    if (root == null) return result;

    Deque<TreeNode> stack = new ArrayDeque<>();
    stack.push(root);

    while (!stack.isEmpty()) {
        TreeNode node = stack.pop();
        result.add(node.val);

        // правий першим — бо стек (LIFO), лівий обробиться раніше
        if (node.right != null) stack.push(node.right);
        if (node.left != null) stack.push(node.left);
    }

    return result;
}

// Iterative In-order (важливий для BST!)
public List<Integer> inorderTraversal(TreeNode root) {
    List<Integer> result = new ArrayList<>();
    Deque<TreeNode> stack = new ArrayDeque<>();
    TreeNode curr = root;

    while (curr != null || !stack.isEmpty()) {
        // йдемо максимально вліво
        while (curr != null) {
            stack.push(curr);
            curr = curr.left;
        }
        // обробляємо вузол
        curr = stack.pop();
        result.add(curr.val);
        // переходимо до правого піддерева
        curr = curr.right;
    }

    return result;
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Tree
│
├── Потрібна інформація від дітей до батька?
│   └── POST-ORDER DFS
│       (висота, діаметр, LCA, max path sum)
│
├── Потрібно передати інформацію від батька до дітей?
│   └── PRE-ORDER DFS
│       (validate BST з діапазоном, path sum зверху вниз)
│
├── BST задача?
│   └── IN-ORDER DFS (дає відсортований порядок)
│       (kth smallest, validate, successor)
│
├── "Level order" / "right side view" / "zigzag"?
│   └── BFS з Queue (+ levelSize trick)
│
├── "Serialize" / "clone"?
│   └── PRE-ORDER DFS (корінь перший → легко десеріалізувати)
│
└── Глибоке дерево або явно просять iterative?
    └── ITERATIVE DFS зі Stack
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| Повертати діаметр/maxSum з рекурсії | Рекурсія повертає висоту/gain, результат — instance variable або int[] |
| Не робити копію path у Path Sum II | `new ArrayList<>(path)` при додаванні до result |
| `Integer.MIN_VALUE` у Validate BST | `Long.MIN_VALUE / Long.MAX_VALUE` — уникаємо overflow |
| Ігнорувати від'ємні gain у Max Path Sum | `Math.max(gain(node), 0)` — не включаємо від'ємні гілки |
| BFS без `levelSize` — не відокремлюємо рівні | `int levelSize = queue.size()` перед inner loop |
| In-order iterative: забути `curr = curr.right` | Після `stack.pop()` переходимо до правого піддерева |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #104 Maximum Depth of Binary Tree
- [ ] #226 Invert Binary Tree
- [ ] #101 Symmetric Tree
- [ ] #112 Path Sum
- [ ] #102 Binary Tree Level Order Traversal
- [ ] #199 Binary Tree Right Side View
- [ ] #98 Validate Binary Search Tree
- [ ] #235 Lowest Common Ancestor of BST

### Should Solve (Middle)
- [ ] #543 Diameter of Binary Tree
- [ ] #124 Binary Tree Maximum Path Sum
- [ ] #113 Path Sum II
- [ ] #103 Binary Tree Zigzag Level Order
- [ ] #230 Kth Smallest Element in BST
- [ ] #236 Lowest Common Ancestor of Binary Tree
- [ ] #297 Serialize and Deserialize Binary Tree

### Stretch Goals
- [ ] #105 Construct Binary Tree from Preorder and Inorder
- [ ] #114 Flatten Binary Tree to Linked List
- [ ] #987 Vertical Order Traversal
- [ ] #1448 Count Good Nodes in Binary Tree

---

## 🔑 Quick Reference: Tree операції

```java
// Перевірка листка
boolean isLeaf = node.left == null && node.right == null;

// Висота дерева
int height(TreeNode node) {
    if (node == null) return 0;
    return Math.max(height(node.left), height(node.right)) + 1;
}

// Кількість вузлів
int count(TreeNode node) {
    if (node == null) return 0;
    return count(node.left) + count(node.right) + 1;
}

// BFS шаблон з рівнями
Queue<TreeNode> q = new ArrayDeque<>();
q.offer(root);
while (!q.isEmpty()) {
    int size = q.size(); // ← ключовий рядок для роботи з рівнями
    for (int i = 0; i < size; i++) {
        TreeNode node = q.poll();
        if (node.left != null) q.offer(node.left);
        if (node.right != null) q.offer(node.right);
    }
}

// Iterative In-order (BST)
Deque<TreeNode> stack = new ArrayDeque<>();
TreeNode curr = root;
while (curr != null || !stack.isEmpty()) {
    while (curr != null) { stack.push(curr); curr = curr.left; }
    curr = stack.pop();
    // → обробка curr
    curr = curr.right;
}

// Глобальний результат через масив (уникаємо instance variable)
int[] maxSum = {Integer.MIN_VALUE};
// використовуємо maxSum[0] всередині лямбди або вкладеного методу
```
