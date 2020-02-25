# Usage
```
dice 0.1.0

USAGE:
    dice.exe [OPTIONS] [dice-coll]...

OPTIONS:
    -a, --aggregate <aggregate>
            Optional aggregate function to apply to the collected rolls of a die.

            One of 'sum', 'avg', 'max', 'min'

ARGS:
    <dice-coll>...
            Dice to roll. Eg. "d6", "5d10" etc

```
# Example
```rust
> dice 5d6 2d12
5d6 2 4 2 1 2
2d12 10 1
```
```rust
> dice 10d50 -a sum
10d50 266
```