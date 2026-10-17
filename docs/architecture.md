# Arquitectura actual

## Recorrido principal

```text
CLI bitto
   ↓
operaciones del núcleo
   ├── recursos y relaciones TOML
   ├── estructura JSON con instancias
   └── resolución semántica
```

La estructura es el único contenedor de organización de las instancias. Los recursos y relaciones son archivos independientes; las relaciones se leen directamente sin instanciación. El segmento de control `__relation` contiene únicamente los componentes `from` y `to`. El espacio de identidad de ambos extremos se expresa mediante el tipo de componente (`resource-id` o `instance-id`), sin un componente adicional `kind`. La resolución combina la definición compartida con las diferencias locales de la instancia.

## Módulos

- `resource.rs`: modelo de recurso y metadatos.
- `relations.rs`: creación, lectura y validación de extremos de relaciones.
- `store.rs`: lectura y escritura de recursos TOML.
- `workspace_model.rs`: instancias, contención y reglas del modelo de estructura.
- `workspace.rs`: serialización JSON de la estructura.
- `semantic_masks.rs`: segmentos, conceptos, componentes, máscaras y resolución.
- `mask_operations.rs`: operaciones de edición compartida/local y resolución de instancias.
- `lifecycle.rs`: carga, creación y eliminación de instancias.
- `navigation.rs`: ubicación actual de navegación.
- `application.rs`: coordinación mínima de operaciones para una interfaz.
- `cli/src/main.rs`: interfaz de terminal.

Los módulos se separan por responsabilidad. Una separación adicional solo se justifica cuando un archivo contiene responsabilidades independientes que se puedan mover sin crear dependencias circulares.

## Persistencia y límites

Las operaciones escriben directamente los archivos implicados. La escritura del recurso y de la estructura no forma una transacción conjunta; un fallo entre ambas escrituras puede dejar un estado parcial. No hay journal ni reconciliación automática.

La prioridad es que el camino normal de datos sea visible. No se incorporan mecanismos de recuperación, temporales administrados, transacciones entre archivos o adaptadores futuros sin una decisión expresa.

## Fuera del núcleo actual

La interfaz web es experimental y guarda su propio estado en el navegador. No está conectada al núcleo Rust. Servidor, sincronización, autenticación, bases de datos externas y despliegue WASM no forman parte del flujo local actual.
