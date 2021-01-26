use memmap::{Mmap, MmapMut, MmapOptions};
use rayon::prelude::*;
use std::io::{Seek, SeekFrom, Write};

use std::fs::{File, OpenOptions};

const CHUNK_SIZE: usize = 4 * 1024 * 1024;

pub fn reverse_file(input_filepath: &String, output_filepath: &String) {
    let input_data = create_input_mapping(input_filepath);

    let input_size = input_data.len();
    let mut output_data = create_output_mapping(output_filepath, input_size as u64);

    output_data
        .par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(chunk_index, output_chunk)| {
            let (start, end) = output_chunk_range(chunk_index, CHUNK_SIZE, input_size);
            reverse_copy(output_chunk, &input_data[start..end]);
        });
}

fn create_output_mapping(output_filepath: &String, size: u64) -> MmapMut {
    let mut output_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output_filepath)
        .expect("Unable to open file");

    output_file.seek(SeekFrom::Start(size - 1)).unwrap();
    output_file.write_all(&[0]).unwrap();
    output_file.seek(SeekFrom::Start(0)).unwrap();

    unsafe {
        memmap::MmapOptions::new()
            .map_mut(&output_file)
            .expect("Could not access data from memory mapped file")
    }
}

fn create_input_mapping(input_filepath: &String) -> Mmap {
    let file = File::open(input_filepath).expect("Failed to open a file");
    unsafe {
        MmapOptions::new()
            .map(&file)
            .expect("Failed to MMAP a file")
    }
}

fn reverse_copy(dst: &mut [u8], src: &[u8]) -> () {
    let size = src.len();
    unsafe {
        for i in 0..size {
            *dst.get_unchecked_mut(i) = *src.get_unchecked(size - 1 - i);
        }
    }
}

fn output_chunk_range(chunk_index: usize, chunk_size: usize, total_len: usize) -> (usize, usize) {
    let start = if total_len < (chunk_index + 1) * chunk_size {
        0
    } else {
        total_len - (chunk_index + 1) * chunk_size
    };
    let end = total_len - chunk_index * chunk_size;
    (start, end)
}
