pub trait Notificador {
    fn enviar(&self, mensaje: &str, destinatario: &str) -> Result<(), String>;
}

pub mod email;
pub mod sms;
pub mod telegram;
