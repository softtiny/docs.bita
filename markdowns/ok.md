
```mermaid
    sequenceDiagram
    participant client
    participant http
    participant server
    client-->>http: run
    http-->>server: request
    server -->>http: send
    http-->>client: hashmap
```