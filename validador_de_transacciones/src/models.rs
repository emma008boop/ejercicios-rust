/* pub trait Verificable {
    fn realizar_verificacion(&self) -> bool;
} */
pub struct Transaccion {
    pub emisor: String,
    pub receptor: String,
    pub monto: i32
}

impl Transaccion {

    pub async fn validar_remoto(&self) -> bool {
        println!("Conectando con el nodo central para validar...");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        if self.emisor == self.receptor { return false; }
        self.monto > 99
    }

}
/*
impl Verificable for Transaccion {
    fn realizar_verificacion(&self) -> bool{
        if self.emisor == self.receptor {
            return false;
        }
        
        match self.monto {
            i32::MIN..=99 => false,
            _ => true, 
        }
        
    }   
}
*/
pub struct Usuario {
    pub edad: i32,
    pub nombre: String,
} 

impl Usuario {
    pub async fn validar_remoto(&self) -> bool {
        println!("Conectando con el nodo central para validar...");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        match self.edad {
            i32::MIN..=17 => false,
            _ => true,
        }

    }
}

/*
impl Verificable for Usuario {
    fn realizar_verificacion(&self) -> bool {
        match self.edad {
            i32::MIN..=17 => false,
            _ => true,
        }
    }
}
*/