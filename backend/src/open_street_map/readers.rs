use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::member::{self, Member};
use crate::node::Node;
use crate::relation::Relation;
use crate::tag::Tag;
use crate::utils::MapsType;
use crate::way::Way;

/// Reads nodes from an OpenStreetMap (OSM) XML file.
///
/// ## Arguments
/// * `path` - The path to the OSM XML file.
///
/// ## Returns
/// * A result containing a vector of `Node` if successful, or an error if the reading fails.
pub fn read_nodes_from_file(path: &str) -> Result<Vec<Node>, Box<dyn Error>>{
    // Open the XML file
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));

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

/// Reads ways from an OpenStreetMap (OSM) XML file.
///
/// ## Arguments
/// * `path` - The path to the OSM XML file.
///
/// ## Returns
/// * A result containing a vector of `Way` if successful, or an error if the reading fails.
pub fn read_ways_from_file(path: &str) -> Result<Vec<Way>, Box<dyn Error>>{
    // Open the XML file
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));

    let mut ways: Vec<Way> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            // Handle the start of a <way> element with nested tags (non-self-closing)
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"way") => {
                let mut way = Way {
                    id: 0,
                    version: 0,
                    timestamp: String::new(),
                    changeset: 0,
                    uid: 0,
                    user: String::new(),
                    node_refs: Vec::new(),
                    tags: Vec::new(),
                };

                // Parse the attributes of the <way> element
                for attr in e.attributes() {
                    match attr? {
                        a if a.key == quick_xml::name::QName(b"id") => way.id = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"version") => way.version = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"timestamp") => way.timestamp = String::from_utf8(a.value.to_vec())?,
                        a if a.key == quick_xml::name::QName(b"changeset") => way.changeset = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"uid") => way.uid = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"user") => way.user = String::from_utf8(a.value.to_vec())?,
                        _ => (),
                    }
                }
                ways.push(way);
            },

            // Handle <nd> elements nested within <way> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"nd") => {
                if let Some(last_way) = ways.last_mut() {
                    // Parse the attributes of the <tag> element
                    let mut node_ref = -1;
                    for attr in e.attributes() {
                        match attr? {
                            a if a.key == quick_xml::name::QName(b"ref") => node_ref = String::from_utf8(a.value.to_vec())?.parse()?,
                            _ => (),
                        }
                    }
                    if node_ref != -1 {
                        last_way.node_refs.push(node_ref);
                    }
                }
            }

            // Handle <tag> elements nested within <node> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"tag") => {
                if let Some(last_way) = ways.last_mut() {
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
                    last_way.tags.push(tag);
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

    Ok(ways)
}

/// Reads relations and it's members from an OpenStreetMap (OSM) XML file.
///
/// ## Arguments
/// * `path` - The path to the OSM XML file.
///
/// ## Returns
/// * A result containing a vector of `Relation` if successful, or an error if the reading fails.
pub fn read_relations_from_file(path: &str) -> Result<Vec<Relation>, Box<dyn Error>>{
    // Open the XML file
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));

    let mut relations: Vec<Relation> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            // Handle the start of a <relation> element with nested tags (non-self-closing)
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"relation") => {
                let mut relation = Relation {
                    id: 0,
                    version: 0,
                    timestamp: String::new(),
                    changeset: 0,
                    uid: 0,
                    user: String::new(),
                    tags: Vec::new(),
                    members: Vec::new(),
                };

                // Parse the attributes of the <way> element
                for attr in e.attributes() {
                    match attr? {
                        a if a.key == quick_xml::name::QName(b"id") => relation.id = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"version") => relation.version = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"timestamp") => relation.timestamp = String::from_utf8(a.value.to_vec())?,
                        a if a.key == quick_xml::name::QName(b"changeset") => relation.changeset = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"uid") => relation.uid = String::from_utf8(a.value.to_vec())?.parse()?,
                        a if a.key == quick_xml::name::QName(b"user") => relation.user = String::from_utf8(a.value.to_vec())?,
                        _ => (),
                    }
                }
                relations.push(relation);
            },

            // Handle <member> elements nested within <relation> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"member") => {
                if let Some(last_relation) = relations.last_mut() {
                    // Parse the attributes of the <member> element
                    let mut ref_id = 0;
                    let mut maps_type = MapsType::Other("Unknown");
                    let mut role = String::new();

                    for attr in e.attributes() {
                        match attr? {
                            a if a.key == quick_xml::name::QName(b"type") => maps_type = String::from_utf8(a.value.to_vec())?.parse()?,
                            a if a.key == quick_xml::name::QName(b"ref") => ref_id = String::from_utf8(a.value.to_vec())?.parse()?,
                            a if a.key == quick_xml::name::QName(b"role") => role = String::from_utf8(a.value.to_vec())?.to_string(),
                            _ => (),
                        }
                    }

                    if maps_type != MapsType::Other("Unknown") {
                        // Create the member with the correct relation_id
                        let member = Member::new(last_relation.id, ref_id, maps_type, role);
                        last_relation.members.push(member);
                    }
                }
            }

            // Handle <tag> elements nested within <node> elements
            Ok(Event::Empty(ref e)) if e.name() == quick_xml::name::QName(b"tag") => {
                if let Some(last_relation) = relations.last_mut() {
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
                    last_relation.tags.push(tag);
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

    Ok(relations)
}
