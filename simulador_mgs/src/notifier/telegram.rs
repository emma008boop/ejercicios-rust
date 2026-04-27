use super::Notificador;

pub struct TelegramNotifier {
    pub servidor_telegram: String,
}

impl Notificador for TelegramNotifier {
    fn enviar(&self, mensaje: &str, destinatario: &str) -> Result<(), String> {
        if mensaje.is_empty() && destinatario.is_empty() {
            Err(String::from("Fijese que haya destinatiario y un mensaje"))
        } else {
            println!("Enviando telegram a {}: {}", destinatario, mensaje);
            Ok(())
        }
    }
}
