pub mod entry;
// pub mod exception;

// NOTE: use the logger from arcboot, or the entire terminal
// Its possible to just debug to UART while the kernel subsystems start, until userspace shows up
// And you can launch spx:system that sets up a system/software shell and everything else, then info! gets piped to an actual console
