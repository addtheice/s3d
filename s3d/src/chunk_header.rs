/// The data for each file contained within the s3d file
/// is stored in a series of one or more data 'chunks'
/// these chunks are the actual compressed data of the file.
/// At the beginning of each chunk is a chunk header which
/// describes how large the chunk is decompressed and
/// compressed.
#[derive(Clone, Debug, Copy)]
pub struct ChunkHeader {
    pub compressed_size: u32,
    pub decompressed_size: u32,
}
