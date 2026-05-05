#[cfg(not(windows))]
pub struct Validador {
    pub id: u32,
}

#[cfg(not(windows))]
impl Validador {
    pub fn iniciar_servidor(&self) {
        println!("Validador {} iniciando en Linux/Unix", self.id);
    }
}