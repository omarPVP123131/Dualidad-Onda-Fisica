# 04 · Instalación y desarrollo

## 1. Instalar Rust

Sigan la guía oficial: https://www.rust-lang.org/tools/install
(instala `rustup`, `rustc` y `Cargo`). Al terminar, verifiquen:

```bash
rustc --version
cargo --version
```

## 2. Dependencias del sistema operativo

macroquad necesita las librerías gráficas del sistema:

- **Windows:** normalmente nada extra (MSVC vía Visual Studio Build Tools si Rust lo pide).
- **macOS:** herramientas de línea de comandos: `xcode-select --install`.
- **Linux (Debian/Ubuntu):** desarrollo X11/OpenGL, ALSA y libxkbcommon:

```bash
sudo apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev libxkbcommon-dev
```

- **Linux (Fedora):** `sudo dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel libxkbcommon-devel`
- **Linux (Arch):** `sudo pacman -S libx11 libxi mesa alsa-lib libxkbcommon`

> **Atajo:** `scripts/instalar-y-ejecutar.sh` detecta tu distro, instala
> dependencias (y Rust si falta) y ejecuta el simulador.
> En Wayland la app corre por XWayland, sin configuración extra.
> Sin GPU no hay problema: el modo de calidad automático baja el detalle
> y la resolución de la escena (ver RENDIMIENTO en el panel de control).

> La lista exacta puede variar según la distro y la versión de macroquad.
> Si la compilación falla por una librería, el mensaje de error indica cuál falta.

## 3. Clonar y ejecutar

```bash
git clone https://github.com/omarPVP123131/Dualidad-Onda-Fisica.git
cd Dualidad-Onda-Fisica
cargo run --release
```

En desarrollo, `cargo run` basta; el `Cargo.toml` del prototipo ya optimiza
las dependencias en modo debug para que la simulación vaya fluida.

## 4. Nota histórica: el nombre del manifiesto (resuelto ✅)

En el repositorio original el manifiesto se llamaba **`cargo.toml`** en
minúsculas, y Cargo busca **`Cargo.toml`**: en Linux la compilación podía
fallar con «could not find `Cargo.toml`». En la formalización (2026-10-16),
con autorización de los autores, el archivo se renombró a `Cargo.toml`
— **solo el nombre; el contenido no cambió**. Si algún día parten de un clon
viejo, el arreglo equivalente es:

```bash
git mv cargo.toml Cargo.toml
git commit -m "fix: renombrar cargo.toml a Cargo.toml para Linux/CI"
```

## 5. Problemas frecuentes

| Síntoma | Causa probable | Qué revisar |
|---|---|---|
| `Cargo` no encuentra el proyecto (Linux) | Estás en un clon viejo con `cargo.toml` en minúsculas, o fuera de la carpeta | Punto 4 y `cd` a la raíz del proyecto |
| Error de enlazado con `-lX11`, `-lasound`, GL… | Faltan libs del SO (Linux) | Punto 2 |
| Ventana que se abre y se cierra / pantalla negra | Drivers OpenGL o entorno sin gráficos | Probar en otra máquina; actualizar drivers |
| Va lento en debug | Ejecutando sin optimizar dependencias | `cargo run --release` |
| No responde al teclado | La ventana no tiene el foco | Hacer clic en la ventana y reintentar |

## 6. Herramientas de calidad (cuando retomen código)

```bash
cargo fmt            # formato
cargo clippy         # avisos
cargo test           # tests (hoy: aún no hay; ver roadmap, Fase 1)
```

Este workspace no pudo verificar la compilación (el entorno de formalización
no tenía Rust instalado): la primera tarea técnica del roadmap es compilar
desde cero en las máquinas de ambos autores y anotar aquí cualquier paso extra.
