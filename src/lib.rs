pub mod package_type;

use std::collections::HashMap;
use bincode::{Encode, Decode, encode_to_vec, decode_from_slice, config::standard};
use anyhow::Result;

pub type CommandFn = fn(&[u8]);



#[derive(Encode, Decode, Debug)]
pub struct Packet {
    pub command: String,
    pub data: Vec<u8>,
}

#[derive(Encode, Decode, Debug)]
pub struct MoveTo {
    pub x: f64,
    pub y: f64,
}

fn print_bytes_hex(label: &str, bytes: &[u8]) {
    print!("{} ({} bytes): ", label, bytes.len());
    for b in bytes {
        print!("{:02X} ", b);
    }
    println!();
}

fn handle_move_to(data: &[u8]) {
    print_bytes_hex("Datos recibidos para MoveTo", data);
    let result = decode_from_slice::<MoveTo, _>(data, standard());
    match result {
        Ok((cmd, _)) => {
            println!("Ejecutando MoveTo → x: {}, y: {}", cmd.x, cmd.y);
        }
        Err(err) => {
            eprintln!("Error al deserializar MoveTo: {:?}", err);
        }
    }
}

fn get_command_registry() -> HashMap<&'static str, CommandFn> {
    let mut map: HashMap<&str, CommandFn> = HashMap::new();
    map.insert("move_to", handle_move_to);
    map
}

pub fn simulate_command_execution() -> Result<()> {
    // Crear comando
    let move_cmd = MoveTo { x: 10.0, y: 5.0 };
    println!("Comando a enviar: {:?}", move_cmd);

    // Serializar comando
    let data = encode_to_vec(&move_cmd, standard())?;
    print_bytes_hex("Datos serializados (payload)", &data);

    // Empaquetar con nombre del comando
    let packet = Packet {
        command: "move_to".to_string(),
        data,
    };
    println!("Paquete a enviar: {:?}", packet);

    // Serializar paquete completo
    let serialized_packet = encode_to_vec(&packet, standard())?;
    print_bytes_hex("Paquete serializado completo", &serialized_packet);

    // Simular recepción y ejecución
    handle_packet(&serialized_packet);

    Ok(())
}

fn handle_packet(bytes: &[u8]) {
    print_bytes_hex("Paquete recibido (bytes)", bytes);

    let result = decode_from_slice::<Packet, _>(bytes, standard());
    match result {
        Ok((packet, _)) => {
            println!("Paquete deserializado: {:?}", packet);

            let registry = get_command_registry();
            match registry.get(packet.command.as_str()) {
                Some(handler) => handler(&packet.data),
                None => eprintln!("Comando desconocido: {}", packet.command),
            }
        }
        Err(err) => {
            eprintln!("Error al deserializar paquete: {:?}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command_execution() {
        simulate_command_execution().unwrap();
    }
}
