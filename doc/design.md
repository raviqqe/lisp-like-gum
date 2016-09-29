# Design

## Data structures

### Message

```
Message =
  // Distributed memory

    Fetch (GA, LA)
  | Resume (LA, GA, Object)

  | AddWeight (LA, Weight)
  | SubWeight (LA, Weight)

  // Scheduling

  | Fish ProcessorId
  | Schedule (Thunk, [(GA, Thunk)])
  | Renew (LA, GA)

  // Others

  | Ack LA
  | Finish
```

### Object

```
Object = /* byte array */

refs : Object -> [Ref]
```

### Ref

```
Ref = (ProcessorId, LA)
```

### Thunk

```
Thunk = Object | App (Ref, Ref)
```

### LocalAddress (LA)

```
LocalAddress = *Thunk
```

### GlobalAddress (GA)

```
GlobalAddress = (ProcessorId, LocalAddress)
```

### Processor

```
Processor = (ProcessorId, GlobalCache)

GlobalCache = GA -> Object
```


## Algorithm

1. Process incoming messages
2. If #tasks > 0
  - Then run a task
  - Else look for tasks


## Language

### Data structures

~~
Every type constructors are represented as concrete types (i.e. Types) in the
language.
~~

```
Ref A = /* Ref in the host language to the type A in the language */

Type = (Ref Symbol, Ref (Symbol -> Function))

Function = RawFunction (Symbol -> Ref, [Symbol], Expr)
         | BuiltinFunction Id

List = (Ref _, Ref List)

Array = (Ref _, Ref _, ..., Ref _)

Symbol = ByteArray

ByteArray = /* literally low-level byte array */
```


## Notes

- Coroutine is not necessary
  - Self implementation of Async IO for NIH syndrome
    - One IO thread
    - Using MIO?
    - Mark the Thunk "blocked"
  - Thunks does not conduct raw calls. i.e. no need for stacks
  - Save Thunks' execution states as App structures
    - Call another builtin function
  - For portability of Thunks' execution states
- No VM or bytecode
  - No real call in the bytecode level
    - Each call is represented as an App structure
- Dynamic typing
  - The original developer was not sophisticated enough to implement static
    typing and type inference while he bought TAPL.
  - The language will probably gradually-typed hopefully.

### Python

- VM
  - Stack machine
  - Well-defined arithmetic operators for big integers and comparison ones
  - Attribute instructions
  - BUILD\_\* instructions to create builtin types from values in the stack
  - CALL\_FUNCTION\* instructions
    - Two kinds of function types exist
      - PyFunction_Type
      - PyCFunction_Type
  - IO
    - PRINT\_\* instructions
      - PRINT\_ITEM\_TO and PRINT\_NEWLINE\_TO?
    - Call C functions
  - cf. [Python bytecode interpreter in Python](the://github.com/nedbat/byterun/blob/master/byterun/pyvm2.py)
