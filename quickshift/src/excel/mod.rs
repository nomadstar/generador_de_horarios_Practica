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
            _ => "".to_string(),
        };
        let nombre = match row.get(1) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let correlativo = match row.get(2) {
            Some(Data::Float(f)) => *f as i32,
            Some(Data::Int(i)) => *i as i32,
            Some(Data::String(s)) => s.parse::<i32>().unwrap_or(0),
            _ => 0,
        };
        let holgura = match row.get(3) {
            Some(Data::Float(f)) => *f as i32,
            Some(Data::Int(i)) => *i as i32,
            Some(Data::String(s)) => s.parse::<i32>().unwrap_or(0),
            _ => 0,
        };
        let critico = match row.get(4) {
            Some(Data::String(s)) => s == "true" || s == "True" || s == "TRUE",
            Some(Data::Int(i)) => *i != 0,
            Some(Data::Float(f)) => *f != 0.0,
            _ => false,
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
    
    let range = workbook.worksheet_range(primera_hoja)?;
    
    for (row_idx, row) in range.rows().enumerate() {
        if row_idx == 0 { continue; } // Saltar header
        
        let codigo = match row.get(0) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let nombre = match row.get(1) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let seccion = match row.get(2) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let horario_str = match row.get(3) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let profesor = match row.get(4) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => "".to_string(),
        };
        let codigo_box = match row.get(5) {
            Some(Data::String(s)) => s.clone(),
            Some(Data::Float(f)) => f.to_string(),
            Some(Data::Int(i)) => i.to_string(),
            _ => codigo.clone(),
        };
        
        // Parsear horarios (separados por comas o punto y coma)
        let horario: Vec<String> = horario_str
            .split(|c| c == ',' || c == ';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if !codigo.is_empty() {
            secciones.push(Seccion {
                codigo,
                nombre,
                seccion,
                horario,
                profesor,
                codigo_box,
            });
        }
    }
    
    Ok(secciones)
}