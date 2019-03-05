//use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
//use std::path::Path;
//use std::str::from_utf8;

use deflate::deflate_bytes_zlib;
use inflate::inflate_bytes_zlib;

//use byteorder::{LittleEndian, ReadBytesExt};

const MAX_BUFFER_SIZE: usize = 8192;

///
#[derive(Clone, Debug)]
pub struct FileData {
    file_name: String,
    compressed_data: Option<Vec<u8>>,
    decompressed_data: Option<Vec<Vec<u8>>>,
    read_chunk_index: usize,
    read_byte_index: usize,
    write_chunk_index: usize,
    write_byte_index: usize,
}

impl FileData {
    pub fn new() -> FileData {
        FileData {
            file_name: String::new(),
            compressed_data: None,
            decompressed_data: None,
            read_chunk_index: 0,
            read_byte_index: 0,
            write_chunk_index: 0,
            write_byte_index: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.compressed_data.is_none() && self.decompressed_data.is_none()
    }

    pub fn compressed_len(&mut self) -> usize {
        let len = match self.compressed_data {
            Some(_) => 0,
            None => {
                let just_compressed_data = match self.decompressed_data {
                    Some(ref data) => FileData::compress(&data),
                    None => {
                        return 0;
                    }
                };

                let len = just_compressed_data.len();
                self.compressed_data = Some(just_compressed_data);

                len
            }
        };

        len
    }

    pub fn decompressed_len(&mut self) -> usize {
        match self.decompressed_data {
            Some(ref decompressed_data) => {
                let mut len: usize = 0;
                let chunk_lengths = decompressed_data
                    .iter()
                    .map(|ref chunk| chunk.len())
                    .collect::<Vec<usize>>();
                
                for chunk_size in chunk_lengths.iter() {
                    len += chunk_size;
                }

                return len;
            }
            None => {
                match self.compressed_data {
                    Some(ref compressed_data) => {
                        let decompressed_data = FileData::decompress(compressed_data);
                        let len = decompressed_data.len();
                        self.decompressed_data = Some(decompressed_data);

                        return len;
                    }
                    None => return 0,
                };
            }
        };
    }

    fn compress(chunks: &Vec<Vec<u8>>) -> Vec<u8> {
        let mut compressed_data = vec![];

        for chunk in chunks {
            compressed_data.extend(deflate_bytes_zlib(&chunk));
        }

        return compressed_data;
    }

    fn decompress(compressed_data: &Vec<u8>) -> Vec<Vec<u8>> {
        let mut decompressed_data = vec![];
        for chunk in compressed_data.chunks(MAX_BUFFER_SIZE) {
            let decompress_result = inflate_bytes_zlib(chunk);

            let decompressed_chunk = decompress_result.unwrap();
            decompressed_data.push(decompressed_chunk);
        }

        decompressed_data
    }
}

impl Write for FileData {
    fn write(&mut self, _data: &[u8]) -> Result<usize> {
        self.read_byte_index = 0;
        self.read_chunk_index = 0;

        match self.decompressed_data.as_ref() {
            Some(_decompressed_data) => {}
            None => {}
        }

        Ok(0)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Read for FileData {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.write_byte_index = 0;
        self.write_chunk_index = 0;

        if self.decompressed_data.is_none() && self.compressed_data.is_none() {
            return Ok(0);
        }

        if self.decompressed_data.is_none() && self.compressed_data.is_some() {
            self.decompressed_data =
                Some(FileData::decompress(self.compressed_data.as_ref().unwrap()));
        }

        let chunks = match self.decompressed_data {
            Some(ref chunks) => chunks,
            None => {
                let err = Error::new(
                    ErrorKind::InvalidData,
                    "An empty file_data can not be read.",
                );
                return Err(err);
            }
        };

        let ref current_chunk = chunks[self.read_chunk_index];
        let chunk_length = current_chunk.len();

        if self.read_byte_index == chunk_length {
            self.read_byte_index = 0;
            self.read_chunk_index += 1;
        }

        let ref current_chunk = chunks[self.read_chunk_index];

        let chunk_length = current_chunk.len();

        let mut current_length = 0;
        for index in self.read_byte_index..chunk_length {
            if current_length == buf.len() {
                break;
            }

            self.read_byte_index = index;
            buf[current_length] =
                self.decompressed_data.as_ref().unwrap()[self.read_chunk_index][index];
            current_length += 1;
        }

        Ok(current_length)
    }
}

#[cfg(test)]
mod tests {
    //! Tests for the S3D format
    //!

    use crate::file_data::FileData;

    #[test]
    fn new_file_data_is_empty() {
        let empty = FileData::new();
        assert_eq!(empty.is_empty(), true);
    }

}
