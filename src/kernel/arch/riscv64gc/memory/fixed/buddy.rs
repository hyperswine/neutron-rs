// store blocks of 2^i bytes for i=4..12
struct BuddyAllocator{
    start_i: u32,
    end_i: u32,
    
}

// type BuddySize = u16; // later

impl BuddyAllocator {
    fn new() -> BuddyAllocator {
        BuddyAllocator{}
    }

    fn alloc(&self, size: u64) {
        // choose the next highest size
    }

    // deallocate a node, placing it back at the head of the node's linked list (size siz)
    fn dealloc(&self, node: &mut Node) {

    }
}