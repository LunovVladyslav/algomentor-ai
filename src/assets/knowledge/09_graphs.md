# 09. Graphs — DFS, BFS, Union-Find, Topological Sort

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 4–5 днів

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "number of islands", "connected components" | DFS / BFS / Union-Find |
| "shortest path", "minimum steps" | BFS (невзважений граф) |
| "can reach", "path exists" | DFS або BFS |
| "course schedule", "prerequisites" | Topological Sort |
| "cycle detection" | DFS з кольорами або Union-Find |
| "clone graph" | DFS / BFS + HashMap |
| "word ladder", "minimum transformation" | BFS |
| "number of provinces", "friend circles" | Union-Find або DFS |
| "walls and gates", "rotting oranges" | Multi-source BFS |

---

## 📐 Представлення графа в Java

```java
// Adjacency List — найпоширеніший спосіб
Map<Integer, List<Integer>> graph = new HashMap<>();

// Побудова з edges[][]
for (int[] edge : edges) {
    graph.computeIfAbsent(edge[0], k -> new ArrayList<>()).add(edge[1]);
    graph.computeIfAbsent(edge[1], k -> new ArrayList<>()).add(edge[0]); // undirected
}

// Grid як граф — 4 напрямки
int[][] directions = {{0,1},{0,-1},{1,0},{-1,0}};
for (int[] dir : directions) {
    int newRow = row + dir[0];
    int newCol = col + dir[1];
    if (newRow >= 0 && newRow < rows && newCol >= 0 && newCol < cols) {
        // сусідня клітинка валідна
    }
}
```

---

## 📐 Патерн 1: DFS на графі

### Ключова відмінність від дерева
У графі можуть бути **цикли** → потрібен `visited` set або масив.

### Шаблон

```java
Set<Integer> visited = new HashSet<>();

void dfs(Map<Integer, List<Integer>> graph, int node) {
    visited.add(node);
    // обробка node

    for (int neighbor : graph.getOrDefault(node, new ArrayList<>())) {
        if (!visited.contains(neighbor)) {
            dfs(graph, neighbor);
        }
    }
}
```

### Задача 1: Number of Islands (LeetCode #200)
**Умова:** Grid з '1' (земля) і '0' (вода). Знайти кількість островів.

```java
public int numIslands(char[][] grid) {
    int count = 0;
    int rows = grid.length, cols = grid[0].length;

    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            if (grid[r][c] == '1') {
                count++;
                dfs(grid, r, c); // "затопити" острів
            }
        }
    }

    return count;
}

private void dfs(char[][] grid, int r, int c) {
    // вихід за межі або вода → стоп
    if (r < 0 || r >= grid.length || c < 0 || c >= grid[0].length
            || grid[r][c] != '1') return;

    grid[r][c] = '0'; // позначаємо як відвіданий (in-place)

    dfs(grid, r + 1, c);
    dfs(grid, r - 1, c);
    dfs(grid, r, c + 1);
    dfs(grid, r, c - 1);
}
```
**Складність:** O(m * n) time, O(m * n) space (call stack)

---

### Задача 2: Clone Graph (LeetCode #133)
**Ключова думка:** HashMap: оригінальний вузол → його копія. Уникаємо нескінченного циклу.

```java
Map<Node, Node> cloned = new HashMap<>();

public Node cloneGraph(Node node) {
    if (node == null) return null;
    if (cloned.containsKey(node)) return cloned.get(node); // вже клонований

    Node copy = new Node(node.val);
    cloned.put(node, copy);

    for (Node neighbor : node.neighbors) {
        copy.neighbors.add(cloneGraph(neighbor));
    }

    return copy;
}
```

---

### Задача 3: Pacific Atlantic Water Flow (LeetCode #417)
**Умова:** Вода тече до Pacific (верхній/лівий край) і Atlantic (нижній/правий край). Знайти клітинки звідки вода може текти в обидва океани.

**Ключова думка:** Замість пошуку "звідки вода тече вниз" — шукаємо "куди вода може текти вгору від країв".

