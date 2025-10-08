// --- Sistema Generador de Horarios - Archivo principal ---

mod models;
mod excel;
mod algorithms;

use algorithms::{get_ramo_critico, extract_data, get_clique_max_pond};

fn main() {
    println!("=== Sistema Generador de Horarios ===\n");
    
    // Paso 1: Obtener ramos críticos
    let (ramos_disponibles, nombre_excel_malla) = get_ramo_critico();
    
    println!("\nExtrayendo datos...\n");
    
    // Paso 2: Extraer datos de secciones
    let (lista_secciones, ramos_actualizados) = extract_data(&ramos_disponibles, &nombre_excel_malla);
    
    // Paso 3: Generar recomendaciones de horarios (ahora devuelve soluciones)
    let soluciones = get_clique_max_pond(&lista_secciones, &ramos_actualizados);
    println!("\nSe encontraron {} soluciones recomendadas", soluciones.len());

    for (i, sol) in soluciones.iter().enumerate() {
        println!("\n-- Solución #{} --", i + 1);
        for (sec, prio) in sol {
            println!("{} | {} | Sec: {} | Prio: {}", sec.codigo, sec.nombre, sec.seccion, prio);
        }
    }
    
    println!("\n=== Proceso completado ===");
}
