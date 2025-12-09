use clap::Parser;
use std::process::Command;

/// Programa para ajustar brillo usando brightnessctl.
/// - Sin argumentos: muestra el brillo actual.
/// - Con argumento 0–99: ajusta el brillo.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Valor de brillo (0-99). Si se omite, muestra el brillo actual.
    #[arg(required = false)]
    brillo: Option<u8>,
}

fn main() {
    let args = Args::parse();

    // ---------------------------------------------------------
    // Verificar si brightnessctl está instalado
    // ---------------------------------------------------------
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

    // ---------------------------------------------------------
    // Si no hay argumento → mostrar brillo actual
    // ---------------------------------------------------------
    if args.brillo.is_none() {
        let current = Command::new("brightnessctl")
            .arg("get")
            .output()
            .expect("No se pudo obtener el brillo actual");
        
        let max = Command::new("brightnessctl")
            .arg("max")
            .output()
            .expect("No se pudo obtener el brillo máximo");

        let current_val: f32 = String::from_utf8_lossy(&current.stdout)
            .trim()
            .parse()
            .unwrap_or(0.0);

        let max_val: f32 = String::from_utf8_lossy(&max.stdout)
            .trim()
            .parse()
            .unwrap_or(1.0);

        let percent = (current_val / max_val) * 100.0;

        println!("Brillo actual: {:.0}%", percent);
        return;
    }

    // ---------------------------------------------------------
    // Si hay argumento → ajustar el brillo
    // ---------------------------------------------------------
    let brillo = args.brillo.unwrap();

    if brillo > 99 {
        eprintln!("El valor debe estar entre 0 y 99");
        std::process::exit(1);
    }

    let value = format!("{}%", brillo);

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

