pub trait Verificable {
    fn realizar_verificacion(&self) -> bool;
}
pub struct Transaccion {
    pub emisor: String,
    pub receptor: String,
    pub monto: i32
}

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
pub struct Usuario {
    edad: i32
}

impl Verificable for Usuario {
    fn realizar_verificacion(&self) -> bool {
        match self.edad {
            i32::MIN..=17 => false,
            _ => true,
        }
    }
}