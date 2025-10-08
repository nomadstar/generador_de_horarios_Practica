# quickshift(1) — Manual breve del crate quickshift

NAME
----
quickshift — Generador de horarios: extracción desde Excel y algoritmo de clique ponderado

SYNOPSIS
--------
Desde la raíz del directorio `quickshift`:
- Revisar:    `cargo check`
- Compilar:   `cargo build`
- Ejecutar:   `cargo run`
- Release:    `cargo build --release`

DESCRIPTION
-----------
quickshift es un crate Rust que implementa un flujo para generar horarios académicos:
1. Extrae datos desde archivos Excel (mallas, oferta, mallas del alumno).
2. Construye estructuras internas (Seccion, RamoDisponible, etc.).
3. Ejecuta un algoritmo de clique ponderado para proponer conjuntos de secciones coherentes.

Estructura del proyecto (directorio por directorio)
---------------------------------------------------

Raíz (quickshift/)
- Cargo.toml / Cargo.lock: metadatos y dependencias (calamine, petgraph, chrono, ...).
- MiMalla.xlsx, OfertaAcademica2024.xlsx: ejemplos de datos para pruebas.
- target/: artefactos binarios y compilación generada por Cargo.

src/
- lib.rs
  - Exporta módulos públicos del crate; permite usar quickshift como librería.
- main.rs
  - Punto de entrada del binario.
  - Orquesta el flujo: obtener ramos críticos → extraer datos → ejecutar clique → mostrar resumen.

src/models/
- Contiene las definiciones de dominio:
  - Seccion: representa una sección concreta (código, sección, horario, profesor, prioridad, ...).
  - RamoDisponible: metadata por ramo (aprobados, prioridad, referencia, ...).
  - PertNode y otras estructuras auxiliares.
- Usar este módulo para añadir campos o anotaciones de serialización.

src/excel/
- Abstracción de lectura de Excel usando la crate `calamine`.
- Helpers para abrir workbooks, leer hojas y convertir filas a valores tipados.
- Lugar ideal para validar esquemas de hoja y normalizar columnas.

src/algorithms/
- Núcleo algorítmico:
  - Preparación de datos y utilidades.
  - `get_clique_max_pond`: algoritmo principal, ahora devuelve resultados como datos (Vec de soluciones).
  - Funciones de fallback y simulación (útiles para tests).
- Recomendación: envolver el `Vec` anidado actual en structs (`Solution`, `Entry`) para API más clara.

src/rutacritica/
- Port literal del proyecto RutaCritica (script Python):
  - extract.rs: traducción fiel del parser Python; usa calamine y propaga errores con `Result`.
  - clique.rs: adaptador entre extractor y `get_clique_max_pond`.
  - ruta.rs / mod.rs: orquestador `run_ruta_critica()` que devuelve `Result`.
- Mantener este módulo como referencia para futuras mejoras.

ERROR HANDLING
--------------
- La extracción ahora propaga errores (`Result<..., Box<dyn Error>>`).
- main deberia capturar y mostrar errores legibles al usuario.
- Recomendación: definir un enum de errores propio para mensajes más claros.

COMANDOS ÚTILES
--------------
- Verificación rápida: `cargo check`
- Compilar + generar binario: `cargo build`
- Ejecutar directamente: `cargo run`
- Ejecutar con release: `cargo run --release`
- Ejecutar tests (cuando estén añadidos): `cargo test`

ARCHIVOS DE ENTRADA
-------------------
- Ejemplos incluidos:
  - `MiMalla.xlsx`
  - `OfertaAcademica2024.xlsx`
- Los nombres y rutas pueden parametrizarse en el futuro (usar `clap`/`structopt`).

PRÓXIMOS PASOS RECOMENDADOS
---------------------------
1. Definir tipos de salida:
   - Crear `Solution` y `SectionEntry` para reemplazar `Vec<Vec<(Seccion, i32)>>`.
2. Mejorar los errores:
   - Replazar `Box<dyn Error>` por `enum Error` propio y mensajes claros.
3. Tests de integración:
   - Añadir tests que ejecuten `extract_data` sobre los Excel incluidos y validen outputs básicos.
4. CLI y configuración:
   - Añadir flags para pasar rutas XLSX, modo verbose y salida JSON.
5. Limpieza:
   - Eliminar `#[allow(dead_code)]` usando o removiendo código no usado.
6. Documentación:
   - Comentar API pública y documentar formato de hojas Excel esperadas.

EJEMPLO DE USO RÁPIDO
--------------------
1. Desde la carpeta `quickshift`:
   - `cargo run`
2. Salida esperada:
   - Resumen en consola con número de secciones procesadas y cantidad de soluciones devueltas por el algoritmo.

LICENSE
-------
Revisa `Cargo.toml` para dependencias y licencias. Mantener compatibilidad con licencias de crates usados.

CONTACTO
-------
Este documento resume la arquitectura y puntos de mejora. Para cambios automáticos o parches, ejecutar las acciones sugeridas en el módulo correspondiente.