# Contrato técnico inicial — Bittopos Core 0.1

Estado: primera implementación propuesta; aún no compilada ni probada en este entorno.

## Alcance

- Lenguaje: Rust.
- Persistencia local: un archivo TOML por recurso.
- Recurso mínimo: `id`, `name` y `description` opcional.
- Identificador: UUID v7 generado por la aplicación.
- El ID permanece estable durante modificaciones.
- No incluye perfiles, proyectos, cuentas, instancias, relaciones, contención, CLI ni servidor.

## Archivo

El archivo se llama `<UUID>.toml`. La primera línea debe contener el campo `id`; el ID leído debe coincidir con el UUID del nombre del archivo. La descripción se omite si no tiene valor.

## Modificaciones

- Un campo omitido en `ResourcePatch` permanece sin cambios.
- `description: Some(None)` elimina la descripción.
- Una descripción vacía también se interpreta como ausencia.
- El nombre es obligatorio. Se eliminan los espacios ASCII finales; un nombre vacío o compuesto solo por espacios se rechaza.
- Los espacios iniciales se conservan y generan una advertencia.

## Recuperación

Se procesan los archivos `.toml`. Un archivo ilegible, con TOML inválido, nombre de archivo inválido, ID ausente de la primera línea, ID discordante o nombre inválido se omite y genera una advertencia. Los demás archivos siguen recuperándose. Los archivos temporales no se promueven automáticamente a definitivos.

## Escritura

La escritura usa un archivo temporal en el mismo directorio y sincroniza su contenido antes de persistirlo. La creación no reemplaza un archivo de destino existente. Esto reduce el riesgo de corrupción, pero no constituye una garantía absoluta frente a cualquier sistema de archivos, fallo eléctrico o plataforma.

## Concurrencia

La primera versión presupone un solo proceso escritor. No implementa bloqueos entre procesos.

## Verificación

Hay pruebas unitarias para creación, UUID v7, nombres duplicados, recuperación, modificación parcial, validación del nombre, ID inexistente, archivo corrupto, discordancia de ID y eliminación de descripción. Deben ejecutarse con `cargo test` en un entorno con Rust.
