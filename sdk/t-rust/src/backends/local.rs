use serde::{de::DeserializeOwned, Serialize};
use std::io::BufRead;
use std::prelude::v1::*;
use std::{fs, vec};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    sync::{Mutex, OnceLock},
};

const INPUTS_FILENAME: &str = "inputs.bin";
const PRIVATE_OUTPUTS_FILENAME: &str = "pr_outputs.bin";
const PUBLIC_OUTPUTS_FILENAME: &str = "pu_outputs.bin";

static INPUTS_READER: OnceLock<Mutex<ArgsReader<BufReader<File>>>> = OnceLock::new();
static INPUTS_WRITER: OnceLock<Mutex<ArgsWriter<BufWriter<File>>>> = OnceLock::new();
static PRIVATE_OUTPUTS_READER: OnceLock<Mutex<ArgsReader<BufReader<File>>>> = OnceLock::new();
static PRIVATE_OUTPUTS_WRITER: OnceLock<Mutex<ArgsWriter<BufWriter<File>>>> = OnceLock::new();

static PUBLIC_OUTPUTS_READER: OnceLock<Mutex<ArgsReader<BufReader<File>>>> = OnceLock::new();
static PUBLIC_OUTPUTS_WRITER: OnceLock<Mutex<ArgsWriter<BufWriter<File>>>> = OnceLock::new();

pub struct ArgsReader<R: BufRead> {
    reader: R,
}

impl<R: BufRead> ArgsReader<R> {
    pub fn new(reader: R) -> Self {
        ArgsReader { reader }
    }

    pub fn read<T: DeserializeOwned>(&mut self) -> Result<T, &str> {
        Ok(bincode::deserialize(self.read_vec().unwrap().as_slice())
            .expect("Failed to deserialize data"))
    }

    pub fn read_vec(&mut self) -> Result<Vec<u8>, &str> {
        let mut buf_length = vec![0; 4];
        self.reader.read_exact(&mut buf_length).unwrap();
        let length = u32::from_le_bytes(buf_length.try_into().unwrap());
        let mut vec = vec![0; length as usize];
        self.reader.read_exact(&mut vec).unwrap();
        Ok(vec)
    }

    pub fn read_vecs(&mut self) -> Result<Vec<Vec<u8>>, ()> {
        let mut vecs = vec![];
        let mut buf_length = [0; 4];
        loop {
            match self.reader.read_exact(&mut buf_length) {
                Ok(_) => {
                    let length = u32::from_le_bytes(buf_length);
                    let mut vec = vec![0; length as usize];
                    self.reader.read_exact(&mut vec).unwrap();
                    vecs.push(vec);
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(vecs)
    }
}

pub struct ArgsWriter<W: Write> {
    writer: W,
}

impl<W: Write> ArgsWriter<W> {
    pub fn new(writer: W) -> Self {
        ArgsWriter { writer }
    }

    pub fn write<T: Serialize>(&mut self, value: &T) -> Result<(), ()> {
        let data = bincode::serialize(value).unwrap();
        self.write_slice(&data.as_slice()).unwrap();
        Ok(())
    }

    pub fn write_slice(&mut self, value: &[u8]) -> Result<(), ()> {
        let length = value.len() as u32;
        self.writer.write_all(&length.to_le_bytes()).unwrap();
        self.writer.write_all(value).unwrap();
        self.writer.flush().unwrap();
        Ok(())
    }
}

fn open_mutexed_reader_file(filename: &str) -> Mutex<ArgsReader<BufReader<File>>> {
    let file = File::open(filename).expect("Failed read arguments: use -k to pass input arguments"); // TODO: handle input error properly
    let reader = ArgsReader::new(BufReader::new(file));
    Mutex::new(reader)
}

fn open_mutexed_writer_file(filename: &str) -> Mutex<ArgsWriter<BufWriter<File>>> {
    let file = File::create(filename).expect("Failed to pass arguments");
    let writer = ArgsWriter::new(BufWriter::new(file));
    Mutex::new(writer)
}

pub fn clean() {
    fs::remove_file(INPUTS_FILENAME).unwrap();
    fs::remove_file(PRIVATE_OUTPUTS_FILENAME).unwrap();
    fs::remove_file(PUBLIC_OUTPUTS_FILENAME).unwrap();
}

fn get_inputs_reader() -> &'static Mutex<ArgsReader<BufReader<File>>> {
    INPUTS_READER.get_or_init(|| open_mutexed_reader_file(INPUTS_FILENAME))
}

fn get_inputs_writer() -> &'static Mutex<ArgsWriter<BufWriter<File>>> {
    INPUTS_WRITER.get_or_init(|| open_mutexed_writer_file(INPUTS_FILENAME))
}

fn get_private_outputs_reader() -> &'static Mutex<ArgsReader<BufReader<File>>> {
    PRIVATE_OUTPUTS_READER.get_or_init(|| open_mutexed_reader_file(PRIVATE_OUTPUTS_FILENAME))
}

