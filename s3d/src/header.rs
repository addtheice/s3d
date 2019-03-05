/// The S3D file header which describes the
/// location of the file descriptors within the
/// S3D file. Also contains an S3D magic number
/// and version number.
#[derive(Debug, Clone, Copy)]
pub struct Header {
    /// The offset tells you exactly how far from the
    /// start of the S3D file the file count and
    /// descriptor blocks begin.
    pub offset: u32,
    /// Magic number specific to the S3D file
    /// which indicates that this file is valid.
    /// The number is defined as HEADER_MAGIC_NUMBER
    /// and has a value of 0x20534650.
    /// this is 'PFS ' if converted to text.
    pub magic_number: u32,
    /// The version of the S3D file standard.
    /// The number is defined as HEADER_VERSION_NUMBER
    /// and has a values of 0x20000;
    pub version_number: u32,
}
