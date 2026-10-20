# Registro de cambios (Changelog)

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/).
Las versiones siguen [Versionado Semántico](https://semver.org/lang/es/).

## [1.0.0] — 2026-10-19 — Primera versión estable

Primera versión estable del simulador. Cierra el ciclo abierto en 0.7.0 con
trayectorias completas, la división de la partícula en las rendijas y las
mejoras de rendimiento que permiten sesiones largas sin perder impactos.

### Añadido

- **División de la partícula en las rendijas**: sin detector, el vuelo se
  dibuja dividiéndose en todas las rendijas (cabeza → barrera → ramas) y
  todas las ramas convergen en el mismo impacto; con el detector *which-path*
  se dibuja un único camino definido (ADR-0007).

### Mejorado (rendimiento)

- **Geometría 3D agrupada en lotes** (`batch.rs`, ADR-0008): la escena
  estática se cachea en mallas y solo se rehace si cambia λ, `d`, `a` o el
  nivel (antes ~300 subidas de geometría por frame); los contornos, frentes
  y perfil de onda viajan como cintas en un solo lote (antes ~1 550 líneas
  sueltas); las partículas usan dos lotes por frame (antes hasta ~480
  primitivas, con esferas de ~256 triángulos en Alta — ahora un disco
  orientado a la cámara de 8 triángulos, misma silueta).
- **Impactos con anillo incremental** (ADR-0008): cada impacto nuevo escribe
  su ranura en O(1); la malla ya no se reconstruye entera cada 25 impactos
  ni al deslizarse la ventana del almacén, y la pantalla se actualiza al
  instante. El cambio de λ repinta en su sitio, sin reasignar mallas.

### Corregido

- **Las partículas ya no se quedan a media trayectoria**: con el búfer lleno
  se omiten impactos, pero ningún vuelo en curso se recicla — antes se
  eliminaba el más antiguo en pleno vuelo y, según fps/calidad/ritmo, moría
  cerca de las rendijas («a veces ni llegaban») o antes de la pantalla.
- **Flujo sin congelarse**: el almacén de impactos es ahora una ventana
  deslizante (ADR-0007; sustituye a la opción A de ADR-0005): los impactos
  recientes siempre entran y la malla de la pantalla se sincroniza con cada
  impacto nuevo (anillo incremental, ADR-0008), también tras llenarse el
  almacén.
- **Los sliders físicos ya no reinician el experimento al arrastrarlos**
  (restituido ADR-0006) ni borran los vuelos en curso.

## [0.7.0] — 2026-10-17 — WASD, scrollbar y bugs corregidos

Corrección de los fallos reportados en uso real y las dos mejoras pedidas.

### Añadido

- **Movimiento de cámara con WASD**: `W`/`A`/`S`/`D` mueven la cámara por la
  escena y `Q`/`E` bajan/suben (Mayús: 3× más rápido). Consecuencia: `W`/`S`
  ya no ajustan el ancho `a`; ese parámetro se mueve desde el slider del
  panel. La línea de ayuda en pantalla lo refleja.
- **Barra de desplazamiento en el panel de control**: rueda del ratón sobre el
  panel, pulgar arrastrable y clic en el riel. Todos los controles —incluido
  «CÁMARA LIBRE»— están siempre alcanzables aunque la ventana sea baja.
- La longitud de onda se muestra con 3 decimales cuando es menor de 10 nm:
  el electrón muestra «λ 0.123 nm» en vez de «0 nm».

### Corregido

- **La cámara ya no puede bajar del suelo**: límite angular dinámico que
  garantiza una altura mínima de 0.30 u en cualquier órbita; también se
  redujo la distancia máxima (48 → 32 u) para no perder el banco óptico.
- **La vista «Pantalla» ya no enseña una pantalla negra**: la cámara queda
  dentro de la sala mirando la cara iluminada, a 5.2 u de distancia.
- **Partículas que desaparecían**: el destino vertical del vuelo estaba
  0.35 u por debajo del impacto registrado, así que muchas caían fuera de la
  pantalla. Además, cada partícula nace y muere con un difuminado suave y,
  con el techo de partículas activo, se recicla la más antigua.
- **«Mancha» al usar el electrón**: el color de la superficie ya no puede
  salir del rango válido (la λ de Broglie clampea a 380 nm para el color
  visible) y la escena es visible desde el primer instante: el camino de
  dibujo con calidad Alta pasa por el mismo destino de render que la Ligera,
  con lo que la escena se ve desde el arranque también sin GPU.
- Pared de fondo y suelo ligeramente aclarados para distinguirlos del fondo.

### Guardia automática

- Suite de 41 tests: suelo de la cámara, conversión de λ fuera de rango,
  vuelo de partículas con difuminado y geometría de la barra de desplazamiento.

## [0.6.0] — En desarrollo · Zoom suave y cámara libre

### Agregado
- Cámara libre con movimiento amortiguado: la pose actual persigue a la
  objetivo con amortiguación exponencial (independiente de los fps), así
  órbita, desplazamiento y zoom se sienten con inercia, sin saltos.
- Zoom suave y completo con la rueda: rango 0.8–48 unidades (del detalle
  de una rendija a la vista general del laboratorio), interpolado en
  escala logarítmica; deslizador de zoom en el panel (sección
  CÁMARA LIBRE) con lectura de distancia.
- Desplazamiento del centro de interés (pan) con botón derecho/central
  o Mayús+izquierdo; lente ajustable con Alt+rueda (20°–70°) aplicada
  también a la proyección de las anotaciones; doble clic para
  restablecer; los presets de vista ahora vuelan animados.
- Modo guiado opcional: al apagar «Cámara libre», pitch y desplazamiento
  se limitan para mantener el experimento siempre en cuadro.

## [0.5.0] — Ligero sin GPU y soporte Linux

### Agregado
- Calidad adaptativa (Auto/Alta/Ligera): mide fps y baja resolución de
  la escena (render a destino escalado), malla de onda, impactos
  dibujados, partículas (cubos baratos) y detalle ambiental. La física
  y los datos nunca se reducen; y la UI siempre va nítida.
- Linux de primera clase: `scripts/instalar-y-ejecutar.sh` (Debian,
  Fedora, Arch), CI con humo de arranque en Xvfb y binario de entrega.

## [0.4.0] — Interfaz moderna y escena enriquecida

### Agregado
- Sistema de diseño propio (`ui.rs`): tipografía Inter incrustada (OFL,
  con acentos, λ y µm), paneles de cristal redondeados con sombra,
  interruptores, badges, tarjetas y etiquetas de sección.
- Dock lateral de control a altura completa; la escena 3D se renderiza en
  un viewport que nunca queda bajo el panel.
- Escena con más materialidad: degradados frontales, marco luminoso y
  escaneo en pantalla, anillos de lente en la fuente, resplandor del haz
  en el suelo, polvo lejano, perfil brillante al llegar a la pantalla
  y partículas con estela.

## [0.3.0] — UI por ratón, más física y entrega

### Agregado
- Panel CONTROL con ratón: sliders λ/d/a/L/ritmo, rendijas 1–8, detector,
  pausa, disparo manual, presets, vistas de cámara, capas, órbita
  cinemática y modo presentación (`H`).
- Física: interferencia generalizada de N rendijas y presets fotón
  azul/verde/rojo y electrón de 100 eV (de Broglie, 0.123 nm).
- Entrega: CI activa en `.github/workflows/ci.yml`, GIF animado en el
  README y ejecutable de Windows cross-compilado (carpeta de entrega).

## [0.2.0] — Arquitectura por crates + motor 3D

### Agregado
- Workspace Cargo con tres crates bajo `src/`: `dualidad-core`
  (física pura y testeable), `dualidad-engine3d` (escena 3D en macroquad)
  y `dualidad-app` (binario `dualidad`).
- Motor 3D: cámara orbital (ratón + rueda), laboratorio con fuente, doble
  rendija y pantalla; superficie de `I(x)` en malla por tiras; impactos
  como mallas por lotes (hasta 40 000); histograma 3D con curva teórica;
  partículas en vuelo esquemáticas; HUD 2D.
- Tests automáticos (física, muestreo, mallas, cámara) y verificación de
  arranque headless sin errores.

### Mejorado (segundo pulido visual: de simple a escena rica)
- Anotaciones proyectadas con nombre de cada elemento (fuente, doble
  rendija + detector, superficie I(x), pantalla) con líneas guía.
- Haces translúcidos fuente → rendijas → pantalla (frustums propios),
  frentes de onda animados que barren la superficie e impactos con halo.
- Aparato detallado: banco óptico con doble riel y regla, placas base,
  módulo electrónico del detector y sombras suaves; panel LECTURA con
  lambda/d/a/L y fps en vivo. Captura: `docs/assets/20261017-escena-3d-v2.png`.

### Mejorado (pulido visual tras revisión con capturas reales)
- Cámara por defecto desde el lado de la fuente: la pantalla ya no tapa la escena.
- Superficie de onda flotante y translúcida con contornos; paleta suavizada
  (adiós verde lima saturado); barrera/pantalla menos masivas; HUD sin
  glifos que la fuente no tiene.
- El histograma salió de la escena 3D (se deformaba y estorbaba) a un
  panel de datos 2D nítido: barras vs curva teórica.

### Cambiado
- El monolito original `src/main.rs` (173 líneas) se eliminó y se
  reescribió dividido por responsabilidades. La física (Fraunhofer con
  detector which-path), los controles y los rangos se conservaron.

## Formalización del workspace (pre-0.1.0)

### Agregado
- Documentación completa en `docs/`: visión, física, arquitectura actual,
  instalación, roadmap, diseño 3D (plantilla), entrega escolar, riesgos,
  preguntas abiertas, glosario y referencias.
- Registro de decisiones en `docs/decisiones/` (ADR-0001 a ADR-0004).
- Texto oficial de licencia en `LICENSE` y guía `LICENCIAS.md`.
- Plantillas de informe escolar y de CI en `docs/plantillas/`.
- Plantillas de issues y pull request en `.github/`.
- Copia histórica del README original en `docs/historico/`.

### Sin cambios (a propósito)
- `src/main.rs` y `Cargo.lock`: **idénticos byte por byte** al prototipo clonado.
  Verificación con SHA-256 en `ESTADO-DEL-PROYECTO.md`.
- Del manifiesto solo cambió el nombre de `cargo.toml` a `Cargo.toml`
  (contenido intacto), con autorización expresa de los autores.
- La física, los controles y el comportamiento visual no se tocaron.

### Decidido
- **Licencia: `GPL-3.0-or-later`** (copyleft). Se eligió GPL sobre MIT/dual,
  se retiró el texto MIT y la licencia única vive en `LICENSE`.
  Ver ADR-0003 y `LICENCIAS.md` (2026-10-16).
- `cargo.toml` → `Cargo.toml`: desbloquea Linux y CI sin tocar código.

### Pendiente (decisión de los autores)
- Definición del alcance 3D. Ver `docs/06-diseno-3d.md`.
- Datos de la entrega escolar y si el docente exige alguna licencia propia.

## [0.1.0] — Prototipo inicial

### Agregado
- Simulador 2D de la doble rendija en Rust con macroquad.
- Intensidad de Fraunhofer `I(x) = sinc²(β)·cos²(δ)`, muestreo por rechazo.
- Pantalla de impactos, histograma vs. teoría, detector *which-path* (`O`).
- Controles de teclado para λ, `d`, `a`, ritmo, pausa y reinicio.
