---
layout: page
title: Stage 3 - IR Generation
parent: Compilation
grand_parent: Rei
nav_order: 3
---

## Overview

We are now at the next end. The middle end of a compiler. Here we can convert a parse tree into a language and machine independent representation. Which we can then use for analysis and optimisation in the backend.

Static checking of your code for compile-time errors is done here. Meaning you cant have a statement like `let x: String = 3`. Which would raise a static error when we build the parse tree and symbol table `x = 3` where `x: String`. We do that analysis with the IR.

## Variants of ASTs

A node represents a construct in the source program. The children represent the meaningful components of the construct.

A DAG for an expression identifies the common subexpr of the expr. DAGs can be constructed by using the same techniques that construct syntax trees.

### DAG for Expressions

A DAG has leaf nodes corresponding to atomic operands (tokens) and interior nodes correspond to operators. The difference is that a node N has more than one parent P if N represents a common subexpr. In a syntax tree, the tree for the common subexpr would be replicated as many times as the subexpr appears in the original expr.

So a DAG not only represents expressions more succintly. It gives the compiler important clues to generate efficient code to evaluate the expressions.

Instead of an AST, we would build a DAG when parsing. Where the leaf nodes have the most precedence. And the root expr is done last.
