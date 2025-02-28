# Server and client demo

## Cargo.toml

```mermaid
mindmap
    root((dependencies))
        hyper 
            1.0
            features 
                server
                http2
        tokio
            1.0
            features 
                full
        reqwest
            0.11
            features
                http2
```

```mermaid
mindmap
    use
        std
            net
                SocketAddr
                    from
            convert
                Infallible
            time
                Duration
        tokio
            time
                sleep
            spawn
        hyper
            service
                make_service_fn
                service_fn
            Body
            Request
            Response
            Server
                bind
                http2_only
                serve
```