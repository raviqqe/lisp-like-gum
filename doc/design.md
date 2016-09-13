# Design

## Data structures

```
Packet = Fetch GA GA | Resume GA GA Thunk
       | Fish ProcessorId | Schedule Graph | Ack GA
       | DelRef Weight
       | Finish

Graph = [(GA, Thunk)]

Thunk = Object | App
App = (Thunk, Thunk) # The latter must be a list.

Object = List | Int

Function = (Env, [Symbol], Expr)
Env = Symbol -> ThunkRef
Expr = Thunk

Symbol = String

Processor = (ProcessorId, LID2LATable, GA2LATale, LA2GATale)

LID2LATable = LocalId -> LocalAddress
GA2LATable = GA -> LA
LA2GATable = LA -> GA
GlobalAddress = (ProcessorId, LocalId)

LocalAddress in Pointer
ProcessorId, LocalId, Weight in NaturalNumber
```


## Procedure

1. GC
2. Process incoming messages
3. If #threads > 0
  - Then run a thread
  - Else look for sparks
