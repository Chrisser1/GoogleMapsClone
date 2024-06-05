use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::node::Node;
use crate::tag::Tag;

/// Reads nodes from an OpenStreetMap (OSM) XML file.
///
/// # Arguments
/// * `path` - The path to the OSM XML file.
///
/// # Returns
/// A result containing a vector of `Node` if successful, or an error if the reading fails.
pub fn read_nodes_from_file(path: &str) -> Result<Vec<Node>, Box<dyn Error>>{
    // Open the XML file
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);

    let mut nodes: Vec<Node> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            // Handle the start of a <node> element with nested tags (non-self-closing)
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"node") => {
                let mut node = Node {
                    id: 0,
                    lat: 0.0,
                    lon: 0.0,
                    version: 0,
                    timestamp: String::new(),
                    changeset: 0,
                    uid: 0,
                    user: String::new(),
                    tags: Vec::new(),
                };

                // Parse the attributes of the <node> element
                for attr in e.attributes() {
                    match attr? {
                        a if a.key == quick_xml::name::QName(b"id") => node.id = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"lat") => node.lat = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"lon") => node.lon = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"version") => node.version = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"timestamp") => node.timestamp = String::from_utf8(a.value.to_vec())?,
                        a if a.key == quick_xml::name::QName(b"changeset") => node.changeset = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"uid") => node.uid = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"user") => node.user = String::from_utf8(a.value.to_vec())?,
                        _ => (),
                    }
                }
                nodes.push(node);
            },
            // Handle self-closing <node> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"node") => {
                let mut node = Node {
                    id: 0,
                    lat: 0.0,
                    lon: 0.0,
                    version: 0,
                    timestamp: String::new(),
                    changeset: 0,
                    uid: 0,
                    user: String::new(),
                    tags: Vec::new(),
                };

                // Parse the attributes of the self-closing <node> element
                for attr in e.attributes() {
                    match attr? {
                        a if a.key == quick_xml::name::QName(b"id") => node.id = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"lat") => node.lat = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"lon") => node.lon = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"version") => node.version = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"timestamp") => node.timestamp = String::from_utf8(a.value.to_vec())?,
                        a if a.key == quick_xml::name::QName(b"changeset") => node.changeset = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"uid") => node.uid = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"user") => node.user = String::from_utf8(a.value.to_vec())?,
                        _ => (),
                    }
                }
                nodes.push(node);
            }
            // Handle <tag> elements nested within <node> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"tag") => {
                if let Some(last_node) = nodes.last_mut() {
                    let mut tag = Tag {
                        key: String::new(),
                        value: String::new(),
                    };

                    // Parse the attributes of the <tag> element
                    for attr in e.attributes() {
                        match attr? {
                            a if a.key == quick_xml::name::QName(b"k") => tag.key = String::from_utf8(a.value.to_vec())?,
                            a if a.key == quick_xml::name::QName(b"v") => tag.value = String::from_utf8(a.value.to_vec())?,
                            _ => (),
                        }
                    }
                    last_node.tags.push(tag);
                }
            }
            // End of the XML document
            Ok(Event::Eof) => break,
            // Handle errors
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
        // Clear buffer for the next read
        buf.clear();
    }

    Ok(nodes)
}