```java
public List<List<Integer>> pacificAtlantic(int[][] heights) {
    int rows = heights.length, cols = heights[0].length;
    boolean[][] pacific = new boolean[rows][cols];
    boolean[][] atlantic = new boolean[rows][cols];

    // DFS від країв обох океанів
    for (int r = 0; r < rows; r++) {
        dfs(heights, pacific, r, 0);           // лівий край → Pacific
        dfs(heights, atlantic, r, cols - 1);   // правий край → Atlantic
    }
    for (int c = 0; c < cols; c++) {
        dfs(heights, pacific, 0, c);           // верхній край → Pacific
        dfs(heights, atlantic, rows - 1, c);   // нижній край → Atlantic
    }

    List<List<Integer>> result = new ArrayList<>();
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            if (pacific[r][c] && atlantic[r][c]) {
                result.add(Arrays.asList(r, c));
            }
        }
    }

    return result;
}

private void dfs(int[][] h, boolean[][] visited, int r, int c) {
    visited[r][c] = true;
    int[][] dirs = {{0,1},{0,-1},{1,0},{-1,0}};

    for (int[] d : dirs) {
        int nr = r + d[0], nc = c + d[1];
        if (nr >= 0 && nr < h.length && nc >= 0 && nc < h[0].length
                && !visited[nr][nc] && h[nr][nc] >= h[r][c]) { // вгору або рівно
            dfs(h, visited, nr, nc);
        }
    }
}
```

---

## 📐 Патерн 2: BFS на графі

### Коли BFS краще за DFS
- Потрібен **найкоротший шлях** (кількість кроків)
- Невзважений граф
- Multi-source (кілька стартових точок одночасно)

### Шаблон — найкоротший шлях

```java
Queue<Integer> queue = new ArrayDeque<>();
Set<Integer> visited = new HashSet<>();

queue.offer(start);
visited.add(start);
int steps = 0;

while (!queue.isEmpty()) {
    int levelSize = queue.size();

    for (int i = 0; i < levelSize; i++) {
        int node = queue.poll();

        if (node == target) return steps;

        for (int neighbor : graph.get(node)) {
            if (!visited.contains(neighbor)) {
                visited.add(neighbor);
                queue.offer(neighbor);
            }
        }
    }

    steps++;
}

return -1; // не досяжно
```

### Задача 4: Rotting Oranges (LeetCode #994)
**Умова:** Grid з свіжими (1) і гнилими (2) апельсинами. Кожну хвилину гниль поширюється. Скільки хвилин?

**Ключова думка:** Multi-source BFS — всі гнилі апельсини стартують одночасно.

```java
public int orangesRotting(int[][] grid) {
    int rows = grid.length, cols = grid[0].length;
    Queue<int[]> queue = new ArrayDeque<>();
    int fresh = 0;

    // Знаходимо всі гнилі (старт BFS) і рахуємо свіжі
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            if (grid[r][c] == 2) queue.offer(new int[]{r, c});
            else if (grid[r][c] == 1) fresh++;
        }
    }

    if (fresh == 0) return 0;

    int[][] dirs = {{0,1},{0,-1},{1,0},{-1,0}};
    int minutes = 0;

    while (!queue.isEmpty() && fresh > 0) {
        minutes++;
        int size = queue.size();

        for (int i = 0; i < size; i++) {
            int[] pos = queue.poll();

            for (int[] d : dirs) {
                int nr = pos[0] + d[0], nc = pos[1] + d[1];

                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols
                        && grid[nr][nc] == 1) {
                    grid[nr][nc] = 2;
                    fresh--;
                    queue.offer(new int[]{nr, nc});
                }
            }
        }
    }

    return fresh == 0 ? minutes : -1;
}
```

---

### Задача 5: Word Ladder (LeetCode #127)
**Умова:** Трансформувати beginWord у endWord змінюючи одну літеру. Мінімальна кількість кроків.

```java
public int ladderLength(String beginWord, String endWord, List<String> wordList) {
    Set<String> wordSet = new HashSet<>(wordList);
    if (!wordSet.contains(endWord)) return 0;

    Queue<String> queue = new ArrayDeque<>();
    Set<String> visited = new HashSet<>();
    queue.offer(beginWord);
    visited.add(beginWord);
    int steps = 1;

    while (!queue.isEmpty()) {
        int size = queue.size();

        for (int i = 0; i < size; i++) {
            String word = queue.poll();
            char[] chars = word.toCharArray();

            // пробуємо замінити кожну літеру
            for (int j = 0; j < chars.length; j++) {
                char original = chars[j];

                for (char c = 'a'; c <= 'z'; c++) {
                    chars[j] = c;
                    String next = new String(chars);

                    if (next.equals(endWord)) return steps + 1;

                    if (wordSet.contains(next) && !visited.contains(next)) {
                        visited.add(next);
                        queue.offer(next);
                    }
                }

                chars[j] = original; // відновлюємо
            }
        }

        steps++;
    }

    return 0;
}
```
**Складність:** O(M² * N) де M — довжина слова, N — розмір словника

---

## 📐 Патерн 3: Cycle Detection

