#### http_reader.rs

```mermaid
---
title: mod http_reader
---
classDiagram
ChunkReader <|--  HttpReader
class HttpReader {
    request_builder: RequestBuilder
    from_request(request_builder)
    from_url(url)
    read_chunk_stream(chunks: Vec~ChunkOffset~) -> ChunkReader 
    read_at(offset,size)
    read_chunks(chunks: Vec~ChunkOffset~)
}
class ChunkReader {
    request_builder: &'a RequestBuilder
    chunk_buf: BytesMut
    chunks: Vec~ChunkOffset~
    chunk_index: usize
    num_adjacent_reads: usize
    request:HttpRangeRequest
    poll_read(cx:context)
    adjacent_reads(chunks: &[ChunkOffset])->usize
    poll_next(cx:context) -> Poll
    size_hint()->chunks_left:usize
}

```