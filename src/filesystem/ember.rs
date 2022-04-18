// Semantic Filesystem
// An instance of Ember is created on top of an HFS as a key-value store database file

use alloc::{string::String, string::ToString, vec, vec::Vec};
use core::option::*;

// Each Ember instance has a single header with a unique ID and a key val store of [file_name: pointer_to_file]
// NOTE: within the kernel, we cant use HashMap so either implement it using KRandom() and Vec<T>, Vec<K> or just use an array
// and search through it for now
struct EmberFSHeader {
    // REQUIRED
    database: Vec<EmberFile>,

    // CACHE USEFUL INFO
    // cache the number of files for easy diagnostics
    n_files: u64,
    // last modified time (unix-64)
    last_modified: u64,
}

// All metadata is stored within the file itself. All files must have a unique name + tag combination
struct EmberFile {
    name: String,
    tags: Vec<EmberTag>,
}
impl EmberFile {
    pub fn new(_name: &str, def_tag: &str) -> Self {
        Self {
            name: _name.to_string(),
            tags: vec![EmberTag::new(def_tag)],
        }
    }
    // only returns a mutable reference that does not change this ref
    pub fn name(&mut self) -> &mut str {
        &mut self.name
    }
    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }
    pub fn get_tags(&self) -> &[EmberTag] {
        &self.tags
    }
    // if tag with the same name does not exist, add it
    pub fn add_tag(&mut self, tag_name: &str) -> bool {
        // can also use iter_mut() to do stuff like in place editing
        match self.tags.iter().find(|t| t.get_name() == tag_name) {
            Some(_val) => false,
            None => {
                self.tags.push(EmberTag::new(tag_name));
                true
            }
        }
    }
    // if tag with the same name does not exist, add it
    pub fn add_full_tag(&mut self, tag: EmberTag) -> bool {
        // can also use iter_mut() to do stuff like in place editing
        match self.tags.iter().find(|t| t.get_name() == tag.get_name()) {
            Some(_val) => false,
            None => {
                self.tags.push(tag);
                true
            }
        }
    }
    pub fn remove_tag(&mut self, tag_: &str) {
        // search for tags that match the string, if found, remove
        // If tags contain other info, consider later
        self.tags.retain(|t| t.get_name() != tag_);
    }
}

struct EmberTag {
    name: String,
}
impl EmberTag {
    pub fn new(tag_name: &str) -> Self {
        Self {
            name: tag_name.to_string(),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[test_case]
fn create_ember_file() {
    let mut _file = EmberFile::new("/root", "root");
    let mut r = _file.name();
    assert_eq!(r, "/root");

    let mut g = "/boot".to_string();
    r = &mut g;
    assert_eq!(r, "/boot");

    // that changes r but not _file.name()
    // println!("_file.name() = {}", _file.name());
    // assert_eq!(_file.name(), "/boot");

    _file.set_name("/boot");
    // println!("_file.name() = {}", _file.name());
    assert_eq!(_file.name(), "/boot");
}

#[test_case]
fn create_tag() {
    let secret_tag = EmberTag::new("secrets");
    let study_tag = EmberTag::new("study");

    assert_eq!(secret_tag.get_name(), "secrets");
    assert_eq!(study_tag.get_name(), "study");

    let mut _file = EmberFile::new("World History", "study");
    // should not be added since already exists
    let r = _file.add_tag("study");
    // println!("Adding tag 'study' = {}", r);

    let r = _file.get_tags();
    let _str = match r.first() {
        Some(v) => v.get_name(),
        None => "",
    };
    // println!("_str = {}", _str);

    assert_eq!(_str, study_tag.get_name());

    // Add a full tag
    _file.add_full_tag(secret_tag);
    let r = _file.get_tags();
    let _str = match r.first() {
        Some(v) => v.get_name(),
        None => "",
    };
    for _r in r {
        // println!("r = {}", _r.get_name());
    }
    // println!("_str = {}", _str);

    // should still be study as the first tag
    assert_eq!(_str, study_tag.get_name());
}
