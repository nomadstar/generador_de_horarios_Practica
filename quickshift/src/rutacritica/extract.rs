// extract.rs - adaptador para las funciones de lectura de Excel

/// Función stub que delega en `crate::excel` para leer mallas o ofertas.
pub fn leer_malla(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("rutacritica::extract -> leer_malla('{}')", path);
    // Por ahora delegamos al módulo excel si es necesario.
    // crate::excel::leer_malla_excel(path)?;
    Ok(())
}
