# 12. Bit Manipulation & Math

> **Рівень:** Junior Strong / Middle  
> **Мова:** Java  
> **Час на освоєння:** 2–3 дні

---

## 🧭 Як розпізнати тип задачі

| Сигнал в умові | Патерн |
|---|---|
| "single number", "find unique" | XOR |
| "power of 2 / 4", "count bits" | Bit tricks |
| "subsets", "all combinations" | Bit mask enumeration |
| "missing number", "duplicate" | XOR або Math |
| "divide without /" | Bit shifts |
| "GCD", "LCM" | Math (Euclidean) |
| "prime numbers up to n" | Sieve of Eratosthenes |
| "digit sum", "happy number" | Math simulation |
| "overflow check", "large numbers" | Long або modular arithmetic |

---

## 📐 Базові бітові операції

```java
// AND — обидва біти 1
a & b

// OR — хоча б один біт 1
a | b

// XOR — різні біти (0^0=0, 1^1=0, 0^1=1)
a ^ b

// NOT — інвертування
~a

// Left shift (множення на 2^n)
a << n   // a * 2^n

// Right shift (ділення на 2^n)
a >> n   // a / 2^n (signed, зберігає знак)
a >>> n  // unsigned right shift

// Перевірка i-го біта
(a >> i) & 1  // 0 або 1

// Встановити i-й біт
a | (1 << i)

// Очистити i-й біт
a & ~(1 << i)

// Перевернути i-й біт
a ^ (1 << i)

// Видалити найправіший встановлений біт
a & (a - 1)

// Залишити тільки найправіший встановлений біт
a & (-a)
```

---

## 📐 Патерн 1: XOR tricks

### Властивості XOR
```
a ^ 0 = a       (XOR з нулем = без змін)
a ^ a = 0       (XOR з собою = 0)
a ^ b ^ a = b   (асоціативність + комутативність)
```

### Задача 1: Single Number (LeetCode #136)
**Умова:** Всі числа зустрічаються двічі крім одного. Знайти його. O(1) space.

```java
public int singleNumber(int[] nums) {
    int result = 0;
    for (int num : nums) result ^= num;
    // всі пари скасовуються (a^a=0), залишається одиночний
    return result;
}
```
**Складність:** O(n) time, O(1) space

---

### Задача 2: Single Number III (LeetCode #260)
**Умова:** Два числа зустрічаються по одному разу, решта — двічі. Знайти обидва.

```java
public int[] singleNumber(int[] nums) {
    int xor = 0;
    for (int num : nums) xor ^= num; // xor = a ^ b

    // знаходимо будь-який біт де a і b відрізняються
    int diffBit = xor & (-xor); // найправіший встановлений біт

    int a = 0, b = 0;
    for (int num : nums) {
        if ((num & diffBit) != 0) a ^= num; // група з бітом
        else b ^= num;                       // група без біту
    }

    return new int[]{a, b};
}
```

---

### Задача 3: Missing Number (LeetCode #268)

```java
// XOR варіант
public int missingNumber(int[] nums) {
    int result = nums.length; // починаємо з n
    for (int i = 0; i < nums.length; i++) {
        result ^= i ^ nums[i]; // XOR з індексом і значенням
    }
    return result;
}

// Math варіант (простіший)
public int missingNumber(int[] nums) {
    int n = nums.length;
    int expected = n * (n + 1) / 2;
    int actual = 0;
    for (int num : nums) actual += num;
    return expected - actual;
}
```

---

## 📐 Патерн 2: Brian Kernighan — Count Bits

### Ключова ідея: `n & (n-1)` видаляє найправіший встановлений біт

```
n     = 1100
n - 1 = 1011
n & (n-1) = 1000  ← один біт видалено
```

### Задача 4: Number of 1 Bits (LeetCode #191)

```java
public int hammingWeight(int n) {
    int count = 0;
    while (n != 0) {
        n &= (n - 1); // видаляємо найправіший 1-біт
        count++;
    }
    return count;
}
```

---

### Задача 5: Counting Bits (LeetCode #338)
**Умова:** Для кожного числа від 0 до n — кількість бітів 1. O(n).

