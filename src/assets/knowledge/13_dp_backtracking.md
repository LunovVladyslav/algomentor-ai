# 13. Dynamic Programming & Backtracking

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 5–7 днів

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "max/min ways", "how many ways" | DP |
| "can you reach", "is it possible" | DP (boolean) |
| "longest/shortest subsequence/subarray" | DP |
| "cut/partition into pieces" | DP |
| "all combinations / permutations / subsets" | Backtracking |
| "generate all valid" | Backtracking |
| "word break", "decode ways" | DP |
| "knapsack", "pick items with weight/value" | DP (Knapsack) |
| "edit distance", "matching strings" | DP 2D |

---

## 📐 DP vs Backtracking — як вибрати

```
Потрібна ОДНА відповідь (max, min, count)?
→ DP

Потрібні ВСІ варіанти (список рішень)?
→ Backtracking

Підзадачі перекриваються (один стан = кілька шляхів)?
→ DP (мемоізація)

Стани не перекриваються або простір станів завеликий?
→ Backtracking з pruning
```

---

## 📐 Патерн 1: 1D DP

### Шаблон

```java
int[] dp = new int[n + 1];
dp[0] = base_case;

for (int i = 1; i <= n; i++) {
    dp[i] = f(dp[i-1], dp[i-2], ...); // перехід
}

return dp[n];
```

### Задача 1: Climbing Stairs (LeetCode #70)
**Умова:** n сходинок, можна робити 1 або 2 кроки. Кількість способів.

```java
public int climbStairs(int n) {
    if (n <= 2) return n;

    int prev2 = 1, prev1 = 2;

    for (int i = 3; i <= n; i++) {
        int curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }

    return prev1;
}
```
> 💡 Оптимізація пам'яті: замість `dp[]` зберігаємо тільки останні 2 значення.

---

### Задача 2: House Robber (LeetCode #198)
**Умова:** Не можна грабувати сусідні будинки. Максимальна сума.

```java
public int rob(int[] nums) {
    if (nums.length == 1) return nums[0];

    int prev2 = nums[0];
    int prev1 = Math.max(nums[0], nums[1]);

    for (int i = 2; i < nums.length; i++) {
        int curr = Math.max(prev1, prev2 + nums[i]);
        prev2 = prev1;
        prev1 = curr;
    }

    return prev1;
}
```

---

### Задача 3: House Robber II (LeetCode #213)
**Умова:** Будинки розташовані в колі. Перший і останній — сусіди.

**Ключова думка:** Два запуски House Robber — з включенням першого або останнього.

```java
public int rob(int[] nums) {
    int n = nums.length;
    if (n == 1) return nums[0];
    if (n == 2) return Math.max(nums[0], nums[1]);

    return Math.max(
        robRange(nums, 0, n - 2), // без останнього
        robRange(nums, 1, n - 1)  // без першого
    );
}

private int robRange(int[] nums, int start, int end) {
    int prev2 = nums[start];
    int prev1 = Math.max(nums[start], nums[start + 1]);

    for (int i = start + 2; i <= end; i++) {
        int curr = Math.max(prev1, prev2 + nums[i]);
        prev2 = prev1;
        prev1 = curr;
    }

    return prev1;
}
```

---

### Задача 4: Decode Ways (LeetCode #91)
**Умова:** Рядок з цифр — скільки способів декодувати в літери (1=A, 26=Z).

```java
public int numDecodings(String s) {
    int n = s.length();
    int[] dp = new int[n + 1];
    dp[0] = 1; // порожній рядок — 1 спосіб
    dp[1] = s.charAt(0) == '0' ? 0 : 1;

    for (int i = 2; i <= n; i++) {
        int oneDigit = Integer.parseInt(s.substring(i - 1, i));
        int twoDigit = Integer.parseInt(s.substring(i - 2, i));

        if (oneDigit >= 1) dp[i] += dp[i - 1]; // одна цифра
        if (twoDigit >= 10 && twoDigit <= 26) dp[i] += dp[i - 2]; // дві цифри
    }

    return dp[n];
}
```

