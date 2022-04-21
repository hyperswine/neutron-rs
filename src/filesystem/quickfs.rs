// ----------------
// QUICK FILESYSTEM
// ----------------

// A FAT32 like filesystem
// implemented quickly

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

const CLUSTER_SIZE_BYTES: u64 = 4192;
use alloc::collections::btree_map::BTreeMap;

#[derive(Clone)]
pub struct FileAllocationTable {
    clusters: Vec<u32>,
    file_mappings: BTreeMap<String, u32>,
    free_clusters: Vec<u32>,
}

impl FileAllocationTable {
    pub fn new(
        clusters: Vec<u32>,
        file_mappings: BTreeMap<String, u32>,
        free_clusters: Vec<u32>,
    ) -> Self {
        Self {
            clusters,
            file_mappings,
            free_clusters,
        }
    }

    // request k blocks for your file/dir. Allocates them directly if file found and disk no overflow (file size or partition size or > 32GB)
    pub fn request_blocks(&mut self, path: &str, k: usize) -> bool {
        // find file index from file -> cluster mapping
        let res = self.file_mappings.iter().find(|f| f.0 == path);

        let ind = match res {
            Some(r) => r.1,
            None => return false,
        }
        .to_owned();

        // get the first block
        match self.clusters.get_mut(ind as usize) {
            Some(c) => {
                // keep chaining lookups until you get to c = 0xFF
                if c.to_owned() == 0xFF {
                    // this is the final block of the file
                    // take the first k free indexes and link them
                    let free_inds: Vec<u32> = self.free_clusters.drain(..k).collect();
                    // allocate each
                    self.clusters[ind as usize] = free_inds[0];
                    let mut prev_ind = ind;
                    let _len = free_inds.len();

                    for i in 0.._len {
                        prev_ind = self.clusters[prev_ind as usize];
                        self.clusters[prev_ind as usize] = free_inds[i];
                    }

                    self.clusters[*free_inds.last().unwrap() as usize] = 0xFF;
                }
            }
            None => return false,
        }

        true
    }

    // deallocate blocks for a file, e.g. when deleting
    // maybe an interface to specify how to dealloc, how many, what indices
    // maybe the bytes/lines/ranges in bytes you want to compare to and dealloc those blocks
    pub fn dealloc_blocks(&self) {
        todo!()
    }
}
