mod model;

#[cfg(not(windows))]
use crate::model::model::Validador; 
fn main() {
    #[cfg(not(windows))]
    {
        let v = Validador { id: 1 };
        v.iniciar_servidor();
    }
    #[cfg(windows)]
    println!("Lo siento, este nodo es solo para Linux/Unix");
}
