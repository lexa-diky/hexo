# Hexo

Tiny binary writer utility, just enough for you

## Installation

// TODO

## CLI

### Compile

Takes `source` file in hexo format and compiles it to binary file `output`

```bash
hexo watch --source <path to source> --output <path to output>
```

## Syntax

### Emitter

To emit a byte use glyph '>' fallowed by byte value:

```hexo
> 0a // by default numbers are interpreted as hexadecimal, will emit decimal 10
> 'HelloWorld' // will emit utf-8 bytes of 'HelloWorld' string
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

### Calling Functions

To call a function use glyph '#' fallowed by function name and arguments:

```hexo
> #len('HelloWorld') // will emit length of 'HelloWorld' in bytes (0a)
```

### Example

Let's write _'HelloWorld'_ Java class bytecode:

```hexo
$ class_name 'HelloWorld'
$ object_superclass_name 'java/lang/Object'

> cafe babe // Magic number

> 0000 0034 // Java Bytecode version
> 0005 // Constant pool size
> 0700 02 // Class at index 2

> 0100 #len($class_name) $class_name
> 0700 04 // Class at index 4
> 0100 #len($object_superclass_name) $object_superclass_name

> 0021 // Supper public
> 0001 0003 // Class pointers
> 0000 0000 0000 0000 // No interfaces, fields, methods, attributes
```