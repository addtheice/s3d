//! s3d reads, parses, extracts, and writes Everquest S3D files.
//!
//! An s3d file is internally a type of compressed file containing a collection of textures
//! usually DDS (direct draw surface) or BMP (bitmaps) though technically no restriction
//! exists as to the types of files which can be stored. The compression used on the internal 
//! data is a standard zlib DEFLATE compression algorithm.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_imports
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

mod chunk_header;
mod descriptor_block;
mod file_data;
mod header;

use crate::chunk_header::ChunkHeader;
use crate::descriptor_block::DescriptorBlock;
use crate::header::Header;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, Error, ErrorKind, Result};
use std::path::Path;
use std::str::from_utf8;

use inflate::inflate_bytes_zlib;

use byteorder::{LittleEndian, ReadBytesExt};

static HEADER_MAGIC_NUMBER: u32 = 0x20534650;
static HEADER_VERSION_NUMBER: u32 = 0x20000;

/// Primary structure used to read, parse, extract,
/// and write Everquest S3D files.
#[derive(Debug)]
pub struct S3d {
    /// The S3D file header describes where
    /// the file descriptors are found within the file
    /// as well as the s3d version information and
    /// a confirmation magic number.
    header: Header,
    /// The collection of descriptors which locate the
    /// contained files.
    file_descriptors: Vec<DescriptorBlock>,
}

impl S3d {
    /// Creates an empty S3D file with no
    /// data, no files, and no descriptors.
    pub fn new() -> S3d {
        S3d {
            header: Header {
                offset: 0,
                magic_number: HEADER_MAGIC_NUMBER,
                version_number: HEADER_VERSION_NUMBER,
            },
            file_descriptors: vec![],
        }
    }

    /// Gets the number of files stored within
    /// this S3D file.
    pub fn file_count(&self) -> usize {
        let descriptor_count = self.file_descriptors.len();

        // The file descriptors variable will contain not just
        // each files description but also the directory file
        // descriptor which contains the list of all the files.
        // So if we have an empty file, we should have an empty 
        // file_descriptors, but if we have 1 file, then we 
        // should have two descriptors in the file_descriptors
        if descriptor_count == 0 {
            return 0;
        }
        else {
            return descriptor_count - 1;
        }
    }

