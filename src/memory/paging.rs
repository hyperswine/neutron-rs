// Useful Types

use core::{convert::From, iter::Step, num::NonZeroUsize, ops::Range};

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum MemAttributes {
    CacheableDRAM,
    Device,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct AttributeFields {
    pub mem_attributes: MemAttributes,
    pub acc_perms: AccessPermissions,
    pub execute_never: bool,
}
