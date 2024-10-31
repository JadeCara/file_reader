// import library lib.rs function read_in
use file_reader::read_in;

pub fn main() {
    read_in(None);
    read_in(Some(String::from("nosuchfile.txt")));
}