    /// Utility function for reading an s3d file from disk.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<S3d> {
        let mut s3d_file = File::open(path)?;
        S3d::read(&mut s3d_file)
    }

    /// Reads from a generic [Reader](https://doc.rust-lang.org/nightly/std/io/trait.Read.html)
    /// to build an S3D file.
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<S3d> {
        let mut result = S3d::new();

        let mut data = vec![];
        reader.read_to_end(&mut data)?;

        let mut cursor = Cursor::new(&data[0..]);
        result.header = S3d::read_header(&mut cursor)?;
        result.file_descriptors = S3d::read_all_descriptors(&mut cursor, result.header.offset)?;

        if result.file_descriptors.len() == 0 {
            let err = Error::new(ErrorKind::InvalidData, "S3D file is valid but empty.");
            return Err(err);
        }

        // Sort the descriptor blocks by offset after
        // fully loading all the descriptors to make
        // it easier to logically iterate over the
        // entire S3D file.
        result.file_descriptors.sort_unstable_by_key(|k| k.offset);

        for mut descriptor in &mut result.file_descriptors {
            let data =
                S3d::read_data(&mut cursor, descriptor.offset, descriptor.decompressed_size)?;
            descriptor.data = data;
        }

        let file_names: Vec<String>;

        let directory_entry = result.file_descriptors.last().unwrap();
        file_names = S3d::read_file_names(&directory_entry.data)?;

        // Skip the last entry since that entry is
        // the directory structure which houses the file
        // names we just extracted.
        for index in 0..result.file_descriptors.len() - 1 {
            result.file_descriptors[index].file_name = file_names[index].clone();
        }

        Ok(result)
    }

    /// Reads the final DescriptorBlock (as ordered by offsets) which contains
    /// the filename information for the S3D file. This could be considered the
    /// 'directory' DescriptorBlock.
    fn read_file_names(data: &Vec<Vec<u8>>) -> Result<Vec<String>> {
        let mut decoded_data = vec![];
        for data_chunk in data {
            let decoded_chunk = inflate_bytes_zlib(&data_chunk).unwrap();
            decoded_data.extend(decoded_chunk);
        }
        let mut cursor = Cursor::new(&decoded_data);
        let file_count = cursor.read_u32::<LittleEndian>().unwrap();
        let mut file_names = vec![];

        for _file_index in 0..file_count {
            let len = cursor.read_u32::<LittleEndian>().unwrap();
            let mut buf = vec![0; len as usize];
            cursor.read_exact(&mut buf)?;

            // from_utf8 assumes the null byte at the end
            // so if we use the last character it pushes on
            // an additional null character, so we just
            // convert without the last character.
            // hence len - 1.
            let name = from_utf8(&buf[0..(len - 1) as usize])
                .unwrap()
                .trim()
                .to_string();
            file_names.push(name);
        }

        Ok(file_names)
    }

    /// Reads the S3D Header.
    fn read_header(cursor: &mut Cursor<&[u8]>) -> Result<Header> {
        cursor.set_position(0);
        let offset = cursor.read_u32::<LittleEndian>()?;
        let magic_number = cursor.read_u32::<LittleEndian>()?;
        let version_number = cursor.read_u32::<LittleEndian>()?;

        if magic_number != HEADER_MAGIC_NUMBER {
            let err = Error::new(ErrorKind::InvalidData, "Data is not in S3D format.");
            return Err(err);
        }

        Ok(Header {
            offset: offset,
            magic_number: magic_number,
            version_number: version_number,
        })
    }

    /// Reads all descriptors as found from the
    /// offset within the underlying Cursor.
    fn read_all_descriptors(
        cursor: &mut Cursor<&[u8]>,
        offset: u32,
    ) -> Result<Vec<DescriptorBlock>> {
        cursor.set_position(offset as u64);
        let file_count = cursor.read_u32::<LittleEndian>()?;
        let mut descriptor_blocks = vec![];

        for _block_index in 0..file_count {
            let descriptor_block = S3d::read_descriptor(cursor)?;
            descriptor_blocks.push(descriptor_block);
        }

        Ok(descriptor_blocks)
    }

    /// Reads a single DescriptorBlock from the underlying Cursor.
    /// Assumes that the Cursor has been set to the start of a
    /// DescriptorBlock.
    fn read_descriptor(cursor: &mut Cursor<&[u8]>) -> Result<DescriptorBlock> {
        let crc = cursor.read_u32::<LittleEndian>()?;
        let offset = cursor.read_u32::<LittleEndian>()?;
        let decompressed_size = cursor.read_u32::<LittleEndian>()?;

        Ok(DescriptorBlock {
            crc: crc,
            offset: offset,
            decompressed_size: decompressed_size,
            file_name: String::new(),
            data: vec![],
        })
    }

    /// Reads a single ChunkHeader from the underlying Cursor.
    /// Assumes that the Cursor has been set to the start of a
    /// ChunkHeader.
    fn read_chunk_header(cursor: &mut Cursor<&[u8]>) -> Result<ChunkHeader> {
        let compressed_size = cursor.read_u32::<LittleEndian>()?;
        let decompressed_size = cursor.read_u32::<LittleEndian>()?;

        let chunk_header = ChunkHeader {
            compressed_size: compressed_size,
            decompressed_size: decompressed_size,
        };

        return Ok(chunk_header);
    }

    /// Reads all the data for a DescriptorBlock.
    /// Assumes the offset is the initial ChunkHeader
    /// for a DescriptorBlock.
    fn read_data(
        cursor: &mut Cursor<&[u8]>,
        offset: u32,
        decompressed_size: u32,
    ) -> Result<Vec<Vec<u8>>> {
        cursor.set_position(offset as u64);
        let mut result = vec![];

        let mut bytes_read = 0;
        while bytes_read != decompressed_size {
            let chunk_header = S3d::read_chunk_header(cursor)?;
            let mut chunk_data = vec![];
            cursor
                .take(chunk_header.compressed_size as u64)
                .read_to_end(&mut chunk_data)?;

            result.push(chunk_data);

            bytes_read += chunk_header.decompressed_size;
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    //! Tests for the S3D format
    //!

    use crate::S3d;

    #[test]
    fn new_s3d_is_empty() {
        let empty = S3d::new();
        assert_eq!(empty.header.offset, 0);
        assert_eq!(empty.file_count(), 0);
        assert_eq!(empty.file_descriptors.len(), 0);
    }

}
