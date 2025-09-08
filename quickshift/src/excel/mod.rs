// Funciones para leer archivos Excel

use std::collections::HashMap;
use calamine::{Reader, Xlsx, open_workbook, Data};
use crate::models::{Seccion, RamoDisponible};

// Función para leer Excel de malla curricular
pub fn leer_malla_excel(nombre_archivo: &str) -> Result<HashMap<String, RamoDisponible>, Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(nombre_archivo)?;
    let mut ramos_disponibles = HashMap::new();
    
    // Obtener la primera hoja disponible en lugar de buscar "Sheet1" específicamente
    let sheet_names = workbook.sheet_names().to_owned();
    if sheet_names.is_empty() {
        return Err("No se encontraron hojas en el archivo Excel".into());
    }
    
    let primera_hoja = &sheet_names[0];
    println!("Leyendo hoja: {}", primera_hoja);
    
    let range = workbook.worksheet_range(primera_hoja)?;
    
    // Iterar sobre las filas (asumiendo que la primera fila son headers)
    for (row_idx, row) in range.rows().enumerate() {
        if row_idx == 0 { continue; } // Saltar header
        
        // Extraer datos de las columnas
        let codigo = match row.get(0) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "".to_string(),
            Some(Data::Error(_)) => "".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "".to_string(),
        };
        let nombre = match row.get(1) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "".to_string(),
            Some(Data::Error(_)) => "".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "".to_string(),
        };
        let correlativo = match row.get(2) {
            Some(Data::Float(f)) => *f as i32,
            Some(Data::Int(i)) => *i as i32,
            Some(Data::String(s)) => s.parse::<i32>().unwrap_or(0),
            Some(Data::Bool(b)) => if *b { 1 } else { 0 },
            Some(Data::Empty) => 0,
            Some(Data::Error(_)) => 0,
            Some(Data::DateTime(_)) => 0,
            Some(Data::DateTimeIso(_)) => 0,
            Some(Data::DurationIso(_)) => 0,
            None => 0,
        };
        let holgura = match row.get(3) {
            Some(Data::Float(f)) => *f as i32,
            Some(Data::Int(i)) => *i as i32,
            Some(Data::String(s)) => s.parse::<i32>().unwrap_or(0),
            Some(Data::Bool(b)) => if *b { 1 } else { 0 },
            Some(Data::Empty) => 0,
            Some(Data::Error(_)) => 0,
            Some(Data::DateTime(_)) => 0,
            Some(Data::DateTimeIso(_)) => 0,
            Some(Data::DurationIso(_)) => 0,
            None => 0,
        };
        let critico = match row.get(4) {
            Some(Data::String(s)) => s == "true" || s == "True" || s == "TRUE",
            Some(Data::Int(i)) => *i != 0,
            Some(Data::Float(f)) => *f != 0.0,
            Some(Data::Bool(b)) => *b,
            Some(Data::Empty) => false,
            Some(Data::Error(_)) => false,
            Some(Data::DateTime(_)) => false,
            Some(Data::DateTimeIso(_)) => false,
            Some(Data::DurationIso(_)) => false,
            None => false,
        };
        
        if !codigo.is_empty() {
            ramos_disponibles.insert(codigo.clone(), RamoDisponible {
                nombre,
                codigo: codigo.clone(),
                holgura,
                numb_correlativo: correlativo,
                critico,
                codigo_ref: Some(codigo),
            });
        }
    }
    
    Ok(ramos_disponibles)
}

