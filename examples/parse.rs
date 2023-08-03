use anyhow::Result;
use bde::Value;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use url::Url;

fn main() -> Result<()> {
    let bytes = b"d6:bananai123e3:car3:rede".to_vec();

    let parsed: Value = bde::from_bytes(&bytes)?;

    dbg!(parsed);

    // let bytes = fs::read("tests/torrents/debian.torrent")?;

    // let parsed: MetaInfo = bde::from_bytes(&bytes)?;

    // dbg!(parsed);

    Ok(())
}

/// Dictionary containg information about the torrent
#[derive(Debug, Deserialize, Serialize)]
pub struct MetaInfo {
    /// The announce url of the tracker.
    /// According to the specification this is always set.
    /// In the real world most torrents ditch it in favor of announce list or trackless peers
    ///
    /// The url supports http tracking via get requests and udp tracking. It is worth noting that many trackers will accept either protocols regardless of the one specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce: Option<Url>,
    /// A list of list of announce urls.
    #[serde(rename = "announce-list", skip_serializing_if = "Option::is_none")]
    pub announce_list: Option<Vec<Vec<Url>>>,
    /// An optional comment about this torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Name of version of the program used to create the torrent
    #[serde(rename = "created by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Time of creation represented in millisecond since [UNIX epoch][`std::time::UNIX_EPOCH`]
    #[serde(rename = "creation date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<u64>,
    /// The encoding format used by [pieces][`Info::pieces`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Unknown field
    #[serde(rename = "httpseeds", skip_serializing_if = "Option::is_none")]
    pub http_seeds: Option<Vec<String>>,
    /// A dictionary containing information about the file(s) of the torrent
    pub info: Info,
    // TODO: docs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<Url>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    /// The name of the file or directory to store multiple files, respecting this field is not mandatory
    pub name: String,
    /// The number of bytes in each piece
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    /// String consisting of the concatenation of all 20-byte SHA1 hash values, one per piece
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
    /// When set to 1 clients should only announce their presence via the tracker specified by the torrent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<u8>,
    /// Unknown field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(flatten)]
    pub files: FileKind,
}

/// A dictionary containing information about the file(s) of the torrent
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum FileKind {
    // Information about multiple files
    MultiFile {
        /// A list of dictionaries, each containing information about one file
        files: Vec<File>,
    },

    /// Information about a single file
    SingleFile {
        /// Length of the file in bytes
        length: u64,
        /// MD5 sum of the file
        #[serde(skip_serializing_if = "Option::is_none")]
        md5sum: Option<String>,
    },
}

/// Dictionary containing information about a file
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    /// Length of the file in bytes
    pub length: u64,
    /// MD5 sum of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
    /// A list where each element corresponds to either a directory name or (in the case of the final element) the filename
    pub path: Vec<String>,
}