**Ключова думка:** `dp[i] = dp[i >> 1] + (i & 1)` — кількість бітів у i = кількість бітів у i/2 + молодший біт.

```java
public int[] countBits(int n) {
    int[] dp = new int[n + 1];
    for (int i = 1; i <= n; i++) {
        dp[i] = dp[i >> 1] + (i & 1);
    }
    return dp;
}
```

---

### Задача 6: Reverse Bits (LeetCode #190)

```java
public int reverseBits(int n) {
    int result = 0;
    for (int i = 0; i < 32; i++) {
        result = (result << 1) | (n & 1); // беремо молодший біт n, додаємо до result
        n >>= 1;
    }
    return result;
}
```

---

## 📐 Патерн 3: Power of 2 / Перевірка степеня

### `n & (n-1) == 0` → n є степенем 2 (і n > 0)

```java
// Power of Two (LeetCode #231)
public boolean isPowerOfTwo(int n) {
    return n > 0 && (n & (n - 1)) == 0;
}

// Power of Four (LeetCode #342)
// Степінь 4: 1, 4, 16, 64... — біт тільки на парних позиціях (0, 2, 4...)
// Маска для парних позицій: 0x55555555 = 01010101...
public boolean isPowerOfFour(int n) {
    return n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0;
}
```

---

## 📐 Патерн 4: Bit Mask — перебір підмножин

### Ключова думка
Для масиву розміру n маємо 2^n підмножин.  
Кожна підмножина відповідає числу від 0 до 2^n - 1 де i-й біт = включити i-й елемент.

```java
int n = nums.length;
for (int mask = 0; mask < (1 << n); mask++) {
    List<Integer> subset = new ArrayList<>();
    for (int i = 0; i < n; i++) {
        if ((mask >> i & 1) == 1) { // i-й біт встановлений
            subset.add(nums[i]);
        }
    }
    // обробляємо subset
}
```

### Задача 7: Subsets (LeetCode #78)

```java
public List<List<Integer>> subsets(int[] nums) {
    List<List<Integer>> result = new ArrayList<>();
    int n = nums.length;

    for (int mask = 0; mask < (1 << n); mask++) {
        List<Integer> subset = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            if ((mask >> i & 1) == 1) subset.add(nums[i]);
        }
        result.add(subset);
    }

    return result;
}
```

> 💡 Для n ≤ 20 bit mask підхід практичний.  
> Для більшого n — використовуй backtracking (розділ 13).

---

## 📐 Патерн 5: Math — GCD, LCM, Primes

### GCD (Euclidean Algorithm)

```java
int gcd(int a, int b) {
    return b == 0 ? a : gcd(b, a % b);
}

// LCM через GCD
int lcm(int a, int b) {
    return a / gcd(a, b) * b; // ділимо спочатку щоб уникнути overflow
}
```

### Sieve of Eratosthenes — всі простi до n

```java
boolean[] sieve(int n) {
    boolean[] isPrime = new boolean[n + 1];
    Arrays.fill(isPrime, true);
    isPrime[0] = isPrime[1] = false;

    for (int i = 2; i * i <= n; i++) {
        if (isPrime[i]) {
            for (int j = i * i; j <= n; j += i) {
                isPrime[j] = false; // прибираємо кратні
            }
        }
    }

    return isPrime;
}
```

### Задача 8: Count Primes (LeetCode #204)

```java
public int countPrimes(int n) {
    if (n <= 2) return 0;

    boolean[] notPrime = new boolean[n];
    int count = 0;

    for (int i = 2; i < n; i++) {
        if (!notPrime[i]) {
            count++;
            for (long j = (long) i * i; j < n; j += i) {
                notPrime[(int) j] = true;
            }
        }
    }

    return count;
}
```
**Складність:** O(n log log n) time

---

## 📐 Патерн 6: Math Simulation

### Задача 9: Happy Number (LeetCode #202)
**Умова:** Число щасливе якщо сума квадратів цифр → 1. Знайти чи є число щасливим.

**Ключова думка:** Якщо не щасливе — цикл → Fast & Slow pointers.