---

### Задача 5: Word Break (LeetCode #139)
**Умова:** Чи можна розбити рядок на слова зі словника?

```java
public boolean wordBreak(String s, List<String> wordDict) {
    Set<String> dict = new HashSet<>(wordDict);
    int n = s.length();
    boolean[] dp = new boolean[n + 1];
    dp[0] = true; // порожній рядок

    for (int i = 1; i <= n; i++) {
        for (int j = 0; j < i; j++) {
            if (dp[j] && dict.contains(s.substring(j, i))) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[n];
}
```

---

## 📐 Патерн 2: 2D DP — рядки

### Задача 6: Longest Common Subsequence (LeetCode #1143)

**Перехід:**
```
dp[i][j] = LCS для s1[0..i-1] і s2[0..j-1]

if s1[i-1] == s2[j-1]: dp[i][j] = dp[i-1][j-1] + 1
else:                   dp[i][j] = max(dp[i-1][j], dp[i][j-1])
```

```java
public int longestCommonSubsequence(String text1, String text2) {
    int m = text1.length(), n = text2.length();
    int[][] dp = new int[m + 1][n + 1];

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (text1.charAt(i - 1) == text2.charAt(j - 1)) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    return dp[m][n];
}
```
**Складність:** O(m*n) time, O(m*n) space

---

### Задача 7: Edit Distance (LeetCode #72)
**Умова:** Мінімальна кількість операцій (insert, delete, replace) для перетворення word1 → word2.

```
dp[i][j] = min операцій для word1[0..i-1] → word2[0..j-1]

if word1[i-1] == word2[j-1]: dp[i][j] = dp[i-1][j-1]
else: dp[i][j] = 1 + min(
    dp[i-1][j],    // delete з word1
    dp[i][j-1],    // insert у word1
    dp[i-1][j-1]   // replace
)
```

```java
public int minDistance(String word1, String word2) {
    int m = word1.length(), n = word2.length();
    int[][] dp = new int[m + 1][n + 1];

    // base cases: перетворення порожнього рядка
    for (int i = 0; i <= m; i++) dp[i][0] = i;
    for (int j = 0; j <= n; j++) dp[0][j] = j;

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (word1.charAt(i - 1) == word2.charAt(j - 1)) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + Math.min(dp[i - 1][j - 1],
                               Math.min(dp[i - 1][j], dp[i][j - 1]));
            }
        }
    }

    return dp[m][n];
}
```

---

## 📐 Патерн 3: 2D DP — Grid

### Задача 8: Unique Paths (LeetCode #62)
**Умова:** Скільки шляхів з лівого верхнього до правого нижнього кута (тільки вправо і вниз)?

```java
public int uniquePaths(int m, int n) {
    int[][] dp = new int[m][n];

    // перший рядок і перший стовпець — тільки один шлях
    for (int i = 0; i < m; i++) dp[i][0] = 1;
    for (int j = 0; j < n; j++) dp[0][j] = 1;

    for (int i = 1; i < m; i++) {
        for (int j = 1; j < n; j++) {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    return dp[m - 1][n - 1];
}
```

---

### Задача 9: Minimum Path Sum (LeetCode #64)

```java
public int minPathSum(int[][] grid) {
    int m = grid.length, n = grid[0].length;
    int[][] dp = new int[m][n];
    dp[0][0] = grid[0][0];

    for (int i = 1; i < m; i++) dp[i][0] = dp[i-1][0] + grid[i][0];
    for (int j = 1; j < n; j++) dp[0][j] = dp[0][j-1] + grid[0][j];

    for (int i = 1; i < m; i++) {
        for (int j = 1; j < n; j++) {
            dp[i][j] = Math.min(dp[i-1][j], dp[i][j-1]) + grid[i][j];
        }
    }

    return dp[m-1][n-1];
}
```

---

## 📐 Патерн 4: Knapsack

