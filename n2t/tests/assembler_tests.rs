use n2t::software::assembler::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    iter::zip,
    path::Path,
};

#[test]
pub fn test_max() {
    let path = Path::new(r#"../ch 6 asm files/Max.asm"#);
    let output = asm_to_hack(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Max_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    for (line_test, line_target) in zip(buf1.lines(), buf2.lines()) {
        assert_eq!(line_test, line_target);
    }
}

#[test]
pub fn test_pong() {
    let path = Path::new(r#"../ch 6 asm files/Pong.asm"#);
    let output = asm_to_hack(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Pong_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    for (line_test, line_target) in zip(buf1.lines(), buf2.lines()) {
        assert_eq!(line_test, line_target);
    }
}

#[test]
pub fn test_rect() {
    let path = Path::new(r#"../ch 6 asm files/Rect.asm"#);
    let output = asm_to_hack(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Rect_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    for (line_test, line_target) in zip(buf1.lines(), buf2.lines()) {
        assert_eq!(line_test, line_target);
    }
}

#[test]
pub fn test_add() {
    let path = Path::new(r#"../ch 6 asm files/Add.asm"#);
    let output = asm_to_hack(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Add_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    for (line_test, line_target) in zip(buf1.lines(), buf2.lines()) {
        assert_eq!(line_test, line_target);
    }
}
