# Renderizado

Este proyecto es un renderizador simple implementado en Rust que utiliza la biblioteca `nalgebra_glm` para realizar transformaciones en 3D (traslación, rotación y escalado). Utiliza también `minifb` para crear una ventana y mostrar los resultados. El renderizador puede cargar un modelo `.obj` y aplicar rotación al modelo, junto con una serie de transformaciones controladas por el usuario.

## Características

- Renderización de modelos 3D en Rust.
- Transformaciones básicas: traslación, rotación y escalado.
- Control de movimiento y escalado mediante teclado.
- Transformaciones adicionales en el eje Y para simular animaciones.
- Gestión de color y profundidad de fragmentos.

## Dependencias

Este proyecto requiere las siguientes dependencias de Rust:
- [`nalgebra_glm`](https://crates.io/crates/nalgebra-glm): para las operaciones de álgebra lineal.
- [`minifb`](https://crates.io/crates/minifb): para manejar la ventana.
- [`obj`](https://crates.io/crates/obj): para cargar modelos en formato `.obj`.

Instala estas dependencias ejecutando:

```bash
    cargo install nalgebra_glm minifb obj
```

## Cómo Usarlo

1. **Descarga o clona este repositorio**.
2. **Prepara tu modelo `.obj`** en la carpeta `assets/` (o en la ubicación que especifiques en tu código).
3. **Compila el proyecto** con Rust en modo release.
4. **Ejecuta el proyecto** para ver el modelo renderizado.

### Controles

- **Movimiento**: Usa las flechas para mover el modelo en el eje X e Y.
- **Escalado**: Tecla `S` para aumentar y tecla `A` para reducir el tamaño del modelo.
- **Rotación**:
  - Eje X: `Q` y `W`
  - Eje Y: `E` y `R`
  - Eje Z: `T` y `Y`
  
### Ejemplo de Resultado

Una vez que ejecutes el proyecto, deberías ver algo similar a la siguiente imagen:

![Resultado del Renderizado](https://github.com/Fabiola-cc/render_pipeline/blob/main/assets/nave_rendered.png)

## Estructura del Proyecto

- **framebuffer**: Módulo para manejar el buffer de fotogramas y renderizar píxeles.
- **triangle**: Función de rasterización de triángulos.
- **vertex**: Estructura y transformaciones de vértices.
- **shaders**: Funciones para aplicar sombreado a los vértices.
