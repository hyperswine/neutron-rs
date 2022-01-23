// TODO: heap allocator based on slab allocation by default

// linked list and stuff too for backup allocation and special cases like initial allocation

// store blocks of 2^i bytes for i=4..12
struct FixedAllocator;

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
}
