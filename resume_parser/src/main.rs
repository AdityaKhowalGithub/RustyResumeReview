// Import necessary modules from the lopdf crate
use lopdf::{Document, Object};

fn main() {
    // Path to the PDF file
    let file_path = "path/to/your/file.pdf";

    // Attempt to open the PDF document
    let doc = Document::load(file_path).expect("Unable to load PDF file");

    // Iterate over the pages of the PDF
    for (page_number, page_id) in doc.page_iter() {
        // Print the current page number
        println!("Page: {}", page_number);

        // Retrieve the content dictionary for the page, note the change here
        let content = doc.get_dictionary((page_id, 0)).unwrap();

        // Look for the 'Contents' entry in the dictionary
        match content.get(b"Contents") {
            Some(&Object::Reference(reference)) => {
                // If found, get the referenced object
                match doc.get_object(reference) {
                    Ok(Object::Stream(stream)) => {
                        // If it's a stream, extract the stream's data
                        match stream.decompressed_content() {
                            Ok(content) => {
                                // Print the decompressed content
                                println!("{}", String::from_utf8_lossy(&content));
                            },
                            Err(e) => println!("Failed to decompress content: {}", e),
                        }
                    },
                    _ => println!("Content is not a stream"),
                }
            },
            _ => println!("No content found on page"),
        }
    }
}

