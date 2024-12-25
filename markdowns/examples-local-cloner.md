#### from local input compressed file to output. and merge example seed file.

```mermaid
    mindmap
        respawn
            CloneOutput
                output_file
                source_index
                feed(chunk)
                
            seed
                seed_file
            archive
                input_file
                StreamExt
                TryStreamExt
                compressed 
                verified 

```


###### Input File

```mermaid
    flowchart LR
        input_path(input_path)
        open(File::open)
        reader(IoReader::new)
        archive["`
        try_init
        chunker_config
        new_chunker(seed|output)
        chunk_stream(chunks)
        build_source_index()
        `"]
        input_path --> open
        open --> reader
        reader --> archive
        TryStreamExt --> map_ok
        map_ok --> StreamExt --> stream.next
        compressed -->  decompress
        verified --> chunks
```

###### output File

```mermaid
    flowchart LR
    open["`
        OpenOptions:new()
        create(true)
        read(true)
        write(true)
        open(path)
        await
        expect()
    `"]
    clone["`
        feed(chunk)
        chunks()
        reorder_in_place(output_index)
    `"]
    output_name --> open
    open --> clone
```