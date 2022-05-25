use goblin::*;

pub struct Object;
impl Object {
    pub fn parse(contents: &str) {}
    pub fn Elf(_: ()) {}
    pub fn Unknown() {}
}

// pub fn load_elf(_file_contents: &str) -> Result<(), ()> {
//     let data = match Object::parse(_file_contents) {
//         Object::Elf(elf) => {
//             elf
//         }
//         Object::Unknown(magic) => {
//             ()
//         }
//     };

//     Ok(data)
// }
