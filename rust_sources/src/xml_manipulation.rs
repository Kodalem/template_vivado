use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn xml_example_main(file_path: String) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            // Show name and attributes of the start element
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                for a in attributes {
                    println!("{:spaces$} {key}={value}", "", spaces = depth * 2, key = a.name, value = a.value);
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(())
}

fn read_if_xml_contains_string(file_path: String, string_to_find: String) -> std::io::Result<bool> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            // Show name and attributes of the start element
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                for a in attributes {
                    println!("{:spaces$} {key}={value}", "", spaces = depth * 2, key = a.name, value = a.value);
                    if a.value == string_to_find {
                        return Ok(true);
                    }
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_example_main() {
        let file_path = "../dummy_project/LabN/LabN.xpr".to_string();
        xml_example_main(file_path).unwrap();
    }
    
    #[test]
    fn test_read_if_xml_contains_string() {
        let file_path = "../dummy_project/LabN/LabN.xpr".to_string();
        let string_to_find = "Basys3_Master".to_string();
        let result = read_if_xml_contains_string(file_path, string_to_find).unwrap();
        assert_eq!(result, true);
    }
}