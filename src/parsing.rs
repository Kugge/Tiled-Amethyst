use std::path::Path;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::mapping::Untiled_Layer;

/*
 * Get & Parse XML/CSV from TMX / TSX Files
 */

fn read_tag(f: &str, tag: &[u8]) -> Vec<String> {
    // Read inside a xml tag
    let path = Path::new(f);
    let mut reader = match Reader::from_file(path) {
        Ok(v) => v,
        Err(e) => panic!("Cannot read xml file {:?} at {:?}: {:?}", f, path, e)
    };

    reader.trim_text(true);
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == tag => {
                txt.push(
                    reader.read_text(tag, &mut Vec::new())
                          .expect("Cannot decode xml text value"),
                );
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("XML Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    txt
}


fn read_attrib(f: &str, tag: &[u8]) -> Vec<String> {
    // Read tag attributes
    let path = Path::new(f);
    let mut reader = match Reader::from_file(path) {
        Ok(v) => v,
        Err(e) => panic!("Cannot read file {:?}: {:?}", f, e)
    };

    reader.trim_text(true);
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == tag => {
                txt.push(e.unescape_and_decode(&reader).expect("Error!"));
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    txt
}


fn parse_csv(s: &str) -> Vec<Vec<i32>> {  // Parse csv into matrix
    let clean_str: String = s.replace(",\r\n", "\r\n");
    let mut reader = ReaderBuilder::new()
    .has_headers(false)
    .from_reader(clean_str.as_bytes());

    let mut matrix: Vec<Vec<i32>> = Vec::new(); // Contains the matrix

    for result in reader.deserialize::<Vec<i32>>() {
        match result {
            Ok(v) => matrix.push(v),
            Err(e) => panic!("Cannot read csv result: {:?}\nString: ({:?})", e, s)
        }
    }
    matrix
}


fn get_layers(f: &str) -> Vec<Untiled_Layer> { // f = Path to map
    // Get the attrib of a layer
    let list: Vec<String> = read_attrib(f, b"layer");  // Get all layers
    list.iter().map(|x| {
        let tmp: Vec<&str> = x.split("\"").collect();
        // Unpacking data
        Untiled_Layer {
            id: tmp[1].parse::<usize>().unwrap(),  // We'll use id to retrieve the right matrix
            name: tmp[3].to_string(),
            width: tmp[5].parse::<u32>().unwrap(),
            height: tmp[7].parse::<u32>().unwrap(),
            matrix: parse_csv(&read_tag(f, b"data")[tmp[1].parse::<usize>().unwrap() - 1])
        }
    }).collect() // Parsing layer list to Layer struct
}

fn get_layers(f: &str) -> Vec<Untiled_Layer> { // f = Path to map
    // Get the attrib of a layer
    let list: Vec<String> = read_attrib(f, b"layer");  // Get all layers
    list.iter().map(|x| {
        let tmp: Vec<&str> = x.split("\"").collect();
        // Unpacking data
        Untiled_Layer {
            id: tmp[1].parse::<usize>().unwrap(),  // We'll use id to retrieve the right matrix
            name: tmp[3].to_string(),
            width: tmp[5].parse::<u32>().unwrap(),
            height: tmp[7].parse::<u32>().unwrap(),
            matrix: parse_csv(&read_tag(f, b"data")[tmp[1].parse::<usize>().unwrap() - 1])
        }
    }).collect() // Parsing layer list to Layer struct
}
