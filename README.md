# Hexo | 10110

![Crates.io Total Downloads](https://img.shields.io/crates/v/hexo?label=version)
![Crates.io Total Downloads](https://img.shields.io/crates/d/hexo?logo=rust&label=crates.io%20downloads)
![GitHub top language](https://img.shields.io/github/languages/top/lexa-diky/hexo?logo=rust)

![Crates.io License](https://img.shields.io/crates/l/hexo?logo=apache)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lexa-diky/hexo/build?branch=main&logo=github)

Tiny binary writer utility, _just enough for you_

## Installation

```bash
cargo install hexo
```

## CLI

### build

Takes `source` file in hexo format and compiles it to binary file `output`

```bash
hexo build --source <path to source> --output <path to output>
```

### watch

Takes `source` file in hexo format and compiles it to binary file `output`. Will recompile on `source` file change

```bash
hexo watch --source <path to source> --output <path to output>
```


## Syntax

### Emitter

To emit a byte use glyph '>' fallowed by byte value:

```hexo
> 0a // by default numbers are interpreted as hexadecimal, will emit decimal 10
> 'HelloWorld' // will emit utf-8 bytes of 'HelloWorld' string
> 10x22 // you can specifiy arbitrary radix in range 2..36, will emit decimal 22
```

### Constants

To declare a constant use glyph '$' fallowed by constant name and value:

```hexo
$ class_name 'HelloWorld'
```

Then you can use it as if you used hex or binary string by prefixing it with '$':

```hexo
> $class_name
```

### Declaring Functions

You can declare arbitrary functions using glyph '#' fallowed by function name and body:

```hexo
# class_declaration {
  > 0100
  > #len($0)
  > $0
}
```

Function arguments are referenced by their index: `$0`, `$1`, `$2`, ...

### Calling Functions

To call a function use glyph '#' fallowed by function name and arguments:

```hexo
> #len('HelloWorld') // will emit length of 'HelloWorld' in bytes (0a)
> #pad_left(AA, 4) // will emit '00 00 00 AA'
> #pad_right(AA, 4) // will emit 'AA 00 00 00'
```

### Example

Let's write _'HelloWorld'_ Java class bytecode:

```hexo
$ class_name 'HelloWorld'
$ object_superclass_name 'java/lang/Object'

# class_declaration {
  > 0100
  > #len($0)
  > $0
}

> cafe babe // Magic number

> 0000 0034 // Java Bytecode version
> 0005 // Constant pool size
> 0700 02 // Class at index 2

> #class_declaration($class_name)
> 0700 04 // Class at index 4
> #class_declaration($object_superclass_name)

> 0021 // Supper public
> 0001 0003 // Class pointers
> 0000 0000 0000 0000 // No interfaces, fields, methods, attributes
```