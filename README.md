# ***The Super Tiny Compiler***

An ultra-simplified example of all the major pieces of a modern compiler.

This is a Rust port of the [the-super-tiny-compiler](https://github.com/thejameskyle/the-super-tiny-compiler) written by @thejameskyle in JavaScript.

We're going to compile some `lisp`-like function calls into some `C`-like
function calls.

<table>
<tr>
<td> Lisp (Before) </td> <td> C (After) </td>
</tr>
<tr>
<td>

```lisp
(add 2 2)
(subtract 4 2)
(add 2 (subtract 4 2))
```

</td>
<td>
    
```c++
add(2, 2)
subtract(4, 2)
add(2, subtract(4, 2))
```
</td>
</tr>
</table>

*While this is neither a complete LISP or C syntax, it will be enough of the syntax to demonstrate many of the major pieces of a modern compiler*:
___ 
Parsing (Lexical Analysis &rarr; Syntactic Analysis) &rarr; Transformation &rarr; Code Generation
___