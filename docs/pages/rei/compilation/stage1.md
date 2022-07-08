---
layout: page
title: Stage 1 - Lexical Analysis
parent: Compilation
grand_parent: Rei
nav_order: 1
---

## Overview

All about the first steps that compilers usually take to compile a string.

We usually tokenise and parse a long string (e.g. combining files into namespace {} mod_strings automatically) to build something that can then be transformed into a lower level representation. This IR is closer to machine code and can be further analyzed and optimised, e.g. LLVM. Finally it can be assembled into actual machine code for a certain platform, e.g. x86_64 linux elf.

For the first stage, lets look at lexers (tokenisation) which is a whole shabang of automata and regex stuff.

Lexical analyzers do two things: Scanning and Lexing. They basically tokenise a fixed sequence of input data into a list of key: value pairs representing a list directives/instructions.

- some directives can be resolved at compile time, like `let x = 5`. Some cannot, like `fn(x: Int) -> none => this.y = x` which usually gets compiled into operations on identifiers on the stack and heap instead of immediates/.data values
- adding things like `static` and `const` makes things even more interesting and uses `.rodata` and may require prev rules to be changed

Token -> pair consisting of a label and an attribute value. The label is an abstract symbol that represents a lexical unit. E.g. a keyword, identifier, etc

Pattern -> description of the form that lexemes of a token can take. Simply a sequence of chars that form the keyword

Lexeme -> sequence of chars in the source that matches the pattern for a token. Can be identified by the analyzer as an instance of that token

| Token | Description | Examples |
| --- | --- | --- |
| let | `reg"let"` | `let` |
| identifier | `reg"[\w\W]{1}[\w\W\d]+"` | `val`, `student_id` |
| numeric | `reg"\d{0-1}\.{0-1}+\d+"` | 0, 4.055215, -90251 |
| literal string (with newlines) | `reg"\\"[.*\n]+\\""`| "Hello, World!" |

So 1 token represents all its identifiers, though one or more tokens can represent a constant

- we need some way to prioritise certain tokens over the other

### Attributes

If more than a single lexeme can match a pattern, the analyzer must provide extra info about that lexeme to the next phase. E.g. if we have `0` and `1`, we need the code generator to know which lexeme was found, hence we would turn back to the parser

- tokens have at most a single associated attribute. This attribute can have several substructures
- the most important is the token `identifier` which we need to associate with a lot of info. `identifier` tokens should be kept in a symbol table, which contains its lexeme, type, location
- so for `identifier` tokens, we assign a `symbtab_pointer` attribute to it since it has a bunch of important subinfo stored in a global container

Example: `E = M * C ** 2`

- the lexed token: attribute pairs

```
<id, pointer to local symtab for E>
<assign_op>
<id, pointer to local symtab for M>
<mult_op>
<id, pointer to local symtab for C>
<exp_op>
<numeric, int32 value 2>
```

Note how there is no way for the lexer to tell if a certain sequence is 'correct'. If you have in rei:

```
go die (person: Person) {
    person.dead()
}
```

we have several things:

- `go` and `die` get transformed into `identifier` tokens, which dont mean anything, and is nowhere close to a correct function declaration
- how do we know type `Person` exists with a method `@instancemethod fn dead()`?

The answer is we dont. The lexer's job is to produce tokens -> label: attribute pairs of 'preliminary' meaning within the syntax itself or "local sequences", not check for semantics. To do that, we would have to pass them onto the parser that checks for meaning from the entire syntax and context for the list of tokens

## Error Recovery

Maybe a sequence of characters makes the analyzer unable to proceed since none of the patterns for tokens matches any prefix of the reamining input. This can happen, but idk how. To recover we can:

- delete successive characters until the analyzer can find a well formed token
- delete a single character from the reamining input
- insert a missing character into the remaining input
- replace a character by another character
- transpose two adjacent characters

We can try these transformations to repair the input sequence. Easiest way is to whether a prefix of the remaining input can be transformed into a valid lexeme by a single transform. But we want a more general strategy to find the smallest number of transformations needed to convert the source code into one that consists only of valid lexemes -> would be quite expensive, but languages like rust usually implement a small fixer that works for the first error only and give an error message like "undefined token X did you mean Y?"

