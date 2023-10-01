
use std::vec::Vec;
use std::string::String;

#[derive(Default,Debug)]
pub struct MediaOverlayNode {
    text: String,
    audio: String,
    role: Vec<String>,
    children: Vec<MediaOverlayNode>
}
