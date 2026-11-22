# Caching

A cache trades freshness for speed or reduced storage load. Decide on keys, capacity, expiry, invalidation, and behavior when many clients miss the same key.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Read → hit and fresh: return value
Read → missing or expired: coalesce concurrent loads, then cache successful data
Write → commit source of truth, then invalidate related cached data
```

## 2. Common mistake

Validate tenant and permission boundaries before sharing cached values across callers.

## Practice

Use a fake clock to test hits, expiry, bounded capacity, and write invalidation.

## Related lesson

[Input validation](<../../23 - Security Engineering/02 - Input validation/README.md>)

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: gRPC](<../12 - gRPC/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Background jobs →](<../14 - Background jobs/README.md>)
