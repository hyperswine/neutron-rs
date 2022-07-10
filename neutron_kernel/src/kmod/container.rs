// --------------
// KERNEL CONTAINERS
// --------------

// Image -> Container -> Volume

// --------------
// Arc Compose
// --------------

// To build an image, best to use a compose file
// which allows multi container apps instead of only a single container or single volume
// https://docs.docker.com/compose/compose-file/

// FORMAT:
/*
services:
    <service_name>
volumes:
    db_data:
        driver:
config:
    http:
        secure:
secrets:
    cert:
networks:
    front_tier:
    back_tier:
*/

/*
NEW PROFILE uses TOML:
[services]
[volumes]
[config]
[networks]
*/

struct NeutronContainerCompose {
    // services: Services,
    // volumes: Volumes,
}

struct VirtualFS;

struct ImageSignature;

// Neutron Container Image Format
// Basically an ELF like format mostly concerned with data rather than instructions and a virtual FS layout/hierarchy and specific files
// with a main header and a section header right afterwards
// no program header or symtab

struct NeutronContainerHeader {
    name: String,
    tag: String,
    // should be either aarch64 or riscv64gc
    arch: String,
    signature: ImageSignature,
}

struct Image {
    header: NeutronContainerHeader,
    data: Vec<u8>,
}

pub struct Container {
    virtual_root_fs: VirtualFS,
}

// Persistent Data in the actual root fs /containers
struct Volume;

// ROOT FS Hierarchy Extension = /container

use alloc::{string::String, vec::Vec};