Example decomposition of Rei:

```
// return the value of a result
fn get_val(res: Result) -> Val {
    return res.val()
}
```

```
<comment>

<function_decl>
<identifier symtab pointer to get_val>
<bracket_left>
<identifier symtab pointer to res>
<colon_operator>
<identifier symtab pointer to Result>
<bracket_right>
<right_arrow>
<identifier symtab pointer to Val>
<brace_left>

<return>
<identifier symtab pointer to res>
<dot_operator>
<identifier symtab pointer to val>
<bracket_left>
<bracket_right>

<brace_right>
```

Example decomposition of HTML:

```html
Here is a photo of <B>my house</B>:
<P><IMG SRC = "house.gif"><BR>
See <A HREF = "morePix.html">More Pictures</A> if you
liked that one.<P>
```

```
<literal "Here is a photo of">
<tag_B_begin>
<literal "my house">
<tag_B_end>
<literal ":">
<tag_P_begin>
<tag_IMG_begin>
<attr_SRC>
<equals_operator>
<literal "house.gif">
<tag_BR_begin>
<literal "See ">
<tag_A_begin>
<attr_HREF>
<equals_operator>
<literal "morePix.html">
<literal "More Pictures">
<tag_A_end>
<literal " if you liked that one.">
<tag_P_end>
```

### Input Buffering

We should buffer the inputs to know for sure the most encapsulated lexeme. The most prioritised token should be mapped to a certain lexeme

- we cannot be sure a sequence is an `identifier` until we see the end of the character that does not belong to the `identifier` pattern. So space, operator, etc
- we cannot be sure a single character operator like `=` doesnt actually mean `==` when we look at the next character. Also non existent operators like `===` which should bring about an error or `unidentified` tag

### Buffer Pairs

We start with two lexeme buffers that are reloaded one after the other. Each buffer is of size `N`, e,g, a page size like 4KiB. So we can read a disk block into RAM at a time. Two virtual pages are mapped to these buffers which the analyzer can use to `read()`

- we know that a page contains the end of the source file if it contains `eof` or is less than size `N`

We then keep 2 pointers, `lexeme_begin`, `forward`. These point to the beginning of the current lexeme and a scanner that keeps moving forward until a pattern match is found

So when the next lexeme is determined, `forward` is set to the character at its right end. We record that lexeme as an attribute value of a token returned by the parser. And set `lexeme_begin` to the character right after `forward`

- we will never overwrite the lexeme in its buffer before determining it. As long as we dont look ahead of the current lexeme so that L + D > N

### Sentinel

The sentinel character is a special char that cannot be part of the source program. We usually use `eof`. We combine the buffer-end test with the test for the current character by making each buffer hold `eof` at the end. If we moved off the current buffer, we must load the next buffer

### Specification of Tokens

Lexemes should be short like in most languages. Usually one-two character lookahead is good enough

- so a buffer size N = 4096 is ample, though some problems like character string literals may extend over many lines. This means the lexeme can be longer than N
- to solve this, we can treat them as a concatenation of each line. We can simply use something like the `+` operator before each line end to concat the next line's string, and do so in the lexer program
- but when arbitrarily long lookaheads are needed, you can treat keywords like `fn` as identifiers rather than actual keywords. Then let the parser resolve the full meaning in conjunction with a namespace level symbol table

Lexical analysers are basically regex engines that convert regular expressions into algorithms that perform token recognition

Let us look at an alphabet, which is a finite set of symbols. ASCII is an important set that we can use to build a programming language. Now consider a lookahead code:

```
switch forward {
    case eof:
        if foward.index == buffer_1[N-1]:
            reload buffer_2 // call read() on the next sequence
            forward = buffer_2[0]
        elif foward.index == buffer_2[N-1]:
            reload buffer_1
            forward = buffer_1[0]
        else:
            exit
    case 'A'
    ... // other chars
}
```

