# Notes

- Process all incoming messages every time when `Memory.is_cached(Ref)` or
  something is called.
- Automatic Drop for Ref and LocalAddress?
- Add TTL field to Demand messages
- Notice sturct to send notices to client code?
- Memory.pseudo\_ref() for upstream Thunk and lazy evaluation implementation?
  - Or, (Weight, TypeId, *Locked*, T)?

- Fetch and Load messages?
  - No. Because they represents the same concept inherently.
