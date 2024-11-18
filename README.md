# Shaders

Este proyecto implementa un sistema de renderizado procedural en Rust utilizando shaders personalizados y ruido procedural para generar efectos visuales de diferentes planetas. Puedes explorar el entorno con controles de cámara y cambiar entre varios planetas para observar distintas texturas y efectos en tiempo real.

## Tecnologías

- **Rust**: Lenguaje de programación utilizado para el desarrollo.
- **nalgebra_glm**: Librería de álgebra lineal para operaciones vectoriales y matriciales.
- **minifb**: Librería para la creación de ventanas y manejo de gráficos.
- **fastnoise_lite**: Librería de ruido procedural para generar texturas.

## Estructura de Archivos

- **`main.rs`**: Archivo principal que inicializa el sistema de renderizado y contiene el loop principal de la aplicación.
- **`renderer.rs`**: Define la estructura y lógica del `Renderer`, que administra el shader y el ruido actual.
- **`shaders.rs`**: Contiene las implementaciones de los shaders para cada planeta.
- **`vertex.rs`**: Define la estructura y transformación de vértices para renderizado.
- **`color.rs`**: Define la estructura de color y las operaciones de interpolación de color.
- **`fragment.rs`**: Define la estructura de fragmento, que almacena los datos de cada pixel en pantalla.

## Controles de Teclado

### Selección de Planetas

Para cambiar entre planetas, usa las teclas del `1` al `7`:

- **1-7**: Cambia al planeta correspondiente y aplica un shader y tipo de ruido específicos.

### Controles de Cámara

- **Órbita de Cámara**:
  - Flecha **Izquierda**: Rotar la cámara hacia la izquierda.
  - Flecha **Derecha**: Rotar la cámara hacia la derecha.
  - **W**: Rotar hacia arriba.
  - **S**: Rotar hacia abajo.

- **Movimiento de Cámara**:
  - **A**: Mover cámara hacia la izquierda.
  - **D**: Mover cámara hacia la derecha.
  - **Q**: Mover cámara hacia arriba.
  - **E**: Mover cámara hacia abajo.

- **Zoom de Cámara**:
  - Flecha **Arriba**: Acercar.
  - Flecha **Abajo**: Alejar.

Cada tecla modifica la posición o el ángulo de la cámara, permitiéndote explorar libremente el entorno y observar los efectos visuales de los diferentes shaders.

## Ejecución del Proyecto

Para compilar y ejecutar el proyecto, asegúrate de tener Rust instalado y ejecuta el siguiente comando en la raíz del proyecto:
``cargo run``

El programa abrirá una ventana donde podrás ver los efectos visuales de cada planeta y experimentar con las distintas opciones de renderizado.

### Resultado
[Ver el video en YouTube](https://youtu.be/XrfG1RIbx9o)