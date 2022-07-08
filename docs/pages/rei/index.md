---
layout: default
title: Rei
has_children: true
---

## Overview

Rei is a human-centric language for building systems and applications. As such it comes with a larger standard library, package management and building semantics, simplified and clean syntax without "junk".

Rei is designed and optimised for Neutron/Arc. Wasm64-Wasi targets also exist for Neutron/Quantii.

## Programming Languages

Theres stuff about parsers, semantic analysis, assembly and assemblers and executable formats like ELF. Also more theoretical stuff like LL(1), LR(1), parse trees/abstract parse trees, EBNF, priority of regex and tokens.

So i'll just put some stuff here as I go. Will be mostly from first principles of the maths

## Rei prototype

- no semicolons, not even optional. Language designed around good spacing/indentation and new lines
- optional braces for one-line statements e.g. `if`, replaced with colon
- `let` and `const` variable declaration semantics. By default all `let`s are mutable. If you dont want mutability, then use `const`
- `static` and `dynamic` lifetime modifiers
- high reliance on annotations for structuring, clealiness and metaprogramming
- supports async/await
- every field (class, function, variable, constant) in a namespace is `private` by default, use `@export` or `@export-default` per object
- doc comments and metacomments with `//` and `#`

```rust
let x: Int = 10

@export-default, template(T: B)
class A {
    construct() {
        self.__val = dynamic String("Class A")
    }

    construct(val: &const String) {
        self.__val = val.clone() // copy constructor of String
    }

    @ignore-warnings // suppress no-explicit-initialisation warning
    construct(val: Int) {
        // Warning! construct method does not explicitly initialise all fields: __val
        // will use default constructor
    }

    del() {
        delete self.__val
    }

    operator+(add_from: &Self) -> Self {
        return Self(&add_from.__val)
    }

    fn get_val() -> &String {
        return self.__val
    }

    let __val: String
    @skip // tell compiler not to include this field
    let b: B
}

// local-scope bound allocated
// when a goes out of scope, `delete a` is called (del() function is called)
let a = A()
// same as let a = new A. `dynamic` is a keyword in rei to mean heap allocated
// when the number of references to a = 0, a is deallocated (del() function is called)
let a = dynamic A()
// same as
@dynamic
let a = A()

fn accept_a(_a: A) => println!("ayy!") 

// to pass a as a reference, make sure to pass &a
// for functions that accept references/values
// otherwise will always pass by copy on the stack
// not like C++ where it automatically casts it into a reference for you

// pass by reference
accept_a(&a)

// pass by value
accept_a(a)

fn accept_a_ref(_a: &A) => println!("references only!")

// create a temporary value a, and pass that by reference
// doesnt actually affect this.a
accept_a_ref(a)

// pass by reference
accept_a_ref(&a)

// MORAL OF THE STORY: look at IDE annotations and autoparameter values closely
// to see how your passing
// always create a value, not a reference. Always define a parameter as a value, not a reference
// then just write &a once when you are calling the function
// NOTE: references are actually a type annotation. A variable with type T is just a variable
// A variable with type T and annotation Reference is a reference/symlink to that variable

// In String
class String {
    ...
    __str: *u8
    size: UInt64

    copy(copy_from: const& String) {
        this.size = copy_from.size
        this.__str = [u8; this.size]

        for (i in size.range()) {
            // value copy without context
            _str[i] = copy_from.__str[i]
        }
    }
}

// Defining an annotation
annotation example {
    // define what object types this applies to, e.g. class, variable, function

    // when used on a function, can hook onto functional patterns
    function => {
        // do something to the function, e.g. check the inputs and outputs before or after calling
        // with arrkia: can hook onto arrkia render cycles
    },

    // when used on a class, can hook onto creation, copy, destruction, method calls
    class => {
        on create:
            {
                // check constructor inputs
            }
    }

    // when used on a variable, can hook onto creation, destruction, updates
    var => {
        on update:
            {
                // check new value
                // e.g. @range(0, 100)
                // maybe if it is not in a certain range, raise an exception
            }
    }
}

// Annotation with arguments
// Highly recommended to make all labels meaningful, esp the params
annotation range(begin, end) {
    var<Numeric> => {
        on update:
            {
                if (self < begin || self > end) throw OUT_OF_RANGE
            }
    }
}

// Defining a macro
// Note should be in upper camel case
macro DoThis {
    () => {
        println("I did something")
    },
    (x: Numeric, y: Numeric) => {
        x + y
    }
    (x: String) => {
        println("String: $x")
    }
}

// deriving traits based on fields
// allow object to be directly compared by comparing each field using operator=
// usually objects should derive Eq and PartialEq, not implement operator= directly
@export, derive(PartialEq, Eq, Debug)
object AnObject {
    id: Int
}

// NOTE: classes should implement operator= directly. If all fields of a class implement operator=, then you can also @derive(Eq)
```

NOTES:

- for rei v1, we just stick to a single syntax and thus lexer-parser without needing to change things up too much, just extend. I hope there are no glaring issues. So we only make a release if there is no big issues
- if there are any big issues later on that are slowly found (or hopefully not found hardcore), then we can make rei v2 with a different lexer-parser
- a rei-prototype as a restricted context-free grammar for ease. No advanced stuff. Maybe annotations. But semicolons, braces, explicit return etc always

