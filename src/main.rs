use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name};

fn main() {
    // URL of the web page
    let url = "https://www.goldstockdata.com/company/1116-American-Pacific-Mining.html"; // Replace with the actual URL

    // Fetch the HTML content of the web page
    let html = reqwest::blocking::get(url)
        .expect("Failed to fetch URL")
        .text()
        .expect("Failed to read response body");

    // Parse the HTML content
    let document = Document::from_read(html.as_bytes()).expect("Failed to parse HTML");

    // Define a selector to find the element
    let selector = Attr("data-general-details");

    // Find the element matching the selector
    if let Some(element) = document.find(selector).next() {
        // Extract the text content of the element
        let value = element.text();

        println!("Value: {}", value);
    } else {
        println!("Element not found.");
    }
}