- multiway branches like the switch statement can be compiled in a single shot by simply storing each case's address in a lookup table and calculating a perfect hash function (randomised) for it

$\epsilon$ -> empty string \
$s$ -> non-empty string \
$|s|$ -> length of $s$

So a `language` is any countable set of strings `s` over some fixed alphabet `A`.

### Parts of a String

`prefix` -> any string obtained by removing $\geq 0$ symbols from the end of `s` \
`suffix` -> from the start \
`substring` -> deleting any prefix and any suffix from `s` \
`proper` prefix or suffix -> results that are not $\epsilon$ or `s` itself \
`subsequence` -> delete $\geq 0$ symbols that are not necessarily consecutive

To concatenate strings, we represent them like `ss`

### Operations on Languages

Note position does matter in a string, so we talk about permutations on strings.

A bunch of set theory like unions and closures. Know that $L^4$ means set of all 4-letter strings in language $L$.

L U D means set of letters and digits. 62 in the English Language.

L* means set of all strings of letters. So if you can imagine an infinite set of strings that contain all the possible letter permutations.

L(L U D)*-> (L U D)* means the set of strings containing all possible letter and digit permutations. L means set of letters. So the whole thing means strings with a letter in the first position and any letter+digit permutation afterwards.

D+ is the set of all strings with $\geq 1$ digit

## Regex

Each regular expression $r$ represents a language $L(r)$. $r$ can then be defined recursively by $r$'s subexpressions

Most languages have identifiers that begin with a letter can contain letters, numbers and underscores:

r"[\l_][\l_\d]"

$\epsilon$ is a regular expression of "" and $L(\epsilon) = \{\epsilon\}$ \
if $a$ is a symbol in `ALPH` then $a$ is a regular expression. This also means $L(a) = \{a\}$ -> same idea as $\epsilon$

- $(r)|(s)$ is a regex denoting the language $L(r) \cup L(s)$
- $(r)(s)$ denotes $L(r)L(s)$ concatenated alternatives
- $(r)*$ denotes $L(r)^*$
- $(r)$ denotes $L(r)$ -> basically we can add additional pairs of brackets without changing the language it represents

### Order of precendence

\* > concat > |

### Extensions of Regex

- `+` -> one or more instances of the previous group. Called the 'positive' closure `(r)+` denoting `L(r)+`
- `?` -> zero or more instances
- $a_i$  -> character classes where a_1 | a_2 ... a_n can be replaced with `[a_1a_2...a_n]`. So `[abc]` means `a | b | c`

### Regular Definitions

We can give names to certain regular expressions and use their names in subsequent expressions. We can also define the set of values which the regex maps to

Examples:

`letter -> A | B | C ... | Z | a | b | ... | z |`\
`digit -> 0 | 1 | ... | 9`\
`underscore -> _`\
`identifier -> (letter|underscore)(letter|underscore|digit)*`

Example of optionals:

`optional_fraction -> .digit+|eps`\
`optional_exp -> (E(+|-|eps)digit+)|eps`\
`number -> digit+ optional_fraction optional_exponent`

Note the spaces shouldnt important in the this case. `1.1` should be the same as `1 . 1` and `1E+10` is the same as `1 E + 10`. But they are, so only the no space ones are taken, i.e. `1.1` and `1E+10` for simplicity sake

## How to recognise Tokens

So how to take the patterns for all the needed tokens and built a piece of code that examines the input string. And finds a prefix that is a lexeme matching one of the patterns?

Example for branching statements:

```
statement -> if expr then statement
        |     if expr then statement else statement
        |     epsilon
expression -> term relop term
        |     term
term  -> identifier
        |     number
```

`relop` -> use the comparison operators of languages like Pascal or SQL where = is "equals" and <> is "not equals"

Terminals of the grammar:

- `if`
- `then`
- `else`
- `relop` (relational operator)
- `identifier`
- `number`

We can then describe these tokens as patterns:

```
digit -> [0-9]
digits -> digit+
number -> digits(.digits)?(E[+-]?digits)?
letter -> [A-Za-z]
id -> letter(letter|digit)
if -> if
then -> then
else -> else
relop -> < | > | <= | >= | = | <>
```