```rust
// rei prototype

// must specify input and outputs, and their types
fn function() -> Int {
    // must have a type annotation and terminal semicolon
    let i: Int = 3;

    return i;
}

static k: Int = 0;

class A {
    // a constructor method is always a constructor method
    // usable by A::constructor() only
    fn constructor() {}

    // a mutable field must have a type and terminated
    let i: Int = 4;

    // non defined field
    let j: Int;

    // method that explicitly takes &self as a reference
    fn a_method(&self) -> Int {
        return i;
    }


    // class level method
    fn another_method() -> Int {
        return k;
    }
}
```

## Build Automation

I like package managers like NPM, Pipenv, Cargo, Gem.

I dont like the stuff Java has. E.g. maven, gardle, etc. Which I think are too generic and although powerful, not focused and leads to bad code. If java tools are too generic, then Make, CMake, Meson, etc are way too generic. For those tools, just no. If you want to build a project with multiple different languages and unrelated functions though, then meson for sure.

Rei comes with ReiPM which is pretty much Cargo but with no extra formats like Toml or Yml. Just rei.

For larger projects and projects where you want to build many different unrelated things to link to each other in a custom way. Then you can use ReiPM's `metabuild` extension:

```rust
lib += metabuild
```

This allows you to then include `.prei` files. Which basically incorporates other project dirs with their own `.prei` files like CMakeLists.txt. In those `rproj` files you can then specify what language/custom scripts you need to build that proj. And how it relates to the main proj described by the top level `proj_name`.

This integrates nicely with ReiPM's `build.rei`. You can use both simultaneously and specify the order of build projects in the top level `.prei`. The top level rei project is then subservient to the RPROJ as its own RPROJ. To build an prei:

```bash
rproj build
```

which runs your build scripts in order. Including any downloading from source, compiling, linking, error checking.

### RProj

Stands for 'Rad Project'. Mostly a separate thing from ReiPM but integrates pretty well if you are looking to build a larger project / something with multiple different langs and concepts and custom scripts/linking.

Structure:

```bash
proj_name/
    proj_name.prei
    subproj/
        include/
        subproj.prei
        main.cpp
    rei_proj/
        build.rei
        src/
```

The example includes two separate projects. A rei project and a custom project (C++). We want to compile the C++ project first (cache if possible), then link it to our rei project via the `include` dir. So we have:

```rust
// subproj.prei
export_api(DEFAULT subproj/include)

// proj_name.prei

include_proj(subproj rei_proj)

// rei_proj/build.rei

lib += subproj
// basically the same as
// lib += subproj::DEFAULT
```

Rei supports C/C++ style headers. So any valid C/C++ text files can be used directly like any other rei interface:

```rust
// in include/api.h
class C {
public:
    int x;
    C(int x) : x(x) {}
}

// in main.rei

use subproj::C

fn main() {
    let c = C(1)
    println("c.x = ${c.x}")
}
```

For other languages though, I dont think I support them as much. C/C++ is easier cause you usually have header/source split. Rust is very similar to rei so it is supported.

A common way to write rei apps is to have the same dir structure as above. With the main controller logic as a rei project. Which includes a bunch of other subprojs in C, java, etc. And builds for neutron.

### RProj, ReiPM and Prei

Prei is simply Project Rei. ReiPM is a way to manage a rei project's deps and build functions as well as metadata.
Prei consists of RProj projects. `Prei` is more specifically a tool used to build RProjects. RProjects have an `.prei` extension and are used for non rei projects. Rei projects still use `.rei` build files.

### Precompiled Projects

You dont have to include the code if you can just precompile it. The only thing you really have to do is include the API code. In C/C++ its pretty straightforward. In java and its derivatives, there are `interface` blocks. And many other langs prob have some form of an FFI you can use. WASM/JS could prob just use a c style API as well.

To use precompiled libraries, you can download it into a `/lib` dir and interface with it via an `/include` or `/api` dir. By default, RProj sources a bunch of useful tools from github (mostly c/c++) and allows them to be included via enabling `common_libs`:

```rust
// a_proj.prei
common_libs = true

include_proj(common_libs::zip
             common_libs::assimp)

// subproj.rei
libs += common_libs::zip
        common_libs::assimp
```

As you see, if you downloaded / setup everything up right, you should be able to use them in rei seamlessly. Otherwise you may have to do some mumbo jumbo to get everything right. If you want to include a c/c++ proj in another non rei proj that might also be kinda problematic but it is supported, at least for a few specific languages.

### Warnings and Messages

RProj allows very powerful warnings and messages through `warn()` and `info()` functions.

### Build Scripts

In addition to including foreign projects, we can write scripts to build them or link them in a specific manner.

```rust
// top level build script

@reish
build {
    cd subproj1 && prei build && file build/include
    prei build
}
```

By default, subprojs are built in the order they are specified. It is recommended to place your build scripts in the level in which they are affected. Then link them together with a more generic build script at the top level. Also use the subproj ordering either manually or with `order`:

```rust
build_order {
    subproj1
    subproj2
    ...
}

// note the projs not specified are built in any order after the specified. To ensure a specific project is built before/after another, just use that build order block
```
