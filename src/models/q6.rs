use serde::Serialize;

#[derive(Serialize)]
pub struct ResElfString{
    pub elf:usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    pub elf_on_a_shelf:usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    pub shelf_no_elf:usize,
}