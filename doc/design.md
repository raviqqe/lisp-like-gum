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

### Made of List and Int

#### Function

```
Function = (Symbol -> Ref, [Symbol], Expr)
```


## Algorithm

1. Process incoming messages
2. If #tasks > 0
  - Then run a task
  - Else look for tasks
