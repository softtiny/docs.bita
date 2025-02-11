###### clone_cmd.rs


```mermaid
---
title: struct/enum
---
classDiagram
    class RemoteInput {
        url: Url
        retries: u32
        retry_delay: Duration
        receive_timeout: Option&lt;Duration&gt;
        headers: HeaderMap
    }
    class InputArchive {
        Local(std::path::PathBuf)
        Remote(Box&lt;RemoteInput&gt;)
        source()
    }
    class Options {
        force_create: bool
        input_archive: InputArchive
        header_checksum: Option&lt;Hashsum&gt;
        output: PathBuf
        seed_stdin: bool
        seed_files: Vec&lt;PathBuf&gt;
        seed_output: bool
        verify_output: bool
        num_chunk_buffers: usize
        +source_url: OPtion&lt;url&gt;
        +split_head: bool
    }
```

```mermaid
stateDiagram-v2
    state match_url <<choice>>
    state split_head <<choice>>
    state "reader.set_source(opts.source_url.clone())" as set_source
    match_input: opts.input_archive
   
    [*] --> clone_cmd :opts=options
    clone_cmd --> match_input
    match_input --> match_url
    state Local {
        IoReader
    }
    match_url --> Local
    state remote {
        request 
    }
    match_url --> remote
    Local --> clone_archive
    remote --> split_head
    split_head --> set_source :opts.split_head=true
    set_source --> clone_archive_ext
    split_head --> clone_archive_ext:opts.split_head=false
    clone_archive --> clone_archive_ext :split_head = false
```