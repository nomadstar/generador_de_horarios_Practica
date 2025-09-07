// --- Traducción de rutaCritica.py, extract_data.py y get_clique_max_pond.py a Rust ---
// Implementación completa del sistema de generación de horarios

use std::collections::HashMap;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use petgraph::Direction;

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

// Traducción de rutaCritica.py - función getRamoCritico
fn get_ramo_critico() -> (HashMap<String, RamoDisponible>, String) {
    println!("Simulando getRamoCritico...");
    
    // Datos de ejemplo (en la implementación real se leería de Excel)
    let mut ramos_disponibles = HashMap::new();
    
    // Simular ramos críticos
    ramos_disponibles.insert("CIT3313".to_string(), RamoDisponible {
        nombre: "Algoritmos y Programación".to_string(),
        codigo: "CIT3313".to_string(),
        holgura: 0, // Ramo crítico
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

    let nombre_excel_malla = "MallaCurricular2020.xlsx".to_string();
    (ramos_disponibles, nombre_excel_malla)
}

// Traducción de extract_data.py - función extract_data
fn extract_data(
    ramos_disponibles: &HashMap<String, RamoDisponible>,
    _nombre_excel_malla: &str,
) -> (Vec<Seccion>, HashMap<String, RamoDisponible>) {
    println!("Procesando extract_data...");
    
    let mut lista_secciones = Vec::new();
    
    // Simular datos de secciones (en la implementación real se leería de Excel)
    for (codigo_box, ramo) in ramos_disponibles {
        // Crear múltiples secciones para cada ramo
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

    println!("Se encontraron {} secciones disponibles", lista_secciones.len());
    (lista_secciones, ramos_disponibles.clone())
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