### Варіант A — DFS з кольорами (directed graph)
```
WHITE (0) = не відвіданий
GRAY  (1) = у процесі обходу (у поточному DFS path)
BLACK (2) = повністю оброблений
```

```java
int[] color; // 0=white, 1=gray, 2=black

boolean hasCycle(int node) {
    color[node] = 1; // gray — входимо

    for (int neighbor : graph.get(node)) {
        if (color[neighbor] == 1) return true;  // gray → цикл!
        if (color[neighbor] == 0) {             // white → рекурсія
            if (hasCycle(neighbor)) return true;
        }
    }

    color[node] = 2; // black — виходимо
    return false;
}
```

### Варіант B — DFS з parent (undirected graph)

```java
boolean dfs(int node, int parent, boolean[] visited) {
    visited[node] = true;

    for (int neighbor : graph.get(node)) {
        if (!visited[neighbor]) {
            if (dfs(neighbor, node, visited)) return true;
        } else if (neighbor != parent) {
            return true; // знайшли відвіданий вузол що не є батьком → цикл
        }
    }

    return false;
}
```

---

## 📐 Патерн 4: Topological Sort

### Коли використовувати
- Directed Acyclic Graph (DAG)
- Задачі на залежності: "чи можна виконати всі курси", "порядок збірки"

### Варіант A — BFS (Kahn's Algorithm) — рекомендований

```java
// Крок 1: рахуємо in-degree кожного вузла
int[] inDegree = new int[n];
for (int[] edge : edges) inDegree[edge[1]]++;

// Крок 2: починаємо з вузлів без залежностей (in-degree = 0)
Queue<Integer> queue = new ArrayDeque<>();
for (int i = 0; i < n; i++) {
    if (inDegree[i] == 0) queue.offer(i);
}

// Крок 3: BFS
List<Integer> order = new ArrayList<>();
while (!queue.isEmpty()) {
    int node = queue.poll();
    order.add(node);

    for (int neighbor : graph.get(node)) {
        inDegree[neighbor]--;
        if (inDegree[neighbor] == 0) queue.offer(neighbor);
    }
}

// Якщо order.size() != n → є цикл (не всі вузли оброблені)
boolean hasCycle = order.size() != n;
```

### Задача 6: Course Schedule (LeetCode #207)
**Умова:** Чи можна пройти всі курси з урахуванням prerequisites?

```java
public boolean canFinish(int numCourses, int[][] prerequisites) {
    List<List<Integer>> graph = new ArrayList<>();
    int[] inDegree = new int[numCourses];

    for (int i = 0; i < numCourses; i++) graph.add(new ArrayList<>());

    for (int[] pre : prerequisites) {
        graph.get(pre[1]).add(pre[0]); // pre[1] → pre[0]
        inDegree[pre[0]]++;
    }

    Queue<Integer> queue = new ArrayDeque<>();
    for (int i = 0; i < numCourses; i++) {
        if (inDegree[i] == 0) queue.offer(i);
    }

    int completed = 0;
    while (!queue.isEmpty()) {
        int course = queue.poll();
        completed++;

        for (int next : graph.get(course)) {
            if (--inDegree[next] == 0) queue.offer(next);
        }
    }

    return completed == numCourses;
}
```

---

### Задача 7: Course Schedule II — повернути порядок (LeetCode #210)

```java
public int[] findOrder(int numCourses, int[][] prerequisites) {
    List<List<Integer>> graph = new ArrayList<>();
    int[] inDegree = new int[numCourses];

    for (int i = 0; i < numCourses; i++) graph.add(new ArrayList<>());
    for (int[] pre : prerequisites) {
        graph.get(pre[1]).add(pre[0]);
        inDegree[pre[0]]++;
    }

    Queue<Integer> queue = new ArrayDeque<>();
    for (int i = 0; i < numCourses; i++) {
        if (inDegree[i] == 0) queue.offer(i);
    }

    int[] order = new int[numCourses];
    int idx = 0;

    while (!queue.isEmpty()) {
        int course = queue.poll();
        order[idx++] = course;

        for (int next : graph.get(course)) {
            if (--inDegree[next] == 0) queue.offer(next);
        }
    }

    return idx == numCourses ? order : new int[]{};
}
```

---

## 📐 Патерн 5: Union-Find (Disjoint Set Union)

### Коли використовувати
- "Connected components" — кількість компонент
- "Are A and B connected?"
- Динамічне об'єднання груп

### Шаблон з Path Compression і Union by Rank