### 0/1 Knapsack — кожен предмет або беремо або ні

```java
// dp[i][w] = макс. цінність з перших i предметів з вагою <= w
int[][] dp = new int[n + 1][W + 1];

for (int i = 1; i <= n; i++) {
    for (int w = 0; w <= W; w++) {
        dp[i][w] = dp[i-1][w]; // не беремо i-й предмет
        if (weights[i-1] <= w) {
            dp[i][w] = Math.max(dp[i][w],
                dp[i-1][w - weights[i-1]] + values[i-1]); // беремо
        }
    }
}
```

**Оптимізація до 1D:**

```java
int[] dp = new int[W + 1];

for (int i = 0; i < n; i++) {
    for (int w = W; w >= weights[i]; w--) { // ← обов'язково від W до weight[i]
        dp[w] = Math.max(dp[w], dp[w - weights[i]] + values[i]);
    }
}
```

### Задача 10: Partition Equal Subset Sum (LeetCode #416)
**Умова:** Чи можна розбити масив на дві частини з рівними сумами? (0/1 Knapsack)

```java
public boolean canPartition(int[] nums) {
    int total = Arrays.stream(nums).sum();
    if (total % 2 != 0) return false;

    int target = total / 2;
    boolean[] dp = new boolean[target + 1];
    dp[0] = true;

    for (int num : nums) {
        for (int j = target; j >= num; j--) { // від target до num
            dp[j] = dp[j] || dp[j - num];
        }
    }

    return dp[target];
}
```

---

### Unbounded Knapsack — кожен предмет можна брати необмежено

```java
int[] dp = new int[W + 1];

for (int i = 0; i < n; i++) {
    for (int w = weights[i]; w <= W; w++) { // ← від weight[i] до W (не навпаки!)
        dp[w] = Math.max(dp[w], dp[w - weights[i]] + values[i]);
    }
}
```

### Задача 11: Coin Change (LeetCode #322)
**Умова:** Мінімальна кількість монет для суми amount. (Unbounded Knapsack)

```java
public int coinChange(int[] coins, int amount) {
    int[] dp = new int[amount + 1];
    Arrays.fill(dp, amount + 1); // "нескінченність"
    dp[0] = 0;

    for (int coin : coins) {
        for (int j = coin; j <= amount; j++) {
            dp[j] = Math.min(dp[j], dp[j - coin] + 1);
        }
    }

    return dp[amount] > amount ? -1 : dp[amount];
}
```

---

### Задача 12: Coin Change II — кількість способів (LeetCode #518)

```java
public int change(int amount, int[] coins) {
    int[] dp = new int[amount + 1];
    dp[0] = 1;

    for (int coin : coins) {
        for (int j = coin; j <= amount; j++) {
            dp[j] += dp[j - coin];
        }
    }

    return dp[amount];
}
```

> ⚠️ Порядок циклів важливий!  
> Outer loop = монети, inner loop = сума → кожна комбінація рахується **один** раз.  
> Якщо поміняти — рахуватимуться permutations (різний порядок монет = різний варіант).

---

## 📐 Патерн 5: Longest Increasing Subsequence (LIS)

### Задача 13: LIS (LeetCode #300)

**O(n²) DP:**
```java
public int lengthOfLIS(int[] nums) {
    int n = nums.length;
    int[] dp = new int[n];
    Arrays.fill(dp, 1);

    int maxLen = 1;
    for (int i = 1; i < n; i++) {
        for (int j = 0; j < i; j++) {
            if (nums[j] < nums[i]) {
                dp[i] = Math.max(dp[i], dp[j] + 1);
            }
        }
        maxLen = Math.max(maxLen, dp[i]);
    }

    return maxLen;
}
```

**O(n log n) — Binary Search + patience sorting:**
```java
public int lengthOfLIS(int[] nums) {
    List<Integer> tails = new ArrayList<>();

    for (int num : nums) {
        int pos = Collections.binarySearch(tails, num);
        if (pos < 0) pos = -(pos + 1); // insertion point

        if (pos == tails.size()) tails.add(num); // розширюємо
        else tails.set(pos, num);                // замінюємо
    }

    return tails.size();
}
```

