# Space Travel

Este proyecto es un simulador visual que recrea el movimiento orbital de planetas alrededor de un sol central. El sistema se construyó utilizando Rust y bibliotecas de gráficos personalizados, incorporando conceptos básicos de física y transformaciones geométricas. Además, implementa la renderización de órbitas, cálculos de movimiento orbital y consideraciones visuales como perspectiva y relaciones de aspecto.

## Tecnologías

- **Rust**: Lenguaje de programación utilizado para el desarrollo.
- **nalgebra_glm**: Librería de álgebra lineal para operaciones vectoriales y matriciales.
- **minifb**: Librería para la creación de ventanas y manejo de gráficos.
- **fastnoise_lite**: Librería de ruido procedural para generar texturas.

## Estructura de Archivos

- **`main.rs`**: Archivo principal que inicializa el sistema de renderizado y contiene el loop principal de la aplicación.s
- **`shaders.rs`**: Contiene las implementaciones de los shaders para cada planeta.
- **`vertex.rs`**: Define la estructura y transformación de vértices para renderizado.
- **`color.rs`**: Define la estructura de color y las operaciones de interpolación de color.
- **`fragment.rs`**: Define la estructura de fragmento, que almacena los datos de cada pixel en pantalla.

## Requisitos del proyecto 

### no subjetivos
- 10 puntos: creación de skybox que muestre estrellas en el horizonte
- 20 puntos: implementar movimiento 3D para la cámara
- 10 puntos: implementar una tecla que permita hacer un bird eye view de todo su sistema
- 20 puntos: renderizar las orbitas de los planetas
- 10 puntos: uso de un mapa normal en el último planeta

### subjetivos
- 30 Puntos por la estética del sistema completo
- 20 Puntos por el performance de la escena completa

## Controles de Teclado

### Bird View

Para visualizar el sistema completo

- **B**: Cambia la posición de zoom y centro, y muestra la órbita de los planetas

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
[Ver el video en YouTube](https://youtu.be/fbn-ZK-xIrE)