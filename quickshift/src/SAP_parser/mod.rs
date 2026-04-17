use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::error::Error;

use serde::Deserialize;
use serde_json;

use crate::models::{Seccion, RamoDisponible};

#[derive(Debug, Deserialize)]
struct JsonSeccion {
    codigo: String,
    nombre: Option<String>,
    seccion: Option<String>,
    horario: Option<Vec<String>>,
    profesor: Option<String>,
    codigo_box: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JsonRamo {
    codigo: String,
    nombre: Option<String>,
    aprobados: Option<i32>,
    prioridad: Option<i32>,
    codigo_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JsonInput {
    secciones: Option<Vec<JsonSeccion>>,
    ramos_disponibles: Option<Vec<JsonRamo>>,
}

/// Lee un archivo JSON y devuelve (Vec<Seccion>, HashMap<código, RamoDisponible>).
/// Formato JSON esperado (ejemplo):
/// {
///   "secciones": [ { "codigo":"MAT101-01", "nombre":"Álgebra", "seccion":"01", "horario":["LU 08:30","MI 08:30"], "profesor":"Dr. X", "codigo_box":"MAT101" } ],
///   "ramos_disponibles": [ { "codigo":"MAT101", "nombre":"Álgebra", "aprobados":1, "prioridad":0 } ]
/// }
pub fn read_from_json<P: AsRef<Path>>(path: P) -> Result<(Vec<Seccion>, HashMap<String, RamoDisponible>), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let parsed: JsonInput = serde_json::from_str(&content)?;

    // Convertir secciones
    let mut secciones: Vec<Seccion> = Vec::new();
    if let Some(js) = parsed.secciones {
        for s in js {
            let horario = s.horario.unwrap_or_else(|| vec!["Sin horario".to_string()]);
            let sec = Seccion {
                codigo: s.codigo.clone(),
                nombre: s.nombre.unwrap_or_else(|| "Sin nombre".to_string()),
                seccion: s.seccion.unwrap_or_else(|| "1".to_string()),
                horario,
                profesor: s.profesor.unwrap_or_else(|| "Sin asignar".to_string()),
                codigo_box: s.codigo_box.unwrap_or_else(|| {
                    // derivar codigo_box desde codigo si no viene
                    s.codigo.split('-').next().unwrap_or(&s.codigo).to_string()
                }),
                // Si tu struct Seccion tiene más campos, inicializarlos aquí con defaults
                ..Default::default()
            };
            secciones.push(sec);
        }
    }

    // Convertir ramos_disponibles
    let mut ramos_map: HashMap<String, RamoDisponible> = HashMap::new();
    if let Some(ramos) = parsed.ramos_disponibles {
        for r in ramos {
            let rd = RamoDisponible {
                codigo: r.codigo.clone(),
                nombre: r.nombre.unwrap_or_default(),
                aprobados: r.aprobados.unwrap_or(0),
                prioridad: r.prioridad.unwrap_or(0),
                codigo_ref: r.codigo_ref,
                // Rellenar otros campos si el struct los tiene; usar defaults si necesario
                ..Default::default()
            };
            ramos_map.insert(r.codigo, rd);
        }
    }

    Ok((secciones, ramos_map))
}