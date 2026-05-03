use validador_de_transacciones::models::{Transaccion, Verificable};
fn main() {
   let transaccion_nueva = Transaccion { 
        emisor : String::from("Alice"),
        receptor : String::from("bob"),
        monto: 250,
   };

   if transaccion_nueva.realizar_verificacion() {
        println!("Transaccion verificada")
   } else {
        println!("No se pudo verificar la transaccion");
   }

}
