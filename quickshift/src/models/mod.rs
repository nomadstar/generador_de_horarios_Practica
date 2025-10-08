// Estructuras de datos principales

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Seccion {
    pub codigo: String,
    pub nombre: String,
    pub seccion: String,
    pub horario: Vec<String>,
    pub profesor: String,
    pub codigo_box: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RamoDisponible {
    pub nombre: String,
    pub codigo: String,
    pub holgura: i32,
    pub numb_correlativo: i32,
    pub critico: bool,
    pub codigo_ref: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PertNode {
    pub codigo: String,
    pub nombre: String,
    pub es: Option<i32>,  // Earliest Start
    pub ef: Option<i32>,  // Earliest Finish
    pub ls: Option<i32>,  // Latest Start
    pub lf: Option<i32>,  // Latest Finish
    pub h: Option<i32>,   // Holgura
}