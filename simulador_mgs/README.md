------------ ESTRUCTURA 

proyecto_notificaciones/
├── Cargo.toml
└── src/
    ├── main.rs         (Punto de entrada)
    ├── lib.rs          (Define los módulos)
    └── notifier/       (Carpeta del módulo)
        ├── mod.rs      (Declaraciones del submódulo)
        ├── email.rs    (Implementación Email)
        └── sms.rs      (Implementación SMS)