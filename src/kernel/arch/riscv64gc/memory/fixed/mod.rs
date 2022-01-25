// Fixed Size Block Allocator and its derivatives
pub mod buddy;
pub mod slab;

// TODO: heap allocator based on slab allocation by default

// So use allocate(nbytes)
// and deallocate(nbytes)
// use these two functions to implement Box::new(), Dyn::new(), Vec::new() and vec!()

// heap -> allocates variable chunks of data from a certain address
// uses the paging system to acquire frames for a process allocating the space
// heap always located at a certain region. Should be fine since its 64bit i think

// linked list and stuff too for backup allocation and special cases like initial allocation


// a linked list for each fixed size block i=4..12. So 12-4 = 8 entries in the array
// so [LinkedList; 8]
// [0] => 2^4 LL(4)
// [1] => 2^5 LL(5)
// ...
// [7] => 2^12 LL(12)

struct MemoryList{
    size: u8,
    head: Node
}

// just a linked list
impl MemoryList {
    fn new() -> MemoryList {
        MemoryList {}
    } 
}

// ! for now just allocate as much space as needed to store the metadata for all the nodes
// though it makes sense at boot or during idle to not have to allocate the nodes and instead specify the amount of nodes expected to be available
struct Node {
    // mutable number of bytes of size "size"
    data: *mut u8,
    // so the allocator knows which list to put a node back in
    size: u8,
}

const DEFAULT_MEM_LIST_SIZ: u8 = 8;

// store blocks of 2^i bytes for i=4..12
struct FixedAllocator{
    linked_lists: [MemoryList; DEFAULT_MEM_LIST_SIZ],
}

impl FixedAllocator {
    pub fn new() -> FixedAllocator {
        // set the sizes for each list (of free memory blocks)
        let n_16 = 9;


        FixedAllocator {}
    }

    // should return the allocated node pointer
    pub fn alloc(n_bytes: u64) -> &Node {
        Node{}
    }

    // if operation was a success and memory was freed and placed back, return true
    // else if something weird happened or the node was already freed, return false (for testing)
    pub fn dealloc(node: &Node) -> bool {
        true
    }
}


