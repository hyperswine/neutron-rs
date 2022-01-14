pub struct ProcessManager{
    processes: Vec<Process> //could sort by id, name, space, 
}

pub struct Process{
    id: u32,
    name: String, //owns the name
    space_allocated: u32, //RAM stuff
    space_used: u32,
    status: ProcessStatus,
    pages: Vec<Page>
}

//* temp for paging
struct Page;

enum ProcessStatus {
    UP, DOWN, BLOCKED
}
