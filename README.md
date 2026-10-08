# Bittopos Core 0.1

Núcleo inicial para crear, guardar, recuperar y modificar recursos locales.

## Requisitos

- Rust estable y Cargo.

## Ejecutar pruebas

```sh
cargo test
```
## Estado de esta revisión

El protocolo de persistencia utiliza temporales ".<UUID>.toml.tmp", evita sobrescribir temporales abandonados, advierte de su presencia durante la recuperación y evita escrituras cuando el contenido serializado no cambia. También se prueba que un fallo de promoción conserva el temporal validado.

**Verificación local:** `cargo test --locked` terminó con 13 pruebas aprobadas y 0 fallidas en Termux. `cargo fmt --check` y `git diff --check` terminaron sin errores. La revisión final del diff de implementación se ha realizado.
