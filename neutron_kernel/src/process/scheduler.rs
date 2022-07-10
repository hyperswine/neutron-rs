// SMP Scheduling

// So you have m processes and n processors
// Some threads want to access resources like a PCIe device or DRAM, which are all the same distance

// NUMA Scheduling

// So you have m processes and n processors
// Some threads want to access resources which may be closer or farer away from a certain processor
// A thread that wants to access a certain node should be scheduled on a processor node nearer to the resource. Need some API in the driver or scheduler/interrupt handler that determines it
