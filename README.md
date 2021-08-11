# Mini Language
A minimal, experimental language for lazy evaluation.

## Syntax
These are the syntax for Mini Language.
(Actually, the `//` comment is not allowed.)

### Numerical Literal
```scala
print 42      // The integrer number
print -42     // Plus and Minus signs are allowed
print + 42   // and you can insert spaces between them.
```

### Operations
```scala
// Arithmetic
print 3 + 2    // 5
print 3 - 2    // 1
print 3 * 2    // 6
print 28 / 5   // 5 (floored)
print 28 % 5   // 3

// Comparison
print 2 > 1    // 1 (true)
print 1 > 1    // 0 (false)
print 2 >= 1   // 1
print 1 >= 1   // 1
print 2 < 1    // 0
print 1 < 1    // 0
print 2 <= 1   // 0
print 1 <= 1   // 1

// Equal, Not equal
print 2 == 1   // 0
print 1 == 1   // 1
print 2 != 1   // 1
print 1 != 1   // 0

// Conditional branch
print if 3 > 2 \   // 5
  then 5 \
  else 0 \
```

### Variables
```scala
let x = 5
let y = 3
print x + y   // 8
```

### Functions
```scala
// The simple add function
def add(x, y) = x + y

// Recursive function
def fibo(n) = \
  if n <= 2 \
    then n \
    else fibo(n-2) + fibo(n-1)

print add(3, 2)   // 5
print fibo(10)    // 55
```

## API
This language provides the rust library interface, and API documentations are
available by [docs.rs](https://docs.rs/mini-lang/).

## License
Cross Clip is licensed under the MIT license. See [LICENSE](https://github.com/watcol/mini-lang/blob/main/LICENSE) for details.