---

## 📐 Патерн 6: Backtracking

### Шаблон

```java
void backtrack(стан, результати) {
    if (базовий_випадок) {
        результати.add(копія_стану);
        return;
    }

    for (кожен можливий вибір) {
        if (вибір не підходить) continue; // pruning

        зробити_вибір(стан);
        backtrack(стан, результати);
        скасувати_вибір(стан); // ← обов'язковий backtrack
    }
}
```

### Задача 14: Subsets (LeetCode #78)

```java
public List<List<Integer>> subsets(int[] nums) {
    List<List<Integer>> result = new ArrayList<>();
    backtrack(nums, 0, new ArrayList<>(), result);
    return result;
}

private void backtrack(int[] nums, int start,
                       List<Integer> current, List<List<Integer>> result) {
    result.add(new ArrayList<>(current)); // додаємо поточний стан

    for (int i = start; i < nums.length; i++) {
        current.add(nums[i]);
        backtrack(nums, i + 1, current, result);
        current.remove(current.size() - 1); // backtrack
    }
}
```

---

### Задача 15: Permutations (LeetCode #46)

```java
public List<List<Integer>> permutations(int[] nums) {
    List<List<Integer>> result = new ArrayList<>();
    backtrack(nums, new boolean[nums.length], new ArrayList<>(), result);
    return result;
}

private void backtrack(int[] nums, boolean[] used,
                       List<Integer> current, List<List<Integer>> result) {
    if (current.size() == nums.length) {
        result.add(new ArrayList<>(current));
        return;
    }

    for (int i = 0; i < nums.length; i++) {
        if (used[i]) continue;

        used[i] = true;
        current.add(nums[i]);
        backtrack(nums, used, current, result);
        current.remove(current.size() - 1);
        used[i] = false;
    }
}
```

---

### Задача 16: Combination Sum (LeetCode #39)
**Умова:** Знайти всі комбінації що дають target (елементи можна повторювати).

```java
public List<List<Integer>> combinationSum(int[] candidates, int target) {
    List<List<Integer>> result = new ArrayList<>();
    Arrays.sort(candidates); // для pruning
    backtrack(candidates, target, 0, new ArrayList<>(), result);
    return result;
}

private void backtrack(int[] candidates, int remaining, int start,
                       List<Integer> current, List<List<Integer>> result) {
    if (remaining == 0) {
        result.add(new ArrayList<>(current));
        return;
    }

    for (int i = start; i < candidates.length; i++) {
        if (candidates[i] > remaining) break; // pruning — sorted array

        current.add(candidates[i]);
        backtrack(candidates, remaining - candidates[i], i, current, result); // i, не i+1 → повтор
        current.remove(current.size() - 1);
    }
}
```

---

### Задача 17: N-Queens (LeetCode #51)

```java
public List<List<String>> solveNQueens(int n) {
    List<List<String>> result = new ArrayList<>();
    int[] queens = new int[n]; // queens[row] = col
    Arrays.fill(queens, -1);

    Set<Integer> cols = new HashSet<>();
    Set<Integer> diag1 = new HashSet<>(); // row - col
    Set<Integer> diag2 = new HashSet<>(); // row + col

    backtrack(0, n, queens, cols, diag1, diag2, result);
    return result;
}

private void backtrack(int row, int n, int[] queens,
                       Set<Integer> cols, Set<Integer> diag1,
                       Set<Integer> diag2, List<List<String>> result) {
    if (row == n) {
        result.add(buildBoard(queens, n));
        return;
    }

    for (int col = 0; col < n; col++) {
        if (cols.contains(col) || diag1.contains(row - col)
                || diag2.contains(row + col)) continue;

        queens[row] = col;
        cols.add(col); diag1.add(row - col); diag2.add(row + col);

        backtrack(row + 1, n, queens, cols, diag1, diag2, result);

        queens[row] = -1;
        cols.remove(col); diag1.remove(row - col); diag2.remove(row + col);
    }
}

private List<String> buildBoard(int[] queens, int n) {
    List<String> board = new ArrayList<>();
    for (int row = 0; row < n; row++) {
        char[] line = new char[n];
        Arrays.fill(line, '.');
        line[queens[row]] = 'Q';
        board.add(new String(line));
    }
    return board;
}
```

