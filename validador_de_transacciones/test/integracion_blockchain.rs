use validador_de_transacciones::models::{Transaccion, Verificable};

#[test]
fn falla_si_el_nombre_es_el_mismo() {
    let usuario_es_el_mismo = Transaccion{
        emisor: String::from("Emma"),
        receptor: String::from("Emma"),
        monto: 200,
    };
    assert_eq!(usuario_es_el_mismo.realizar_verificacion(), false)
}
#[test]
fn falla_si_el_monto_es_menor() {
    let monto_insuficiente = Transaccion{
        emisor: String::from("Emma"),
        receptor: String::from("Stacy"),
        monto: 10,
    };
    assert_eq!(monto_insuficiente.realizar_verificacion(), false)
}
#[test]
fn pasa() {
    let monto_insuficiente = Transaccion{
        emisor: String::from("Emma"),
        receptor: String::from("Stacy"),
        monto: 200,
    };
    assert!(!monto_insuficiente.realizar_verificacion())
}

