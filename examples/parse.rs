use hocr_parser::HOCR;

fn main() -> hocr_parser::Result<()> {
    let xml_str = std::fs::read_to_string("examples/data/paper-image.hocr").unwrap();
    let hocr = HOCR::from_str(&xml_str)?;

    println!("OCR System: {}", &hocr.system);
    println!("Capabilities: {:?}", &hocr.capabilities);
    println!("Number of Pages: {:?}", &hocr.number_of_pages);
    println!("Langs: {:?}", &hocr.langs);
    println!("Scripts: {:?}", &hocr.scripts);
    println!("# of Elements: {}", hocr.iter().count());

    Ok(())
}