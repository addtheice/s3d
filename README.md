# s3d
S3d reads, parses, extracts, and writes Everquest S3D files.

[![Travis Build Status](https://travis-ci.org/scriptandcompile/s3d.svg?]branch=master)](https://travis-ci.org/addtheice/s3d) [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/cws0gxh623w7tt76?svg=true)](https://ci.appveyor.com/project/scriptandcompile/s3d) [![Codecov Status](https://codecov.io/gh/scriptandcompile/s3d/branch/master/graph/badge.svg)](https://codecov.io/gh/scriptandcompile/s3d) [![MIT License](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/scriptandcompile/s3d/blob/master/LICENSE-MIT) [![Rust Documentation](https://img.shields.io/badge/rust-documentation-blue.svg)](https://scriptandcompile.github.io/s3d)


##### Project Hygenics
- [x] Enforced Lints.
- [x] Code Documentation.
- [ ] Unit Tests.
- [ ] Integration Tests.
- [ ] Performance/Benchmarks.
- [x] Continuous Integration Linux.
- [x] Continuous Integration Windows.
- [x] Codecov Code Coverage.
- [ ] Coveralls.io Code Coverage.
- [ ] Examples.
- [x] Documentation on Github.io.
- [ ] Crates.io.

# Features

  ##### Generic
  - [x] Empty s3d files.
  - [x] Determine the contents of s3d file.
  
  ##### Reading
  - [x] Read/Parse from generic `Reader`s.
  - [ ] Validate checksum on loading.
  - [x] Specializated utility function to read s3d format from disk.
  - [ ] Provide generic `Reader` for content file unpacking.

  ##### Writing
  - [ ] Provide generic `Writer` for content file packing.
  - [ ] Write s3d file to generic `Writer`.
  - [ ] Specializated utility function to write s3d format to disk.
  - [ ] Specializated utility function to pack files into s3d files.


