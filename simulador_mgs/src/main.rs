use emma::notifier::{
    Notificador, email::EmailNotifier, sms::SmsNotifier, telegram::TelegramNotifier,
};

fn main() {
    let servicios: Vec<Box<dyn Notificador>> = vec![
        Box::new(EmailNotifier {
            servidor_smtp: String::from("smtp.gmail.com"),
        }),
        Box::new(SmsNotifier),
        Box::new(TelegramNotifier {
            servidor_telegram: String::from("API_BOT_99"),
        }),
    ];

    let mensaje = "Alerta: El sistema detecto un reinicio";
    let usuario = "admin01";

    println!("--- Iniciando envio masivo ---");
    for servicio in &servicios {
        if let Err(e) = servicio.enviar(&mensaje, &usuario) {
            println!("Error detectado: {}", e);
        }
    }
}
