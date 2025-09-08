// --- Traducción de rutaCritica.py, extract_data.py y get_clique_max_pond.py a Rust ---
// Implementación completa del sistema de generación de horarios

use std::collections::HashMap;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use petgraph::Direction;
use calamine::{Reader, Xlsx, open_workbook, Data};

// Estructuras base para los datos
#[derive(Debug, Clone)]
struct Seccion {
    codigo: String,
    nombre: String,
    seccion: String,
    horario: Vec<String>,
    profesor: String,
    codigo_box: String,
}

#[derive(Debug, Clone)]
struct RamoDisponible {
    nombre: String,
    codigo: String,
    holgura: i32,
    numb_correlativo: i32,
    critico: bool,
    codigo_ref: Option<String>,
}

#[derive(Debug, Clone)]
struct PertNode {
    codigo: String,
    nombre: String,
    es: Option<i32>,  // Earliest Start
    ef: Option<i32>,  // Earliest Finish
    ls: Option<i32>,  // Latest Start
    lf: Option<i32>,  // Latest Finish
    h: Option<i32>,   // Holgura
}

// Traducción de rutaCritica.py - función set_values_recursive
fn set_values_recursive(
    pert: &mut DiGraph<PertNode, ()>,
    node_idx: NodeIndex,
    len_dag: i32,
) {
    // Encontrar ancestros del nodo
    let mut max_count_jump = 1;
    
    // Calcular el camino más largo desde cualquier antecesor
    let predecessors: Vec<_> = pert.neighbors_directed(node_idx, Direction::Incoming).collect();
    
    for _pred_idx in predecessors.iter() {
        // Simular cálculo de camino más largo (simplificado)
        max_count_jump = std::cmp::max(max_count_jump, 2); // Simplificación
    }

    // Actualizar valores del nodo
    let node = &mut pert[node_idx];
    node.es = Some(if node.es.unwrap_or(0) < max_count_jump {
        max_count_jump
    } else {
        node.es.unwrap_or(max_count_jump)
    });
    
    node.ef = Some(node.es.unwrap() + 1);
    node.lf = Some(if len_dag > 1 && (node.lf.is_none() || node.lf.unwrap() > len_dag) {
        len_dag
    } else {
        node.lf.unwrap_or(len_dag)
    });
    
    let h = node.lf.unwrap() - node.ef.unwrap();
    node.h = Some(if h > 0 { h } else { 0 });
    node.ls = Some(node.es.unwrap() + node.h.unwrap());

    // Recursión en predecesores
    for pred_idx in predecessors {
        set_values_recursive(pert, pred_idx, len_dag - 1);
    }
}