```java
class UnionFind {
    int[] parent;
    int[] rank;
    int components;

    UnionFind(int n) {
        parent = new int[n];
        rank = new int[n];
        components = n;
        for (int i = 0; i < n; i++) parent[i] = i; // кожен сам собі батько
    }

    int find(int x) {
        if (parent[x] != x) {
            parent[x] = find(parent[x]); // path compression
        }
        return parent[x];
    }

    boolean union(int x, int y) {
        int px = find(x), py = find(y);
        if (px == py) return false; // вже в одній групі

        // union by rank — менше дерево під більше
        if (rank[px] < rank[py]) { int tmp = px; px = py; py = tmp; }
        parent[py] = px;
        if (rank[px] == rank[py]) rank[px]++;

        components--;
        return true;
    }

    boolean connected(int x, int y) {
        return find(x) == find(y);
    }
}
```

### Задача 8: Number of Provinces (LeetCode #547)

```java
public int findCircleNum(int[][] isConnected) {
    int n = isConnected.length;
    UnionFind uf = new UnionFind(n);

    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            if (isConnected[i][j] == 1) uf.union(i, j);
        }
    }

    return uf.components;
}
```

---

### Задача 9: Redundant Connection (LeetCode #684)
**Умова:** Знайти ребро що створює цикл у графі.

```java
public int[] findRedundantConnection(int[][] edges) {
    int n = edges.length;
    UnionFind uf = new UnionFind(n + 1);

    for (int[] edge : edges) {
        if (!uf.union(edge[0], edge[1])) {
            return edge; // union повернув false → вже з'єднані → цикл
        }
    }

    return new int[]{};
}
```

---

### Задача 10: Number of Islands з Union-Find

```java
public int numIslands(char[][] grid) {
    int rows = grid.length, cols = grid[0].length;
    UnionFind uf = new UnionFind(rows * cols);
    int water = 0;

    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            if (grid[r][c] == '0') {
                water++;
                continue;
            }
            // з'єднуємо з сусідами
            int[][] dirs = {{0,1},{1,0}};
            for (int[] d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr < rows && nc < cols && grid[nr][nc] == '1') {
                    uf.union(r * cols + c, nr * cols + nc);
                }
            }
        }
    }

    return uf.components - water;
}
```

---

## 📐 Патерн 6: Advanced Graph — Shortest Path (Dijkstra)

### Коли використовувати
- Зважений граф
- Найкоротший шлях від одного вузла до всіх інших

```java
public int[] dijkstra(int n, int[][] edges, int src) {
    // Побудова графа: node → list of [neighbor, weight]
    Map<Integer, List<int[]>> graph = new HashMap<>();
    for (int[] e : edges) {
        graph.computeIfAbsent(e[0], k -> new ArrayList<>()).add(new int[]{e[1], e[2]});
        graph.computeIfAbsent(e[1], k -> new ArrayList<>()).add(new int[]{e[0], e[2]});
    }

    int[] dist = new int[n];
    Arrays.fill(dist, Integer.MAX_VALUE);
    dist[src] = 0;

    // Min-heap: [відстань, вузол]
    PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
    pq.offer(new int[]{0, src});

    while (!pq.isEmpty()) {
        int[] curr = pq.poll();
        int d = curr[0], node = curr[1];

        if (d > dist[node]) continue; // застаріла запис

        for (int[] neighbor : graph.getOrDefault(node, new ArrayList<>())) {
            int newDist = dist[node] + neighbor[1];
            if (newDist < dist[neighbor[0]]) {
                dist[neighbor[0]] = newDist;
                pq.offer(new int[]{newDist, neighbor[0]});
            }
        }
    }

    return dist;
}
```

### Задача 11: Network Delay Time (LeetCode #743)

