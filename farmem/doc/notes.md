# Notes

- Process all incoming messages every time when `Memory.is_cached(Ref)` or
  something is called.
- Automatic Drop for Ref and LocalAddress?
- Add TTL field to Demand messages
- Notice sturct to send notices to client code?
- Memory.pseudo\_ref() for upstream Thunk and lazy evaluation implementation?
  - Or, (Weight, TypeId, *Locked*, T)?
  - Or, *Dirty*?
- When memory mutability is necessary, how can it be implemented?
  - Memory.locals and LocalAddress /= raw pointer?

- Fetch and Load messages?
  - No. Because they represents the same concept inherently.
