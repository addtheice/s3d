extern crate s3d;

use s3d::S3d;
use std::fs;

fn main() {
    let paths = fs::read_dir("./examples").unwrap();

    for path in paths {
        let pathbuf = path.unwrap().path();
        let extension = pathbuf.extension();

        if extension.is_none()
            || (extension.unwrap().to_str().unwrap() != "s3d"
                && extension.unwrap().to_str().unwrap() != "eqg")
        {
            continue;
        }

        let filepath = pathbuf.to_str().unwrap();

        let f = S3d::open(filepath).unwrap();
        
        println!("processed '{}'", filepath);
        println!("number of files: {}", f.file_count())
    }
}