```java
public int networkDelayTime(int[][] times, int n, int k) {
    Map<Integer, List<int[]>> graph = new HashMap<>();
    for (int[] t : times) {
        graph.computeIfAbsent(t[0], key -> new ArrayList<>())
             .add(new int[]{t[1], t[2]});
    }

    int[] dist = new int[n + 1];
    Arrays.fill(dist, Integer.MAX_VALUE);
    dist[k] = 0;

    PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
    pq.offer(new int[]{0, k});

    while (!pq.isEmpty()) {
        int[] curr = pq.poll();
        int d = curr[0], node = curr[1];

        if (d > dist[node]) continue;

        for (int[] next : graph.getOrDefault(node, new ArrayList<>())) {
            int newDist = dist[node] + next[1];
            if (newDist < dist[next[0]]) {
                dist[next[0]] = newDist;
                pq.offer(new int[]{newDist, next[0]});
            }
        }
    }

    int maxDist = 0;
    for (int i = 1; i <= n; i++) {
        if (dist[i] == Integer.MAX_VALUE) return -1;
        maxDist = Math.max(maxDist, dist[i]);
    }

    return maxDist;
}
```
**Складність:** O((V + E) log V)

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Graph
│
├── "Connected components" / "number of islands"?
│   ├── DFS / BFS — простіше кодувати
│   └── Union-Find — ефективніше при динамічних з'єднаннях
│
├── "Shortest path"?
│   ├── Невзважений граф → BFS (кожен крок = 1)
│   └── Зважений граф → Dijkstra (Min-Heap + dist[])
│
├── "Cycle detection"?
│   ├── Undirected → DFS з parent або Union-Find
│   └── Directed → DFS з кольорами (white/gray/black)
│
├── "Prerequisites" / "order of tasks"?
│   └── Topological Sort (Kahn's BFS з in-degree)
│       → якщо processed != n → є цикл
│
├── "Multi-source" (кілька стартів одночасно)?
│   └── BFS — додаємо всі старти в queue на початку
│
└── "Clone" граф?
    └── DFS / BFS + HashMap (original → copy)
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| Не додавати до visited **перед** додаванням у queue | `visited.add(node)` одразу при `queue.offer(node)` — уникаємо дублікатів |
| Забути перевірку меж у grid DFS | `r < 0 \|\| r >= rows \|\| c < 0 \|\| c >= cols` |
| Union-Find без path compression | `parent[x] = find(parent[x])` у методі `find` |
| Topological Sort: не перевіряти `order.size() == n` | Без цього не виявляємо цикли |
| Dijkstra: не пропускати застарілі записи | `if (d > dist[node]) continue;` |
| Directed graph: додавати ребро в обидва напрямки | Для directed — тільки `graph.get(from).add(to)` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #200 Number of Islands
- [ ] #133 Clone Graph
- [ ] #207 Course Schedule
- [ ] #547 Number of Provinces
- [ ] #994 Rotting Oranges
- [ ] #210 Course Schedule II

### Should Solve (Middle)
- [ ] #417 Pacific Atlantic Water Flow
- [ ] #684 Redundant Connection
- [ ] #743 Network Delay Time
- [ ] #127 Word Ladder
- [ ] #130 Surrounded Regions
- [ ] #695 Max Area of Island

### Stretch Goals
- [ ] #269 Alien Dictionary (Topological Sort)
- [ ] #787 Cheapest Flights Within K Stops
- [ ] #1584 Min Cost to Connect All Points (MST)
- [ ] #323 Number of Connected Components in Undirected Graph

---

## 🔑 Quick Reference: Graph у Java

```java
// Adjacency List (найпоширеніший)
List<List<Integer>> graph = new ArrayList<>();
for (int i = 0; i < n; i++) graph.add(new ArrayList<>());
graph.get(u).add(v);
graph.get(v).add(u); // undirected

// HashMap варіант (коли вузли не 0..n-1)
Map<Integer, List<Integer>> graph = new HashMap<>();
graph.computeIfAbsent(u, k -> new ArrayList<>()).add(v);

// Grid → індекс
int idx = row * cols + col;
int row = idx / cols;
int col = idx % cols;

// 4 напрямки
int[][] dirs = {{0,1},{0,-1},{1,0},{-1,0}};

// 8 напрямків (включаючи діагоналі)
int[][] dirs8 = {{0,1},{0,-1},{1,0},{-1,0},{1,1},{1,-1},{-1,1},{-1,-1}};

// BFS шаблон з visited
Queue<Integer> q = new ArrayDeque<>();
boolean[] visited = new boolean[n];
q.offer(start);
visited[start] = true;           // ← одразу при offer, не при poll!
while (!q.isEmpty()) {
    int node = q.poll();
    for (int next : graph.get(node)) {
        if (!visited[next]) {
            visited[next] = true;
            q.offer(next);
        }
    }
}

// Union-Find (мінімальна версія для інтерв'ю)
int[] parent = new int[n];
Arrays.fill(parent, -1);         // або: for(int i=0;i<n;i++) parent[i]=i;

int find(int x) {
    return parent[x] < 0 ? x : (parent[x] = find(parent[x]));
}

void union(int x, int y) {
    x = find(x); y = find(y);
    if (x == y) return;
    if (parent[x] > parent[y]) { int t = x; x = y; y = t; }
    parent[x] += parent[y];      // rank зберігаємо як від'ємне число
    parent[y] = x;
}
```
