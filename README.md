# RayTracerCube Graficas

Raytracer interactivo en Rust que renderiza un cubo 3D sobre un piso usando rayos primarios, interseccion rayo-triangulo, materiales, iluminacion Phong, z-buffer y camara orbital.

## Caracteristicas

- Renderizado por ray tracing: se genera un rayo por pixel desde la camara.
- Cubo 3D centrado en la escena.
- Cubo construido con 6 caras divididas en 12 triangulos.
- Piso construido con 2 triangulos.
- Interseccion rayo-triangulo con el algoritmo Moller-Trumbore.
- Comparacion de profundidad para conservar el impacto mas cercano.
- Z-buffer por pixel inicializado con `f32::INFINITY`.
- Materiales con albedo y coeficientes `k_a`, `k_d`, `k_s` y `shininess`.
- Luz puntual con color, intensidad y atenuacion.
- Iluminacion Phong: componente ambiente, difusa y especular.
- Camara orbital alrededor del centro del cubo.
- Control interactivo con las flechas del teclado.

## Controles

- Flecha izquierda: disminuir `yaw`.
- Flecha derecha: aumentar `yaw`.
- Flecha arriba: aumentar `pitch`.
- Flecha abajo: disminuir `pitch`.
- Escape: cerrar la ventana.

La camara se mueve alrededor del cubo. El cubo permanece centrado y no rota como sustituto de la camara.

## Estructura

```text
src/
├── main.rs
├── camera.rs
├── color.rs
├── cube.rs
├── floor.rs
├── framebuffer.rs
├── intersect.rs
├── light.rs
├── material.rs
├── ray.rs
├── renderer.rs
└── triangle.rs
```

## Modulos principales

- `ray.rs`: define `Vec3`, operaciones vectoriales y `Ray`.
- `camera.rs`: implementa la camara orbital con `orbit_radius`, `yaw`, `pitch`, `eye`, `center` y `up_reference`.
- `triangle.rs`: resuelve intersecciones rayo-triangulo con Moller-Trumbore.
- `cube.rs`: crea el cubo usando 12 triangulos.
- `floor.rs`: crea el piso usando 2 triangulos.
- `material.rs`: define el material usado por el modelo de iluminacion.
- `light.rs`: define la luz puntual.
- `renderer.rs`: recorre los pixeles, genera colores, aplica z-buffer y calcula iluminacion.
- `framebuffer.rs`: guarda el buffer de pixeles que se envia a la ventana.

## Como ejecutar

```bash
cargo run
```

Se abrira una ventana con el cubo y el piso. Mantener presionadas las flechas permite orbitar la camara alrededor de la escena.

## Verificacion

```bash
cargo fmt --check
cargo check
cargo test
```

Las pruebas cubren:

- Conteo de triangulos del cubo.
- Conteo de triangulos del piso.
- Interseccion rayo-triangulo.
- Clamp del pitch de la camara orbital.
- Conversion esferica de la posicion orbital.
- Clamp del radio orbital.
- Seleccion del impacto mas cercano.
- Combinacion de albedo y color de luz en Phong.

## Notas de implementacion

El raytracer usa backward ray tracing: los rayos salen de la camara hacia la escena. Cada rayo se prueba contra todos los triangulos y se conserva la interseccion valida mas cercana.

La camara orbital calcula su posicion con coordenadas esfericas:

```text
x = orbit_radius * pitch.cos() * yaw.cos()
y = orbit_radius * pitch.sin()
z = orbit_radius * pitch.cos() * yaw.sin()
```

Luego apunta siempre al centro de la escena:

```text
forward = normalize(center - eye)
```

La iluminacion combina:

- Ambiente: `albedo * ambient_light * k_a`
- Difusa: `albedo * light_color * max(0, N dot L) * k_d`
- Especular: `light_color * max(0, R dot V)^shininess * k_s`

El z-buffer se inicializa con infinito para cada pixel y solo se actualiza cuando la nueva interseccion esta mas cerca de la camara.
