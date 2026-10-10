# Bittopos Core

Núcleo de Bittopos, desarrollado en Rust. El proyecto busca proporcionar una base local para crear, almacenar, consultar y modificar recursos.

## Estado del proyecto

* **Versión actual de referencia:** 0.1.0
* **Versión en desarrollo:** 0.2.0

La versión 0.1.0 establece la base inicial de persistencia y gestión de recursos. La versión 0.2.0 está en planificación e implementación, con el objetivo de ampliar el modelo para incluir información estructurada de los recursos y espacios de trabajo que registren referencias explícitas a ellos.

La versión 0.2.0 todavía está en desarrollo. Su alcance y sus criterios de aceptación se definen en el contrato técnico correspondiente.

## Contratos técnicos

- [CONTRATO-0.1.md](CONTRATO-0.1.md): especificación de la versión inicial.
- [CONTRATO-0.2.md](CONTRATO-0.2.md): especificación de la versión en desarrollo.

Los contratos documentan el alcance y el comportamiento esperado de cada versión. No implican que todas las funciones descritas ya estén implementadas.

## Tecnologías

- Rust y Cargo.
- Persistencia local mediante archivos TOML para recursos.
- JSON para el registro de referencias del espacio de trabajo previsto en la versión 0.2.0.

## Desarrollo y verificación

Para ejecutar las pruebas del proyecto:

```bash
cargo test
```

Para comprobar el formato del código:

```bash
cargo fmt --check
```

Los resultados de estas comprobaciones deben corresponder al estado real del código y ejecutarse nuevamente después de los cambios relevantes.

## Alcance de la versión 0.2.0

El desarrollo se centrará en el núcleo local y en una interfaz de línea de comandos pequeña que utilice dicho núcleo. Las relaciones avanzadas, los permisos, la sincronización remota, los servidores y el despliegue en la nube quedan fuera del alcance actual.
