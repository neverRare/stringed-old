# Stringed

An expression-based esoteric programming language that works with REPL.

Just a little note, I'm ~~dumb~~ a beginner when it comes to making a programming language and stuffs, this project may not even be considered as programming language. Also, I just started using rust like few months ago, so the code quality may be bad. I'm working to make this improve and learn :D . I hope you'll like this project.

## Installation

Standalone binary isn't readily available yet. For now, you'll need Git and Cargo.

## Examples

```txt
"Hello, world!"
```

More boring examples at [example.md](./example.md). It will be added in the future, and less boring examples here in the readme. I'm working on it :)

## How it works

Stringed only have 1 data type: UTF-8 string, and it only have few operations. It is purely expression-based, the following are the possible expression.

| Syntax   | Name   | Note                                                              |
| -------- | ------ | ----------------------------------------------------------------- |
| `_`      | Input  |                                                                   |
| `"..."`  | String | A string literal with escape notation                             |
| `A+B`    | Concat | `A` and `B` is a valid expression                                 |
| `A[B:C]` | Slice  | `A`, `B`, and `C` is a valid expression, `B` and `C` are optional |
| `$A`     | Eval   | `A` is a valid expression, this syntax is greedy                  |
| `(A)`    | Group  | `A` is a valid expression                                         |

With the following precedence, the top having the highest precedence.

- Group
- Slice
- Concat
- Eval (greedy)

Stringed works with REPL. Everything is evaluated and outputted.

> In the current implementation (this), slice and concat have equal precedence and evaluated left to right. This will be changed in the future.

### String

Pretty much like string literals but it have very simple escaping, it only escapes `"` and `\`. It do escape other characters but it almost does nothing, such as `\n` is interpreted as `n` instead of a line feed.

```txt
"string"
```

### Concat

This concatenates the string. I hope this explanation helps.

```txt
"concat" + "enation"
```

### Slice

This copies a portion of the string. Each bounds are converted from string to number, it can only be a positive integer or a zero.

It emits an error whenever the bounds can't be converted, the lower bound is greater than the upper bound, or the upper bound is greater than the length of the original string.

The bounds are optional. For lower bound, it will default to 0. For upper bound, it is the length of the original string.

```txt
"heartful"["2":"5"]
```

### Eval

This evaluates the string. It emits syntax error or whatever the evaluation would emit.

```txt
$"\"eval\" + \"uation\""
```

### Group

This groups an expression to better control the order of evaluation.

```txt
("heart" + "ful")["2":"5"]
```

### Input

The only existing variable of this language. For REPL, if it is provided, the program enters a new loop in which the input variable is evaluates to whatever the string is inputted.

```txt
> "hand"+_
accepting inputs...
> some prince
= handsome prince
> ling a
= handling a
> ful of pickles
= handful of pickles
```
