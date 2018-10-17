---
title: Literals
permalink: /docs/literals/
---

Rose supports many literal types.

| Literal   | Sample values                                         |
|-----------|-------------------------------------------------------|
| Nil       | `()`, `nil`                                           |
| Infinity  | `Inf`, `∞`                                            |
| NaN       | `NaN`, `∞/∞`                                          |
| Bool      | `true`, `false`                                       |
| Integer   | `64`, `-20`                                           |
| Float     | `1.0`, `1.1e+01`                                      |
| Rational  | `1//2`, `5.0//2.0`                                    |
| Complex   | `2im`, `1 + 2im`                                      |
| Char      | `'a'`, `'\n'`, `'ॠ'`                                  |
| String    | `"foo"`, `` `foo` ``                                  |
| Symbol    | `:foo`, `:"foo"`                                      |
| Array     | `[ 1, 2, 3 ]`, `[] of Int32`                          |
| Hash      | `{ "foo" => 1 }`, `{} of String => Int32`             |
| Range     | `1..10`, `1...11`                                     |
| Sequence  | `1..2..11`, `1..2...12`                               |
| Regex     | `/foo?bar/`, `/\s*/`                                  |
| Struct    | `{ name: "Rose", type: "Lang" }`                      |
| Tuple     | `( "Rose", "Lang" )`                                  |
| Block     | `begin let a := 1 ; let b := 2 ; a + b end`           |
| Proc      | `( a : Int32, b : Int32 ) -> Int32 do a + b end`      |
| Type      | `type I32 = Int32`, `type Z = Integer`                |