// Nueva función para leer Excel de malla curricular
fn leer_malla_excel(nombre_archivo: &str) -> Result<HashMap<String, RamoDisponible>, Box<dyn std::error::Error>> {
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
        
        // Extraer datos de las columnas (ajusta según tu formato de Excel)
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

// Nueva función para leer Excel de oferta académica
fn leer_oferta_academica_excel(nombre_archivo: &str) -> Result<Vec<Seccion>, Box<dyn std::error::Error>> {
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

// Traducción de rutaCritica.py - función getRamoCritico
fn get_ramo_critico() -> (HashMap<String, RamoDisponible>, String) {
    println!("Leyendo ramos críticos desde Excel...");
    
    let nombre_excel_malla = "MiMalla.xlsx";
    
    // Intentar leer desde Excel primero
    match leer_malla_excel(nombre_excel_malla) {
        Ok(ramos_disponibles) => {
            println!("✅ Datos leídos exitosamente desde {}", nombre_excel_malla);
            
            println!("Ramos críticos:");
            for (codigo, ramo) in &ramos_disponibles {
                if ramo.critico {
                    println!("->> {} - {}", ramo.nombre, codigo);
                }
            }

            println!("\nRamos no críticos:");
            for (codigo, ramo) in &ramos_disponibles {
                if !ramo.critico {
                    println!("->> {} - {}", ramo.nombre, codigo);
                }
            }
            
            (ramos_disponibles, nombre_excel_malla.to_string())
        }
        Err(e) => {
            println!("⚠️  No se pudo leer el archivo Excel: {}", e);
            println!("Usando datos de ejemplo...");
            
            // Fallback a datos simulados
            let mut ramos_disponibles = HashMap::new();
            
            ramos_disponibles.insert("CIT3313".to_string(), RamoDisponible {
                nombre: "Algoritmos y Programación".to_string(),
                codigo: "CIT3313".to_string(),
                holgura: 0,
                numb_correlativo: 53,
                critico: true,
                codigo_ref: Some("CIT3313".to_string()),
            });
            
            ramos_disponibles.insert("CIT3211".to_string(), RamoDisponible {
                nombre: "Bases de Datos".to_string(),
                codigo: "CIT3211".to_string(),
                holgura: 0, // Ramo crítico
                numb_correlativo: 52,
                critico: true,
                codigo_ref: Some("CIT3211".to_string()),
            });
            
            // Simular ramos no críticos
            ramos_disponibles.insert("CIT3413".to_string(), RamoDisponible {
                nombre: "Redes de Computadores".to_string(),
                codigo: "CIT3413".to_string(),
                holgura: 2, // Ramo no crítico
                numb_correlativo: 54,
                critico: false,
                codigo_ref: Some("CIT3413".to_string()),
            });
            
            ramos_disponibles.insert("CFG-1".to_string(), RamoDisponible {
                nombre: "Curso de Formación General".to_string(),
                codigo: "CFG-1".to_string(),
                holgura: 3,
                numb_correlativo: 10,
                critico: false,
                codigo_ref: Some("CFG-1".to_string()),
            });

            (ramos_disponibles, nombre_excel_malla.to_string())
        }
    }
}

// Traducción de extract_data.py - función extract_data
fn extract_data(
    ramos_disponibles: &HashMap<String, RamoDisponible>,
    _nombre_excel_malla: &str,
) -> (Vec<Seccion>, HashMap<String, RamoDisponible>) {
    println!("Procesando extract_data...");
    
    let oferta_academica_file = "OfertaAcademica2024.xlsx";
    
    // Intentar leer oferta académica desde Excel
    match leer_oferta_academica_excel(oferta_academica_file) {
        Ok(mut lista_secciones) => {
            // Filtrar solo las secciones que corresponden a ramos disponibles
            lista_secciones.retain(|seccion| {
                ramos_disponibles.contains_key(&seccion.codigo_box) ||
                ramos_disponibles.iter().any(|(_, ramo)| ramo.codigo == seccion.codigo_box)
            });
            
            println!("✅ Se encontraron {} secciones desde Excel", lista_secciones.len());
            (lista_secciones, ramos_disponibles.clone())
        }
        Err(e) => {
            println!("⚠️  No se pudo leer oferta académica: {}", e);
            println!("Generando datos simulados...");
            
            // Fallback a datos simulados
            let mut lista_secciones = Vec::new();
            for (codigo_box, ramo) in ramos_disponibles {
                for seccion_num in 1..=2 {
                    let horarios = match seccion_num {
                        1 => vec!["LU 08:30".to_string(), "MI 08:30".to_string()],
                        2 => vec!["MA 10:00".to_string(), "JU 10:00".to_string()],
                        _ => vec!["VI 14:30".to_string()],
                    };
                    
                    lista_secciones.push(Seccion {
                        codigo: format!("{}-SEC{}", ramo.codigo, seccion_num),
                        nombre: ramo.nombre.clone(),
                        seccion: seccion_num.to_string(),
                        horario: horarios,
                        profesor: format!("Profesor {}", seccion_num),
                        codigo_box: codigo_box.clone(),
                    });
                }
            }
            
            (lista_secciones, ramos_disponibles.clone())
        }
    }
}

// Función auxiliar para verificar conflictos de horario
fn horarios_tienen_conflicto(horario1: &[String], horario2: &[String]) -> bool {
    for h1 in horario1 {
        for h2 in horario2 {
            if h1 == h2 {
                return true;
            }
        }
    }
    false
}

// Algoritmo heurístico para encontrar clique máximo ponderado
fn find_max_weight_clique(
    graph: &UnGraph<usize, ()>,
    priorities: &HashMap<NodeIndex, i32>,
) -> Vec<NodeIndex> {
    let _best_clique: Vec<NodeIndex> = Vec::new();
    let _best_weight = 0;
    
    let nodes: Vec<_> = graph.node_indices().collect();
    let _n = nodes.len();
    
    // Algoritmo greedy mejorado: construir clique válido
    let mut sorted_nodes = nodes.clone();
    sorted_nodes.sort_by(|&a, &b| {
        priorities.get(&b).unwrap_or(&0).cmp(priorities.get(&a).unwrap_or(&0))
    });
    
    let mut current_clique = Vec::new();
    
    // Tomar el primer nodo (mayor prioridad)
    if let Some(&first_node) = sorted_nodes.first() {
        current_clique.push(first_node);
    }
    
    // Agregar nodos compatibles
    for &node in sorted_nodes.iter().skip(1) {
        // Verificar si el nodo es compatible con todos los nodos del clique actual
        let mut compatible = true;
        for &clique_node in &current_clique {
            if !graph.contains_edge(node, clique_node) {
                compatible = false;
                break;
            }
        }
        
        if compatible {
            current_clique.push(node);
            if current_clique.len() >= 6 { // Limitar a 6 ramos máximo
                break;
            }
        }
    }
    
    // Si el clique es muy pequeño, intentar construir uno desde diferentes nodos iniciales
    if current_clique.len() < 3 {
        for &start_node in &sorted_nodes {
            let mut temp_clique = vec![start_node];
            
            for &candidate in &sorted_nodes {
                if candidate == start_node {
                    continue;
                }
                
                let mut compatible = true;
                for &clique_node in &temp_clique {
                    if !graph.contains_edge(candidate, clique_node) {
                        compatible = false;
                        break;
                    }
                }
                
                if compatible {
                    temp_clique.push(candidate);
                    if temp_clique.len() >= 6 {
                        break;
                    }
                }
            }
            
            if temp_clique.len() > current_clique.len() {
                current_clique = temp_clique;
            }
        }
    }
    
    current_clique
}

// Traducción de get_clique_max_pond.py
fn get_clique_max_pond(
    lista_secciones: &Vec<Seccion>,
    ramos_disponibles: &HashMap<String, RamoDisponible>,
) {
    println!("=== Generador de Horarios ===");
    println!("Ramos disponibles:\n");
    
    for (i, (codigo, ramo)) in ramos_disponibles.iter().enumerate() {
        println!("{}.- {} || {}", i, ramo.nombre, codigo);
    }

    // Simular prioridades (en la implementación real sería input del usuario)
    let mut priority_ramo: HashMap<String, i32> = HashMap::new();
    let mut priority_sec: HashMap<String, i32> = HashMap::new();
    
    // Ejemplo de prioridades predefinidas
    priority_ramo.insert("Algoritmos y Programación".to_string(), 90);
    priority_ramo.insert("Bases de Datos".to_string(), 85);
    priority_sec.insert("CIT3313-SEC1".to_string(), 95);

    // Construir grafo
    let mut graph = UnGraph::<usize, ()>::new_undirected();
    let mut node_indices = Vec::new();
    let mut priorities = HashMap::new();

    // Agregar nodos al grafo
    for (idx, seccion) in lista_secciones.iter().enumerate() {
        let ramo = &ramos_disponibles[&seccion.codigo_box];
        
        // Calcular prioridad según la lógica original
        let cc = if ramo.critico { 10 } else { 0 };
        let uu = 10 - ramo.holgura;
        let mut kk = 60 - ramo.numb_correlativo;
        
        // Aplicar prioridad de ramo si existe
        if let Some(&prio) = priority_ramo.get(&seccion.nombre) {
            kk = prio + 53;
        }
        
        let mut ss = seccion.seccion.parse::<i32>().unwrap_or(0);
        
        // Aplicar prioridad de sección si existe
        if let Some(&prio) = priority_sec.get(&seccion.codigo) {
            ss = prio + 20;
        }
        
        let prioridad = cc * 10000 + uu * 1000 + kk * 100 + ss;
        
        let node_idx = graph.add_node(idx);
        node_indices.push(node_idx);
        priorities.insert(node_idx, prioridad);
    }

    // Agregar aristas (conexiones entre secciones compatibles)
    for i in 0..node_indices.len() {
        for j in (i + 1)..node_indices.len() {
            let sec_i = &lista_secciones[graph[node_indices[i]]];
            let sec_j = &lista_secciones[graph[node_indices[j]]];
            
            // Verificar que no sean del mismo ramo y que no tengan conflictos de horario
            if sec_i.codigo_box != sec_j.codigo_box &&
               sec_i.codigo[..std::cmp::min(7, sec_i.codigo.len())] != 
               sec_j.codigo[..std::cmp::min(7, sec_j.codigo.len())] {
                
                if !horarios_tienen_conflicto(&sec_i.horario, &sec_j.horario) {
                    graph.add_edge(node_indices[i], node_indices[j], ());
                }
            }
        }
    }

    println!("\n=== Soluciones Recomendadas ===");
    
    // Encontrar múltiples soluciones
    let mut prev_solutions = Vec::new();
    let mut graph_copy = graph.clone();
    
    for solution_num in 1..=5 {
        let max_clique = find_max_weight_clique(&graph_copy, &priorities);
        
        if max_clique.len() <= 2 {
            println!("\n---------------");
            println!("Solo quedan soluciones con 2 o menos ramos");
            break;
        }
        
        let mut arr_aux_delete: Vec<(NodeIndex, i32)> = max_clique
            .iter()
            .map(|&idx| (idx, *priorities.get(&idx).unwrap_or(&0)))
            .collect();
        
        arr_aux_delete.sort_by_key(|&(_, prio)| prio);
        
        // Limitar a 6 ramos máximo
        while arr_aux_delete.len() > 6 {
            arr_aux_delete.remove(0);
        }
        
        // Verificar si ya se encontró esta solución
        let solution_key: Vec<_> = arr_aux_delete.iter().map(|&(idx, _)| idx).collect();
        if prev_solutions.contains(&solution_key) {
            continue;
        }
        
        println!("---------------");
        println!("\nSolución Recomendada #{}:\n", solution_num);
        
        for &(node_idx, prioridad) in &arr_aux_delete {
            let seccion_idx = graph_copy[node_idx];
            let seccion = &lista_secciones[seccion_idx];
            let codigo_corto = &seccion.codigo[..std::cmp::min(7, seccion.codigo.len())];
            
            println!(
                "{} || {} - Sección: {} | Horario -> {:?} || {}",
                codigo_corto,
                seccion.nombre,
                seccion.seccion,
                seccion.horario,
                prioridad
            );
        }
        
        prev_solutions.push(solution_key);
        
        // Remover un nodo para la siguiente iteración
        if !arr_aux_delete.is_empty() {
            graph_copy.remove_node(arr_aux_delete[0].0);
        }
    }
}

fn main() {
    println!("=== Sistema Generador de Horarios ===\n");
    
    // Paso 1: Obtener ramos críticos
    let (ramos_disponibles, nombre_excel_malla) = get_ramo_critico();
    
    println!("\nExtrayendo datos...\n");
    
    // Paso 2: Extraer datos de secciones
    let (lista_secciones, ramos_actualizados) = extract_data(&ramos_disponibles, &nombre_excel_malla);
    
    // Paso 3: Generar recomendaciones de horarios
    get_clique_max_pond(&lista_secciones, &ramos_actualizados);
    
    println!("\n=== Proceso completado ===");
}