```java
public boolean isHappy(int n) {
    int slow = n;
    int fast = sumOfSquares(n);

    while (fast != 1 && slow != fast) {
        slow = sumOfSquares(slow);
        fast = sumOfSquares(sumOfSquares(fast));
    }

    return fast == 1;
}

private int sumOfSquares(int n) {
    int sum = 0;
    while (n > 0) {
        int digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    return sum;
}
```

---

### Задача 10: Excel Sheet Column Number (LeetCode #171)
**Умова:** "AB" → 28 (система числення з основою 26).

```java
public int titleToNumber(String columnTitle) {
    int result = 0;
    for (char c : columnTitle.toCharArray()) {
        result = result * 26 + (c - 'A' + 1);
    }
    return result;
}
```

---

### Задача 11: Pow(x, n) (LeetCode #50) — Fast Power
**Ключова думка:** Швидке піднесення до степеня — ділимо степінь навпіл.

```java
public double myPow(double x, int n) {
    long N = n; // int може бути Integer.MIN_VALUE
    if (N < 0) { x = 1.0 / x; N = -N; }
    return fastPow(x, N);
}

private double fastPow(double x, long n) {
    if (n == 0) return 1.0;

    double half = fastPow(x, n / 2);

    if (n % 2 == 0) return half * half;
    else return half * half * x;
}
```
**Складність:** O(log n)

---

### Задача 12: Modular Arithmetic
**Коли використовувати:** "Відповідь по модулю 10^9+7" — великі числа.

```java
final int MOD = 1_000_000_007;

// Завжди застосовуй MOD при множенні і додаванні
long result = ((long) a * b) % MOD;
result = (result + c) % MOD;

// ВАЖЛИВО: довжина типу int може переповнитись при множенні
// тому приводь до long перед множенням:
long product = (long) a * b % MOD; // правильно
int product = a * b % MOD;         // неправильно! overflow якщо a,b ~ 10^9
```

---

## 🗺️ Вибір патерну — дерево рішень

```
Задача на Bits / Math
│
├── "find unique / single number"?
│   └── XOR (всі пари скасовуються)
│
├── "count 1 bits" / "check power of 2"?
│   └── n & (n-1) — видаляє найправіший біт
│
├── "all subsets" (n ≤ 20)?
│   └── Bit Mask 0..(2^n - 1)
│
├── "GCD / LCM"?
│   └── Euclidean: gcd(a, b) = gcd(b, a % b)
│
├── "prime numbers up to n"?
│   └── Sieve of Eratosthenes O(n log log n)
│
├── "fast power" x^n?
│   └── Divide & Conquer: half * half (або half * half * x)
│
└── "result modulo 10^9+7"?
    └── (long) a * b % MOD при кожній операції
```

---

## ⚠️ Типові помилки

| Помилка | Правильно |
|---|---|
| `n & n-1` без дужок | `n & (n - 1)` — пріоритет операторів |
| Перевірка `n == 0` замість `n > 0` у isPowerOfTwo | `n > 0 && (n & (n-1)) == 0` |
| `int * int` перед `% MOD` → overflow | `(long) a * b % MOD` |
| `n = Integer.MIN_VALUE` при `-n` у Pow | Конвертувати до `long N = n` перед нагацією |
| `i * i` у Sieve → overflow для великих n | `(long) i * i <= n` |
| XOR: не враховувати що `a^a=0` тільки при парній кількості | Переконайся що кожен елемент крім одного з'являється рівно двічі |

---

## 📝 Список задач для практики

### Must Solve (Junior Strong)
- [ ] #136 Single Number
- [ ] #191 Number of 1 Bits
- [ ] #338 Counting Bits
- [ ] #268 Missing Number
- [ ] #231 Power of Two
- [ ] #204 Count Primes

### Should Solve (Middle)
- [ ] #260 Single Number III
- [ ] #78 Subsets (bit mask варіант)
- [ ] #190 Reverse Bits
- [ ] #50 Pow(x, n)
- [ ] #202 Happy Number
- [ ] #342 Power of Four

### Stretch Goals
- [ ] #371 Sum of Two Integers (без + і -)
- [ ] #137 Single Number II
- [ ] #201 Bitwise AND of Numbers Range