// Función para leer Excel de oferta académica
pub fn leer_oferta_academica_excel(nombre_archivo: &str) -> Result<Vec<Seccion>, Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(nombre_archivo)?;
    let mut secciones = Vec::new();
    
    // Obtener la primera hoja disponible
    let sheet_names = workbook.sheet_names().to_owned();
    if sheet_names.is_empty() {
        return Err("No se encontraron hojas en el archivo Excel".into());
    }
    
    let primera_hoja = &sheet_names[0];
    println!("Leyendo hoja: {}", primera_hoja);
    
    // Intentar leer el rango con manejo de errores más específico
    let range = match workbook.worksheet_range(primera_hoja) {
        Ok(range) => range,
        Err(e) => {
            println!("Error específico al leer el rango: {:?}", e);
            return Err(format!("Error al acceder al rango de la hoja '{}': {}", primera_hoja, e).into());
        }
    };
    
    // Verificar que el rango tenga datos
    println!("Rango obtenido exitosamente");
    
    let size = range.get_size();
    println!("Tamaño del rango: {:?}", size);
    
    if size == (0, 0) {
        return Err("El archivo Excel está vacío".into());
    }
    
    let height = range.height();
    let width = range.width();
    
    println!("Dimensiones del Excel: {} filas, {} columnas", height, width);
    
    if height < 2 {
        return Err("El archivo debe tener al menos 2 filas (header + datos)".into());
    }
    
    if width < 6 {
        println!("⚠️  Advertencia: El archivo tiene solo {} columnas, se esperaban 6", width);
    }
    
    // Intentar iterar sobre las filas con manejo de errores
    let rows: Vec<_> = range.rows().collect();
    println!("Total de filas recolectadas: {}", rows.len());
    
    for (row_idx, row) in rows.iter().enumerate() {
        println!("Procesando fila {}: {:?}", row_idx, row);
        
        if row_idx == 0 { 
            println!("Headers encontrados: {:?}", row);
            continue; // Saltar header
        }
        
        // Verificar que la fila tenga contenido
        if row.is_empty() {
            println!("Fila {} está vacía, saltando", row_idx);
            continue;
        }
        
        let codigo = match row.get(0) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "".to_string(),
            Some(Data::Error(e)) => {
                println!("Error en celda [{}][0]: {:?}", row_idx, e);
                "".to_string()
            },
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "".to_string(),
        };
        
        if codigo.is_empty() {
            println!("Código vacío en fila {}, saltando", row_idx);
            continue; // Saltar filas vacías
        }
        
        let nombre = match row.get(1) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "Sin nombre".to_string(),
            Some(Data::Error(_)) => "Sin nombre".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "Sin nombre".to_string(),
        };
        
        let seccion = match row.get(2) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "1".to_string(),
            Some(Data::Error(_)) => "1".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "1".to_string(),
        };
        
        let horario_str = match row.get(3) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "".to_string(),
            Some(Data::Error(_)) => "".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "".to_string(),
        };
        
        let profesor = match row.get(4) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => "Sin asignar".to_string(),
            Some(Data::Error(_)) => "Sin asignar".to_string(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => "Sin asignar".to_string(),
        };
        
        let codigo_box = match row.get(5) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            Some(Data::Bool(b)) => b.to_string(),
            Some(Data::Empty) => {
                // Extraer código del ramo desde el código completo
                if codigo.contains('-') {
                    codigo.split('-').next().unwrap_or(&codigo).to_string()
                } else {
                    codigo.clone()
                }
            },
            Some(Data::Error(_)) => codigo.clone(),
            Some(Data::DateTime(d)) => d.to_string(),
            Some(Data::DateTimeIso(d)) => d.clone(),
            Some(Data::DurationIso(d)) => d.clone(),
            None => {
                // Extraer código del ramo desde el código completo
                if codigo.contains('-') {
                    codigo.split('-').next().unwrap_or(&codigo).to_string()
                } else {
                    codigo.clone()
                }
            },
        };
        
        // Parsear horarios (separados por comas o punto y coma)
        let horario: Vec<String> = if horario_str.is_empty() {
            vec!["Sin horario".to_string()]
        } else {
            horario_str
                .split(|c| c == ',' || c == ';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };
        
        secciones.push(Seccion {
            codigo: codigo.clone(),
            nombre: nombre.clone(),
            seccion: seccion.clone(),
            horario,
            profesor,
            codigo_box: codigo_box.clone(),
        });
        
        println!("✅ Sección procesada exitosamente: {} - {} (CódigoBox: {})", codigo, nombre, codigo_box);
    }
    
    println!("Total de secciones procesadas: {}", secciones.len());
    Ok(secciones)
}