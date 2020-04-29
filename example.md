# Simple examples

Some examples. Pretty boring to put to readme.

## Hello World

A timeless classic.

```txt
"Hello world"
```

## Boolean Logic

Here are some simple example regarding boolean logic, it accepts either `1` or `0` or maybe more, like `01`.

```txt
Not: only accepts `1` or `0`
"10"[_:][:"1"]

Or: 2 binary digits
((_[:"1"])+"1")[_["1":]:][:"1"]

And: 2 binary digits
("0"+(_[:"1"]))[_["1":]:][:"1"]

Xor: 2 binary digits
((_[:"1"])+("10"[_[:"1"]:][:"1"]))[_["1":]:][:"1"]

Display: 1 binary digit
$"\"truefalse\"["+(":"[:_])+"\"4\""+(":"[_:])+"]"
```