The lexical analyser recognises the keywords `if, then, else` and lexemes that match `id, relop, number`. We can also say that the keywords themselves (proper string) are not identifiers, i.e. they are "reserved words"

Then we assign the lexer the job of stripping out whitespaces by recognising the pseudo token `ws`:

```
ws -> (blank|tab|newline)+
```

So `ws` is a special token that we use for the lexer only, and do not pass to the parser. It is used for recognising lexemes

- For the 6 relational operators, symbolic constants `LE`, `LT`, etc. are used for the attribute value. This allows us to indicate which instance of the token `relop` we found without ambiguity

| Lexeme | Token | Attribute |
| --- | --- | --- |
| any `ws` | - | - |
| if | if | - |
| then | then | - |
| else | else | - |
| any `id` | id | symtab pointer |
| any `number` | number | symtab pointer (for immediates) |
| < | relop | LT |
...

### Transition Diagram

It is helpful to fist convert each pattern into stylised flowcharts called TDs. TDs have a collection of nodes/circles called 'states'

- each state represents a condition that could occur during the process of scanning the input looking for a lexeme that matches one of several patterns
- a state basically summarises all we need to know about what characters we've seen between `lexeme_begin` and `forward`

Edges between states represent transitions. They are labeled by a symbol or a set of symbols

- if in state `s` and the next input symbol is `a`, we look for a possible edge `a` out of `s`. If `a` exists, advance `forward` and enter the state of the diagram to that node

Assume that our TD is deterministic. So there is never more than one edge out of a given state. As long as that state has a given symbol among its labels

- this isnt strictly necessary

If a state is `final` or `accepting`, then a lexeme has been found. The actual lexeme might not consist of all positions between `lexeme_begin` and `forward`

- an accepting state is a double circle instead of a single circle
- we can attach an action like returning a token : attribute pair to the parser to the accepting state

If we have to retract the `forward` pointer one position back, then we place a `*` near that accepting state. In the above patterns, we dont have to retract more than once

- if we did then we simply just attach `n` * for `n` steps backward to that nearest accepting state

The `start` or `initial` state is indicated by an edge labeled `start` entering from nowhere

### Example: TD for relop

We start at state `0` from nowhere. From there, we can transition to 3 different states:

- state 1 `<`
- state 5 `=` : `EQ`
- state 6 `>`

This means if we get to states 1 and 6, there are more options depending on subsequent characters. But if we see `=` from nothing, then we know for sure that can only be recognised as the EQ operator

State 1 has 3 more transitions, all of which are accepting

- `=` : LE
- `>` : NE
- `*` : LT

As you can see from our specified relop patterns, this makes a lot of sense since there are no other ways to tokenise

- if we started at state 0 and did not see a valid transition, then that means our current TD we are using is wrong. So we could e.g., use another one. This doesnt apply after state 0 since `*` captures everything before the next `ws`

### Recognising reserved words vs identifiers

This is a key problem. `if` and `then` are usually reserved and if found by themselves, should be treated as reserved words rather than `identifiers`. So the problem is they can be treated as both

- in our reserved word and id TD, we start from nowhere and have a single possible transition to state 10 if we see a `letter`

Now heres the thing. We loop the edge of state 10 to itself if we see a `letter|digit`. If we see something else, we return the lexeme with the functions `get_token()` and `install_id()`

- but how do we handle reserved words that look like an `id`? Well it all lies in `get_token` and `install_id`. If there is no matching entry, then the token must be an ordinary `id`

Method 1: Install the reserved words in the symtab beforehand. A field of the symtab indicates that these strings are not ordinary `identifiers`. Then `get_token()` examines the symtab entry for the lexeme found, and returns that value

Method 2: Create separate TDs for each keyword. So we prioritse the tokens so that the reserved word tokens are recognised in preference to `id`. This is not a great idea

### Example: TD for Numbers

Numbers are usually the more complex ones since we have optionals and a lot of stuff

