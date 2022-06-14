pub type PertId = i32;

#[derive(Debug)]
pub struct Pert {
    id: PertId,
    pub name: String,
}

impl Pert {
    pub fn new(id: PertId, name: String) -> Self {
        Self { id, name }
    }
}