fn get_private_outputs_writer() -> &'static Mutex<ArgsWriter<BufWriter<File>>> {
    PRIVATE_OUTPUTS_WRITER.get_or_init(|| open_mutexed_writer_file(PRIVATE_OUTPUTS_FILENAME))
}

fn get_public_outputs_reader() -> &'static Mutex<ArgsReader<BufReader<File>>> {
    PUBLIC_OUTPUTS_READER.get_or_init(|| open_mutexed_reader_file(PUBLIC_OUTPUTS_FILENAME))
}

fn get_public_outputs_writer() -> &'static Mutex<ArgsWriter<BufWriter<File>>> {
    PUBLIC_OUTPUTS_WRITER.get_or_init(|| open_mutexed_writer_file(PUBLIC_OUTPUTS_FILENAME))
}

pub fn write<T: Serialize>(value: &T) {
    get_private_outputs_writer()
        .lock()
        .unwrap()
        .write(value)
        .unwrap();
}

pub fn write_slice(value: &[u8]) {
    get_private_outputs_writer()
        .lock()
        .unwrap()
        .write_slice(value)
        .unwrap()
}

pub fn commit<T: Serialize>(value: &T) {
    get_public_outputs_writer()
        .lock()
        .unwrap()
        .write(value)
        .unwrap();
}

pub fn commit_slice(value: &[u8]) {
    get_public_outputs_writer()
        .lock()
        .unwrap()
        .write_slice(value)
        .unwrap()
}

pub fn read<T: DeserializeOwned>() -> T {
    get_inputs_reader().lock().unwrap().read().unwrap()
}

pub fn read_vec() -> Vec<u8> {
    get_inputs_reader().lock().unwrap().read_vec().unwrap()
}

pub fn read_private_output_vecs() -> Vec<Vec<u8>> {
    get_private_outputs_reader()
        .lock()
        .unwrap()
        .read_vecs()
        .unwrap()
}

pub fn read_public_outputs_vecs() -> Vec<Vec<u8>> {
    get_public_outputs_reader()
        .lock()
        .unwrap()
        .read_vecs()
        .unwrap()
}

pub fn write_input_slice(value: &[u8]) {
    get_inputs_writer()
        .lock()
        .unwrap()
        .write_slice(&value)
        .unwrap()
}

pub fn write_input<T: Serialize>(value: &T) {
    get_inputs_writer().lock().unwrap().write(&value).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{
        commit, commit_slice, read, read_public_outputs_vecs, read_vec, write_input,
        write_input_slice,
    };

    #[test]
    pub fn should_read_input_and_outputs() {
        should_read_input_u32();
        should_read_input_slice();
        should_read_output_u32();
        should_read_output_slice();
    }

    pub fn should_read_input_u32() {
        let n: u32 = 1;
        write_input::<u32>(&n);
        let input = read::<u32>();
        assert_eq!(input, n);
    }

    pub fn should_read_input_slice() {
        let slice: [u8; 4] = [0, 1, 2, 3];
        write_input_slice(&slice);
        let input = read_vec();
        assert_eq!(input, slice);
    }

    pub fn should_read_output_u32() {
        let n: u32 = 1;
        commit::<u32>(&n);
        let outputs = read_public_outputs_vecs();
        assert_eq!(outputs[0], n.to_le_bytes());
    }

    pub fn should_read_output_slice() {
        let slice: [u8; 4] = [0, 1, 2, 3];
        commit_slice(&slice);
        let outputs = read_public_outputs_vecs();
        assert_eq!(outputs[0], slice);
    }
}