---

## 📐 Патерн 7: Мемоізація (Top-Down DP)

### Коли використовувати
- Рекурсивне рішення очевидне, але повільне через повторні обчислення
- Додаємо кеш `Map` або `int[][]`

### Задача 18: Fibonacci (мемоізація)

```java
Map<Integer, Long> memo = new HashMap<>();

long fib(int n) {
    if (n <= 1) return n;
    if (memo.containsKey(n)) return memo.get(n);

    long result = fib(n - 1) + fib(n - 2);
    memo.put(n, result);
    return result;
}
```

### Задача 19: Unique Paths з мемоізацією

```java
public int uniquePaths(int m, int n) {
    return dfs(m - 1, n - 1, new int[m][n]);
}

private int dfs(int r, int c, int[][] memo) {
    if (r == 0 || c == 0) return 1;
    if (memo[r][c] != 0) return memo[r][c];
    return memo[r][c] = dfs(r - 1, c, memo) + dfs(r, c - 1, memo);
}
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на DP / Backtracking
│
├── Потрібна ОДНА відповідь (count/max/min/bool)?
│   └── DYNAMIC PROGRAMMING
│       ├── 1 параметр змінюється → 1D DP
│       ├── 2 параметри (рядки, grid) → 2D DP
│       ├── "вибрати або ні" без повторень → 0/1 Knapsack
│       └── "вибирати необмежено" → Unbounded Knapsack
│
├── Потрібні ВСІ рішення (список)?
│   └── BACKTRACKING
│       ├── subsets → start index, не використовуй used[]
│       ├── permutations → використовуй used[]
│       └── combinations → start index + pruning якщо sorted
│
└── Рекурсія очевидна але повільна (TLE)?
    └── МЕМОІЗАЦІЯ (додай Map/array кеш)
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| Не робити копію при `result.add(current)` | `result.add(new ArrayList<>(current))` |
| Забути backtrack (не відміняти вибір) | `current.remove(...)` або `used[i] = false` після рекурсії |
| 0/1 Knapsack: inner loop від малого до великого | Inner loop від `W` до `weight[i]` (зворотній порядок) |
| Unbounded Knapsack: inner loop навпаки | Inner loop від `weight[i]` до `W` (прямий порядок) |
| `dp[0]` не ініціалізований (base case) | Завжди встановлюй base case вручну |
| Coin Change: `dp[amount+1]` як нескінченність | Використовуй `amount + 1` — будь-яка відповідь ≤ amount |
| Permutations з дублікатами: не сортувати і не пропускати | `Arrays.sort` + `if (i > 0 && nums[i] == nums[i-1] && !used[i-1]) continue` |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #70 Climbing Stairs
- [ ] #198 House Robber
- [ ] #322 Coin Change
- [ ] #139 Word Break
- [ ] #300 Longest Increasing Subsequence
- [ ] #78 Subsets
- [ ] #46 Permutations
- [ ] #39 Combination Sum

### Should Solve (Middle)
- [ ] #213 House Robber II
- [ ] #91 Decode Ways
- [ ] #1143 Longest Common Subsequence
- [ ] #72 Edit Distance
- [ ] #416 Partition Equal Subset Sum
- [ ] #518 Coin Change II
- [ ] #62 Unique Paths
- [ ] #51 N-Queens

### Stretch Goals
- [ ] #312 Burst Balloons
- [ ] #10 Regular Expression Matching
- [ ] #115 Distinct Subsequences
- [ ] #132 Palindrome Partitioning II
