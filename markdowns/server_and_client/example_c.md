# Server and client demo

### Example 3: Connection timeout error due to non-existent server


```mermaid
---
Example C
---
gitGraph TB:
    commit id: "new client"
    branch client
    checkout client
    commit id: "request url"
    commit id: "after 1s"
    checkout main
    merge client id: "error" type: REVERSE
```