use super::Notificador;

pub struct EmailNotifier {
    pub servidor_smtp: String,
}

impl Notificador for EmailNotifier {
    fn enviar(&self, mensaje: &str, destinatario: &str) -> Result<(), String> {
        if mensaje.is_empty() {
            Err(String::from("El mensaje no puede estar vacio"))
        } else {
            println!("Enviando mensaje a {}: {}", destinatario, mensaje);
            Ok(())
        }
    }
}
