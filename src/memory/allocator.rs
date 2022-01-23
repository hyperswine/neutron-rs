// TODO: heap allocator based on slab allocation by default

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
    fn new(self) -> {
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
    fn new(self) -> FixedAllocator {
        // set the sizes for each list (of free memory blocks)
        let n_16 = 9;


        FixedAllocator {}
    }
}

struct SlabAllocator;

// store blocks of 2^i bytes for i=4..12
struct BuddyAllocator{
    start_i: u32,
    end_i: u32,
    
}

// type BuddySize = u16; // later

impl BuddyAllocator {
    fn new(self) -> BuddyAllocator {
        BuddyAllocator{}
    }

    fn alloc(self, size: u64) {
        // choose the next highest size
    }

    // deallocate a node, placing it back at the head of the node's linked list (size siz)
    fn dealloc(self, node: &mut Node) {

    }
}