- it is still possible to create a transition diagram

Note a TD for whitespaces is quite simple. We have a single possible transition from start if we see a `ws` char. Then we loop back onto the same node if we see another `ws` until we dont. Then we can terminate

## Building a TD-based Lexical Analyser

We can actually build a lexical analyzer from a bunch of different transition diagrams

- the key thing to see is that each state is represented by a piece of code. So a variable `state` holds the number of the current state for a TD. Then we have a bunch of states to choose from depending on the current state object and the next character
- often the code for a state itself is simply a switch statement or multiway branch that determines the next state by examining the next input char

### Simulating Relop TD

```
fn get_relop() -> Token {
    let ret_token = Relop()
    let state = 0
    let c = ''

    loop {
        switch state {
            0 => {
                c = next_char()
                if c == '<': state = 1
                elif c == '=': state = 5
                elif c == '>': state = 6
                else fail()
            },
            1 => {}
            ...
            8 => {
                // retract the forward pointer and return
                // so the next time it can start from 'nowhere'
                retract()
                ret_token.attribute = Attribute::GT
                return ret_token
            }
        }
    }
}
```

`fail()` could initiate an error correction phase that will try to repair the input if there is no other unused TD that works on the current sequence

- state 8 bears `*` so we must retract the `forward` pointer one position

Now you can see how this fits into the whole lexer:

- arrange for the TDs for each token to be tried sequentially. Then `fail()` resets `forward` to `lexeme_begin` and starts the next TD. So we can use TDs for individual keywords. Only have to use these before we use the diagram for `id` for keywords too, which would be great
- OR we could run the various TDs in parallel. So we feed the next input character to all TDs on their separate threads. Then each of them can make whichever transitions they wanted. But we must be careful when we have to resolve when one diagram finds a lexeme that matches but other diagrams yet to find a match. So we could take the longest prefix of the input that matches any pattern, and thus prefer an identifier token `thenext` instead of `then` or operator `->` to `-`. This should work almost 100% of the time by the sound of it
- OR the best approach. Combine all the TDs into one. Allow the TD to read input until there is no possible next state. Then take the longest lexeme that matches any pattern. The example patterns should be quite easy. We could combine states `0,9,12,22` into a single state `valid_char` and leave other transitions intact

BUT: it is quite a bit more complex to combine TDs for several tokens

## Lex

`flex` is an implementation of a UNIX tool `lex` for specifying regular expressions to ultimately generate a lexical analyser

- transforms input patterns into a single TD. Then generates the code for it in `lex.yy.c` that simulates that TD

First specify an input `.l` file in the lex syntax. The resulting program `a.out` takes in string sequences and outputs tokens. These tokens can then be fed into a parser (like bison)

- attribute values for each token are placed in a global structure `yylval`. This can be shared between the lexical analyser and parser

### Structure of a Lex Program

```
declarations
%%
translation rules
%%
supporting functions
```

The declarations portion should be used for declaring variables, 'manifest constants' and regular definitions.
The translation rules have the form:

```
Pattern { Action }
```

Patterns are regular expressions. Actions are C code. Patterns may use declared variables.
The supporting functions can be used in the translation rule actions. Or they can be compiled separately and used in the compiled C file

When we create a lexical analyser instance, it should interact with the parser. How it does this:

- when called by the parser, the lexical analyser code begins reading its remaining input 1 char at a time. It stops when it finds the longest prefix of the input that matches one of the patterns `P_i`
- then it executes the associated Action code `A_i`. It then returns to the parser
- if it does not, then maybe the lexer has found a `ws` or something. Then it will proceed to find additional lexemes until one of the corresponding actions causes a return to the parser
- the lexical analyser returns a single value: the token name to the parser. It uses the global `yylval` to pass addition info about the lexeme like attributes

Basically, it works like a single TD

