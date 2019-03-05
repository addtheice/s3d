/// Inside the s3d file format the s3d header contains
/// multiple s3d descriptor blocks. These descriptors
/// define where inside the s3d file the contained files are
/// stored. These offsets do not have to be in any particular
/// order which can cause confusion. In practice they tend to
/// be stored in the file in order of their CRC-32's.
///
/// For example, the first descriptor could have an offset of
/// 487, which would be the 487th byte of the over all file,
/// while the second descriptor block might have an offset of
/// 37! This would mean the first file in the header comes
/// after the second file. This is obviously annoying and
/// could cause some slow down when trying to load an entire
/// file.
///
/// One solution to this issue is to sort the descriptors by
/// offset after fully loading all the descriptors.
#[derive(Clone, Debug)]
pub struct DescriptorBlock {
    /// CRC-32 Checksum.
    pub crc: u32,
    /// The offset is the number of bytes from the start of
    /// the S3D file where the data chunks for this
    /// file are stored.
    pub offset: u32,
    /// The size of the contained data when decompressed.
    /// It's important to note this is the size of the data
    /// decrompressed, not it's compressed form on disk.
    pub decompressed_size: u32,
    /// The name of the file. This data is derived by
    /// reading the directoryblock (last DescriptorBlock
    /// sorted by offset), each file name is stored inside
    /// this final DescriptorBlock.
    /// It's stored in this structure since it seems the
    /// logical location to store it.
    pub file_name: String,
    /// The compressed data found within each chunk,
    /// they are appended together after removing the
    /// ChunkHeader to make it simpler to process.
    pub data: Vec<Vec<u8>>,
}
