# Supply chain security

Supply chain security covers build inputs and release artifacts. Review dependency source and code executed during builds. Pin CI inputs where appropriate, protect release credentials, and preserve evidence of what produced an artifact.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
review change → resolve locked dependencies → run isolated CI → attest artifact → deploy approved digest
```

## 2. Common mistake

Build scripts and procedural macros execute during compilation. Include them in dependency review.

## Practice

Define which artifact metadata and provenance a deployment job should verify.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Dependency vulnerabilities](<../06 - Dependency vulnerabilities/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Unsafe code auditing →](<../08 - Unsafe code auditing/README.md>)
