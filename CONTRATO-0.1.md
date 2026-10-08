# Contrato técnico inicial — Bittopos Core 0.1

Estado: contrato del prototipo 0.1. La verificación debe reflejar los resultados reales de las pruebas ejecutadas.

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

Se procesan los archivos con extensión `.toml` cuyo nombre base sea un UUID válido. Un archivo ilegible, con TOML inválido, nombre de archivo inválido, ID ausente de la primera línea, ID discordante o nombre inválido se omite y genera una advertencia. Los demás archivos siguen recuperándose.

Los temporales de guardado siguen el patrón `.<UUID>.toml.tmp`. La recuperación los identifica y advierte de su presencia, pero nunca los promueve automáticamente a definitivos ni altera el recurso confirmado. Otros archivos que no sean recursos confirmados se ignoran.

## Escritura

La escritura prepara un temporal `.<UUID>.toml.tmp` en el mismo directorio del recurso. El temporal se crea sin sobrescribir uno preexistente; si una operación necesita escribir y ese temporal ya existe, falla de forma controlada y conserva tanto ese temporal como el recurso confirmado. Una modificación cuyo contenido serializado ya coincide con el confirmado no necesita temporal y puede terminar sin escritura.

El contenido se escribe completo, se sincroniza y se valida antes de intentar sustituir el archivo definitivo. La creación no debe reemplazar un destino existente. En una modificación, el archivo confirmado permanece intacto hasta el intento de sustitución. Si la representación TOML generada coincide byte por byte con el archivo confirmado, la operación no escribe ni reemplaza el archivo.

Este protocolo reduce el riesgo de corrupción, pero no constituye una garantía absoluta frente a cualquier sistema de archivos, fallo eléctrico o plataforma. Las garantías de sustitución y durabilidad dependen de la operación utilizada y del sistema operativo.

Los temporales son artefactos transitorios de guardado, no sesiones de edición recuperables. La persistencia de borradores y la opción interactiva de reanudar o descartar una edición pendiente quedan fuera del alcance de 0.1. Reanudar un borrador futuro no deberá confirmar sus cambios automáticamente; descartarlo no deberá modificar el último estado confirmado.

## Concurrencia

La primera versión presupone un solo proceso escritor. No implementa bloqueos entre procesos.

## Verificación

La suite de pruebas actual cubre creación y modificación, UUID v7, nombres duplicados, recuperación, modificación parcial, validación del nombre, ID inexistente, archivo corrupto, discordancia de ID, eliminación de descripción, colisión con un temporal abandonado, conservación del temporal preparado cuando falla la promoción, aviso de recuperación de temporales y ausencia de escritura cuando el contenido serializado no cambia. Ejecutar con `cargo test`.