```
IF, LT, THEN ...

delim [ \t\n]
ws {delim}+
letter [A-Za-z]
digit [0-9]
id {letter}({letter}|{digit})*
number {digit}+(\.{digit}+)?(E[+-]?{digit}+)?

%%

{ws} { }
if {return(IF);}
...
{id} { yylval = (int) installID(); return(ID); }
{number} { yylval = (int) installID(); return(NUMBER); }
"<" { yylval = LT; return(RELOP); } 

%%

int installID() {}
int installNum() {}
```

### Prioritisation and Conflict resolution

So when lex encounters several prefixes of the input that match one or more patterns:

- always prefers a longer prefix to a shorter prefix
- if longest possible prefix matches 2 or more patterns, prefer the pattern listed first

### Lookahead Operator

Lex automatically reeads one char ahead of the last char that forms the selected lexeme, as expected

- then retracts the input so only the lexeme itself is consumed in the input string
- but what if we want to "look ahead", i.e. for patterns that should only be matched when a previous pattern had been found
- so we use the `/` operator. In regex, this is also `?!` for negative and `?=` for positive

How `/` works:

- what follows `/` is an additional pattern that must be matched before we can decide that the token in question was seen. But the second pattern does not form part of the lexeme

### Example: Unreserved Identifiers

If we have:

```
IF(I,J) = 3
```

as an array index operator, we may run into some problems. Typically we want something like:

```
IF(condition) THEN ... ENDIF
```

- Note we usually dont worry about the ENDIF part in the lexer. That would be the parser's job

But we can sure that an `IF` keyword is always followed by a left bracket, text describing a condition. But the text describing the condition can also be followed by identifiers, other brackets and operators. So to be very general:

```
IF/\(.*\){letter}
```

- the lexeme matches just the two letters `IF` iff the right stuff comes after it
- this can only match the keyword `IF` if the programmer coded correctly. If we're looking at an index operator with `id` `IF` then it cant match `{letter}` since operator `=` is not a letter
- Note we specified `letter` afterwards to be very general, could be part of `THEN` or something else. The braces are just to say we want to use `letter` instead of "letter"

`IF (A < (B + C) * D) THEN`

- whitespaces dont matter too much in this example
- note how the first left bracket matches before `A` and the first right bracket matches after `C`. In this case the lexeme matches well since we are looking for `\){letter}` which means `)*` doesnt match but `) T` matches

### Example: While Loop

Say we have something like:

```
while (condition) {

}
```

or

```
while (condition) statement
```

then we can use:

`while/\(.*\)[\{letter]`

Note it would be a bit of a problem if there was a new line `\n` within the condition or before/after the condition:

```
while
(condition) statement

// OR

while (condition_part_1
    condition_part_2) statement

// OR

while (
    condition
    )
    {
        // stuff
    }
```

And if there are features like subconditions within conditions, functions within conditions, etc. we can run into many issues just using the default pattern

- hence it is usually better to just make tokens like `if`, `while`, etc. completely reserved when used properly

## Finite Automata

Finite automata are 'reconigsers' that say Yes or No about each possible input string. They come in two ways:

- nondeterministic. No restrictions on labels of edges. A symbol can label several edges out of the same state. `epsilon` is an empty label
- deterministic. Each state and symbol has exactly 1 edge with that symbol leaving that state

Regular languages are languages that regular expressions can describe. It would be great to be able to make a regular language, though it can be hard if you want advanced and nested functionality. The fact that finite automata are able to recognise regular languages is really cool

## NFA

An NFA can be represented by a transition graph. Nodes are states and edges (labeled) represent the transition function

- an edge `a` from `s` to `t` exists iff `t` is one of the next states for `s` and input `a`

What is a TG? It is similar to a TD except:

- the same symbol can label edges from one state to several different states
- an edge may be labeled `epsilon`

So in a TG, we can go to more than one node in a step. If we are stuck in one node on the next input char, then we drop that path and keep going with the other paths. Like BFS

### Transition Tables

We can represent a TG with a table (2D array). You can see how this can be useful

- each row corresponds to 1 state, each col corresponds to the input symbol and `epsilon` (which needs to be always there in its own row)
- so we can have many input symbols, like A-Za-z0-9_, etc. and `ws` too. `epsilon` should be included automatically
- then we write each state out for each possible input char. Each index value is a set of states that the system should transition to when the state + input char is met on a cycle

