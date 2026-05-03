use validador_de_transacciones::models::{Transaccion, Usuario};
#[tokio::main]
async fn main() {
   let tx = Transaccion { 
        emisor : String::from("Alice"),
        receptor : String::from("Emma"),
        monto: 250,
   };

   let usuario = Usuario{
     edad: 18,
     nombre: String::from("Emma"),
   };

   let registro = usuario.validar_remoto().await;
   let es_valida = tx.validar_remoto().await;
   println!("Resultado final {}", es_valida);
   println!("El usuario puede registrate? {}", registro)

}
