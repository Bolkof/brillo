# 🔆 Control de Brillo en Linux (Rust + brightnessctl)

Este es un pequeño programa escrito en **Rust** que permite ajustar el brillo de la pantalla en Linux utilizando el comando `brightnessctl`.  
El programa recibe un número entero de 0 a 99 y ajusta directamente el brillo al porcentaje indicado.

---

## 🚀 Características

- ✔️ Recibe un valor entero entre **0 y 99**
- ✔️ Comprueba si `brightnessctl` está instalado  
- ✔️ Si no lo está, muestra un mensaje indicando cómo instalarlo
- ✔️ Intenta ejecutar `brightnessctl` directamente
- ✔️ Si no tiene permisos, vuelve a intentarlo usando `sudo`
- ✔️ Muestra mensajes claros de error y éxito

---

## 📦 Dependencias

Este proyecto usa:

- **Rust** (edition 2021)
- **Clap** 4.x → para manejar argumentos de línea de comandos

En tu `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
````

---

## 🛠️ Instalación de brightnessctl

Antes de usar el programa, asegúrate de que `brightnessctl` está instalado:

```bash
sudo apt install brightnessctl
```

Si no está instalado, el programa te mostrará exactamente este mensaje.

---

## 🧩 Uso

Compila el programa:

```bash
cargo build --release
```

Ejecuta el binario pasando un valor entre 0 y 99:

```bash
./target/release/brillo 75
```

Esto ajustará el brillo a **75%**.

---

## 🔐 Permisos

Algunos sistemas requieren permisos especiales para cambiar el brillo.

Si aparece:

```
Permission denied
```

el programa intentará usar automáticamente `sudo`.
También puedes ejecutar manualmente:

```bash
sudo ./brillo 75
```

###💡 O puedes permitir a tu usuario modificar el brillo sin sudo:

```bash
sudo usermod -aG video $USER
sudo chmod g+w /sys/class/backlight/*/brightness
```

Luego cierra sesión y vuelve a entrar.

---

## 🎯 Alias recomendado

Puedes crear un alias para ejecutarlo fácilmente:

```bash
alias brillo='/ruta/a/tu/proyecto/target/release/brillo'
```

Ejemplo de uso:

```bash
brillo 30
```

---

## 📄 Código fuente (main.rs)

```rust
use clap::Parser;
use std::process::Command;

/// Programa para ajustar brillo usando brightnessctl.
/// Ejemplo: ./brillo 50
#[derive(Parser)]
struct Args {
    /// Valor de brillo (0-99)
    brillo: u8,
}

fn main() {
    let args = Args::parse();

    if args.brillo > 99 {
        eprintln!("El valor debe estar entre 0 y 99");
        std::process::exit(1);
    }

    let value = format!("{}%", args.brillo);

    // 1. Comprobación de si brightnessctl está instalado
    let check = Command::new("which")
        .arg("brightnessctl")
        .output()
        .expect("No se pudo ejecutar el comando 'which'");

    if !check.status.success() {
        eprintln!(
            "Error: 'brightnessctl' no está instalado.\n\
Por favor instalar con el comando:\n\
sudo apt install brightnessctl"
        );
        std::process::exit(1);
    }

    // 2. Intento normal sin sudo
    let attempt = Command::new("brightnessctl")
        .arg("set")
        .arg(&value)
        .output();

    match attempt {
        Ok(output) => {
            if output.status.success() {
                println!("Brillo ajustado a {}", value);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);

                // Si falta permiso → reintentar con sudo
                if stderr.contains("Permission denied") {
                    println!("Permiso denegado. Reintentando con sudo...");

                    let sudo_attempt = Command::new("sudo")
                        .arg("brightnessctl")
                        .arg("set")
                        .arg(&value)
                        .status()
                        .expect("Error ejecutando sudo");

                    if sudo_attempt.success() {
                        println!("Brillo ajustado a {} con sudo", value);
                    } else {
                        eprintln!("No se pudo ajustar el brillo incluso con sudo.");
                    }
                } else {
                    eprintln!("Error: {}", stderr);
                }
            }
        }
        Err(err) => {
            eprintln!("No se pudo ejecutar brightnessctl: {}", err);
            std::process::exit(1);
        }
    }
}
```

---

## 📦 Licencia

Puedes usar y modificar este proyecto libremente.

---

## ❤️ Contribuciones

¡Se aceptan sugerencias o mejoras!
Si quieres agregar funciones como:

* ⭐ Subir/bajar brillo con `--inc` y `--dec`
* ⭐ Mostrar brillo actual
* ⭐ Validación estricta de dos dígitos
