use std::fmt;
use std::str::FromStr;
use std::num::{ParseIntError, ParseFloatError};
use std::str::Utf8Error;
use std::error::Error as StdError;
use std::io::{self, Write};

use crate::osm_entities::Tag;

/// Custom error type that can encapsulate different kinds of errors that might occur.
#[derive(Debug)]
pub enum ParseError {
    Utf8Error(Utf8Error),
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    NoDataError,
    InvalidMapsTypeError,
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> Self {
        ParseError::Utf8Error(err)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::IntError(err)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::FloatError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Utf8Error(e) => write!(f, "UTF-8 decoding error: {}", e),
            ParseError::IntError(e) => write!(f, "Integer parsing error: {}", e),
            ParseError::FloatError(e) => write!(f, "Floating point parsing error: {}", e),
            ParseError::NoDataError => write!(f, "No data available"),
            ParseError::InvalidMapsTypeError => write!(f, "Invalid MapsType"),
        }
    }
}

impl StdError for ParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ParseError::Utf8Error(e) => Some(e),
            ParseError::IntError(e) => Some(e),
            ParseError::FloatError(e) => Some(e),
            ParseError::NoDataError => None,
            ParseError::InvalidMapsTypeError => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapsType {
    Node,
    Way,
    Relation,
    Other(&'static str),  // Use &'static str to allow literal string references
}

impl MapsType {
    pub fn as_str(&self) -> &str {
        match self {
            MapsType::Node => "node",
            MapsType::Way => "way",
            MapsType::Relation => "relation",
            MapsType::Other(s) => s,
        }
    }
}

impl FromStr for MapsType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "node" => Ok(MapsType::Node),
            "way" => Ok(MapsType::Way),
            "relation" => Ok(MapsType::Relation),
            other => Ok(MapsType::Other(Box::leak(Box::new(other.to_string())))),
        }
    }
}

/// Represents a pairing of a way ID with a tag.
#[derive(Debug, Clone)]
pub struct MapsTag {
    pub id: i64,
    pub tag: Tag,
}

impl MapsTag {
    /// Collects maps IDs, tag keys, and tag values from a slice of MapsTag structs.
    ///
    /// # Arguments
    /// * `maps_tags` - A slice of MapsTag structs to collect data from.
    ///
    /// # Returns
    /// A tuple of three vectors containing maps IDs, tag keys, and tag values respectively.
    pub fn collect_tag_data(maps_tags: &[MapsTag]) -> (Vec<i64>, Vec<&str>, Vec<&str>) {
        let mut ids = Vec::new();
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for maps_tag in maps_tags {
            ids.push(maps_tag.id);
            keys.push(maps_tag.tag.key.as_str());
            values.push(maps_tag.tag.value.as_str());
        }

        (ids, keys, values)
    }
}

pub fn lat_lon_to_screen(lat: f64, lon: f64, top_left: (f64, f64), bottom_right: (f64, f64)) -> (f32, f32) {
    // Normalize the longitude (x-axis)
    let normalized_x = (lon - top_left.1) / (bottom_right.1 - top_left.1);

    // Normalize the latitude (y-axis), invert to account for the natural increase in latitudes as you move north
    let normalized_y = (top_left.0 - lat) / (top_left.0 - bottom_right.0);

    // Map to the range [-1, 1] for NDC
    let screen_x = normalized_x * 2.0 - 1.0;
    let screen_y = normalized_y * 2.0 - 1.0;

    (screen_x as f32, screen_y as f32)
}