## DFA

A DFA is a special case of NFA when:

- there are no moves on input `epsilon`
- for each state `s` and input char `a`, there is a single edge out of `s` labeled `a`

This means each entry is a single state transition, and no need to use set notation

- a simple, concrete algorithm for recognising strrings

Good thing is, every regular expression / NFA can be converted to a DFA. Since DFA simulates when building the lexical analyser rather than the analyser itself

### Example: Simulating a DFA

Like a TD, it can be quite simple and only needs to be single threaded (though exploiting parallelism can be good later on)

```
fn dfa_accept() -> bool {
    let s = State()
    let c = ''

    while (c = next_char()) != EOF {
        s = move(s, c)
    }

    return (s in States::Accepting)
}
```

- so the DFA will return `true` if it finds a lexeme that matches an accepting state. Note we dont have ways to set priorities yet, it only returns the first matched token. But we could reserve stuff as keywords and put them first. I think that makes sense

## Regex -> Automata

We specify a list of regular expressions, convert to an NFA. Then convert that into a DFA. Then we can simulate its behavior to get the resulting lexer

- the NFA -> DFA conversion can take quite a lot of time, even more so than the actual lexer program. There are various heuristics to consider and choose from to build a lexer that trades off speed for simplicity and etc. There doesnt seem to be superior way/one size fits all

### Conversion NFA -> DFA

It is possible that the number of DFA states is exponential in the number of NFA states. This could be bad, but usually for most languages the NFA and DFA have both the same amount of states, and so works pretty well on average

```
fn convert_to_dfa(DStates: NFA) -> DFA {
    let T = State()
    let DTransition = DFA()

    while DStates.unmarked_exists() {
        T = DStates.get_unmarked_state()
        T.mark()

        for _a in input_chars {
            let u = epsilon_closure(move(T, _a))
            if u not in DStates {
                DStates.add(u)
            }
            DTransition[T,a] = u
        }
    }

    return DTransition
}

fn epsilon_closure(nfa_states_t: Set<State>) -> EpsClosure {
    let stack = Stack<State>()
    stack.push_all(nfa_states_t)

    let nfa_states_t = EpsClosure()
    while not stack.empty() {
        let t = stack.pop()
        for u in t.edges(label="eps") {
            if u not in nfa_states_t {
                nfa_states_t.add(u)
                stack.push(u)
            }
        }
    }

    return nfa_states_t
}
```

There are also algorithms to simulate the NFA. Overall the time complexity is O(k(n+m))

- k = length of input
- n = number of nodes
- m = number of eedges

## Regex -> NFA

We can convert any regex to an NFA through the MYT algorithm

- uses a parse tree as it is 'syntax directed'
- each subexpression, the algorithm makes a new NFA with an accepting state

Algorithm:

1. parse `r` into its constituent subexpressins
2. for expression `epsilon` construct i -> f transition where `f` is an accepting state
3. for any subexpression `a` do the same thing with the edge labeled `a`

### Parse Tree

This is a great structure

![](/assets/img/rei/ParseTree.png)

- as you can see, we enter from nowhere into r3. Then we branch to r1, r2, or r4 depending on `a|b` found or something else. Then the rest is history

## Design of a Lexical Analyser Generator

Something like `lex` is designed through DFAs

- we begin with a lex specification that gets compiled into lexer program
- run the program with a transition table and actions based on each pattern. This interacts with the DFA simulator
- the DFA simulator does most of the work of stepping along and recognising the tokens

So we each regex pattern and convert them to NFAs. Combine them all into one by introducing a new start state with `epsilon` transitions for each state

The current 'best' or recommended way is a time-good space-bad DFA simulation algorithm. We use a 2D array to store a single transition state for each entry

State minimisation seems to work for O(nlogn) time and quite space efficient. IDK tbh, I dont really want to implement this myself though later maybe

- Important thing to note is that for every DFA is there a minimum state DFA accepting the same language. The min state DFA for a given language is unique except the names given to the various states
