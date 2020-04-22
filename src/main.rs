use regex::Regex;
use rustbox::{self, Color, Event, InitOptions, Key, RustBox};
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::time;

mod fonts;

fn main() {
    let argumentos: Vec<String> = env::args().skip(1).collect();
    if argumentos.len() != 1 {
        let programa: String = env::args().next().unwrap();
        eprintln!("Error en el uso del programa. Solo debe utiliar un parámetro.");
        eprintln!("Uso:");
        eprintln!("  {} 25s", programa);
        eprintln!("  {} 1m50s", programa);
        eprintln!("  {} 2h45m50s", programa);
        exit(2);
    } else if es_valido(&argumentos[0]) == false {
		let programa: String = env::args().next().unwrap();
		eprintln!("Error en el formato introducido: '{}'",argumentos[0]);
		eprintln!("Uso:");
        eprintln!("  {} 25s", programa);
        eprintln!("  {} 1m50s", programa);
        eprintln!("  {} 2h45m50s", programa);	
        exit(2);
	}

    let tiempo_definido = formatear_duracion(&argumentos[0]);
    let inicio = time::Instant::now();

    let mut exit_code = 0;

    if let Ok(rust_box) = RustBox::init(InitOptions::default()) {
        let tabla = fonts::tabla_simbolos();
        let marco_milisegundos = time::Duration::from_millis(16);

        loop {
            let transcurrido = inicio.elapsed();
            if tiempo_definido < transcurrido {
                exit_code = 0;
                simple_message_box::create_message_box("SE ACABO EL TIEMPO DE ESTA TAREA.  DEFINIR OTRA", "%%%%%%%%% TIEMPO %%%%%%%%%%%%");
                break;
            }
            let falta = tiempo_definido - transcurrido;

            dibujar(&rust_box, falta.as_secs(), &tabla);

            if let Event::KeyEvent(key) = rust_box.peek_event(marco_milisegundos, false).unwrap() {
                if key == Key::Esc || key == Key::Ctrl('c') {
                    exit_code = 1;
                    break;
                }
            }
        }
    }

    exit(exit_code);
}

fn formatear_duracion(duration: &str) -> time::Duration {
    let re = Regex::new(r"((?P<hour>\d+)h)?((?P<minute>\d+)m)?((?P<second>\d+)s)?").unwrap();
    let caps = re.captures(duration).unwrap();
    let h: u64 = caps.name("hour").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minute").map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("second").map_or(0, |m| m.as_str().parse().unwrap());
    time::Duration::new(3600 * h + 60 * m + s, 0)
}

fn dibujar(rb: &RustBox, falta: u64, tabla: &HashMap<char, ([&str; 6], usize)>) {
    let formateado = formatear(falta);
    let simbolos = formateado.chars().map(|c| tabla[&c]);

    let w_sum = simbolos.clone().map(|(_, w)| w).fold(0, |sum, w| sum + w);
    let start_x = rb.width() / 2 - w_sum / 2;
    let start_y = rb.height() / 2 - 3;

    rb.clear();

    let mut x = start_x;
    for (symbol, w) in simbolos {
        mostrar(rb, &symbol, x, start_y);
        x += w;
    }

    rb.present();
}

fn mostrar(rb: &RustBox, symbol: &[&str], start_x: usize, start_y: usize) {
    let mut y = start_y;
    for line in symbol {
        rb.print(start_x, y, rustbox::RB_NORMAL, Color::Default, Color::Default, line);
        y += 1;
    }
}

fn formatear(valor: u64) -> String {
    let (horas, minutos, segundos) = (valor / 3600, (valor % 3600) / 60, valor % 60);
    if horas == 0 {
        format!("{:02}:{:02}", minutos, segundos)
    } else {
        format!("{:02}:{:02}:{:02}", horas, minutos, segundos)
    }
}

fn es_valido(valor: &str) -> bool{
	//println!("el valor itroducido es: {}",valor);
	let re = Regex::new(r"^[\d{1,2}h]*[\d{1,2}m]*\d{1,2}s$").unwrap();
	//println!("es: {}",re.is_match(valor));
	re.is_match(valor)
}
