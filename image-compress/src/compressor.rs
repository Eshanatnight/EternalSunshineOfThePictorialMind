extern crate oxipng;

use oxipng::{optimize, optimize_from_memory};
use oxipng::{InFile, OutFile, IndexSet, PngError};
use std::path::{Path, PathBuf};

pub type PngResult<T> = Result<T, PngError>;

pub struct Compressor {}

impl Compressor {
    pub fn new() -> Self {
        Compressor {  }
    }

    pub fn compress(&self, input: &str) -> PngResult<()> {
        let input = PathBuf::from(input);
        let (output, options) = self.get_opt_with_output(&input);

        let result = optimize(&InFile::Path(input), &output, &options);

        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn compress_from_memory(&self, input: &[u8]) -> PngResult<Vec<u8>> {
        let opts = self.get_options();
        let result = optimize_from_memory(&input, &opts);

        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    fn get_options(&self) -> oxipng::Options {
        let mut options = oxipng::Options {
            force: true,
            ..Default::default()
        };

        let mut filter = IndexSet::new();
        filter.insert(0);
        options.filter = filter;

        return options;
    }

    fn get_opt_with_output(&self, path: &Path) -> (OutFile, oxipng::Options) {
        let output: OutFile = OutFile::Path(Some(path.with_extension("out.png")));
        let options: oxipng::Options = self.get_options();

        (output, options)
    }
}


#[test]
fn test_compress() {
    let engine = Compressor::new();
    let res = engine.compress("compress.out.png");

    match res {
        Ok(_) => {println!("Success")},
        Err(e) => {panic!("Error {e}")},
    }
}