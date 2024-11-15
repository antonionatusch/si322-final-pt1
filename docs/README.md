
<br />
<div align ="center">
  <a href="https://virtual.upsa.edu.bo/">
    <img src="https://virtual.upsa.edu.bo/pluginfile.php/1/theme_lambda/logo/1708129513/logo%20UPSA-universidad-03.png">
  </a>
</div>

# si322-final-pt1 y sus reglas

Repositorio que contiene el proyecto final para la materia **Laboratorio de Sistemas Operativos II - SI322** adaptado a Rust.

## gitignore
El archivo `.gitignore` **bajo _ninguna_ circunstancia** debe modificarse sin previa autorización de los administradores del repositorio, ya que previene problemas de compilación y permite una mayor flexibilidad de colaboración.

## Ramas
Podrás solicitar una nueva rama para contribuir al desarrollo del repositorio siempre y cuando todos los demás colaboradores estén de acuerdo.

A la rama `develop` solo irán aquellos aportes **100% funcionales** que **no interfieran** con el desarrollo individual de las ramas de los colaboradores, pero que están pendientes de expansión o reducción, junto con los directorios de cada colaborador.

A la rama `main` irán todos los aportes **100% funcionales** definitivos, que no necesitarán de modificación posterior, junto con los módulos provenientes de los colaboradores.

La rama de cada desarrollador es libre, sin embargo, **no se aceptarán _pull requests_ que modifiquen archivos importantes como `.gitignore` o `README.md`**.

## Lenguaje
El único lenguaje a utilizarse en este repositorio será **Rust**, ya que es el lenguaje en el cual se optó por desarrollar este proyecto.

## Buenas prácticas
Todas las funciones deben seguir una documentación que tiene la siguiente forma:

```rust
/// Suma dos números enteros
/// 
/// # Parámetros
/// - `a`: primer entero a sumar.
/// - `b`: segundo entero a sumar.
/// 
/// # Retorno
/// Retorna la suma de los dos números enteros `a + b`.
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

Además, se hará uso del idioma inglés para todas las nomenclaturas de variables, funciones, estructuras, etc.Solo se hará uso del español para las salidas a consola, por ejemplo, en los `println!`.

- **PascalCase** para estructuras y métodos.
- **snake_case** para las variables y funciones.
- **SCREAMING_SNAKE_CASE** para constantes globales.

Al ser un proyecto multirepo, para poder ejecutar cada ejercicio se hará uso de <code>cargo run</code> para ejecutar cada ejercicio según corresponda.

### Commits Convencionales

Para mantener la consistencia y claridad en el historial de commits, se utilizarán commits convencionales siguiendo el estándar de **Commitizen**, basado en los commits de Angular. Para especificar el alcance (`scope`), como el ejercicio o módulo afectado, se incluye entre paréntesis después del tipo de commit. 

Ejemplos de commits convencionales con `scope`:

- `feat(ejercicio1): add agent struct for exercise 1`
- `fix(ejercicio2): correct semaphore handling`
- `docs(ejercicio3): update README with usage examples for module`
- `style(ejercicio1): format agent module to match project style guidelines`
- `refactor(bicycle_semaphore): optimize acquire method`
- `test(mounter): add tests for assemble_bicycle method`
- `chore(global): update dependencies`

## Herramientas
Se recomienda el uso de [RustRover](https://www.jetbrains.com/rust/) como IDE principal para el desarrollo del proyecto, con la opción de utilizar Visual Studio Code. Este proyecto se desarrolla en Rust, que incluye un manejo eficiente de concurrencia y semáforos en el paquete `std`.

## Estructura del Proyecto

Este proyecto utiliza una estructura de directorio para un repositorio multi-ejercicio. Cada subdirectorio (`ejercicio1`, `ejercicio2`, etc.) contiene su propio código fuente y configuración.

```plaintext
si322-final-pt1/
├── ejercicio3/             # Subdirectorio para el ejercicio 3
│   ├── Cargo.toml          # Configuración específica del ejercicio 3
│   └── src/                # Código fuente para el ejercicio 3
│       ├── agent.rs        # Módulo para la estructura Agent
│       ├── bicycle_semaphore.rs # Módulo para BicycleSemaphore
│       ├── global.rs       # Variables y funciones globales
│       ├── menu.rs         # Menú interactivo
│       └── main.rs         # Punto de entrada principal
├── ejercicioN/             # Subdirectorio para el ejercicio N
│   ├── Cargo.toml          # Configuración específica del ejercicio N
│   └── src/                # Código fuente para el ejercicio N
└── docs/                   # Documentación y ejercicios del proyecto
└── README.md               # Documentación general
```

## Ejemplos de Implementación en Rust

### Módulo `agent`

Definición de la estructura `Agent` y sus métodos, adaptados del código en C++:

```rust
// src/agent.rs

pub struct Agent;

impl Agent {
    /// Coloca dos ingredientes en la mesa para el proceso de fumadores
    pub fn put_two_ingredients(&self) {
        println!("Colocando dos ingredientes en la mesa.");
    }

    /// Verifica si el proceso debe continuar
    pub fn ask_to_continue() -> bool {
        // Implementación lógica para continuar o detener
        true
    }
}
```

### Módulo `bicycle_semaphore`

Definición de `BicycleSemaphore` usando la estructura `Semaphore` en Rust:

```rust
// src/bicycle_semaphore.rs
use std::sync::{Arc, Mutex, Condvar};

pub struct BicycleSemaphore {
    count: Mutex<u32>,
    condvar: Condvar,
}

impl BicycleSemaphore {
    pub fn new(count: u32) -> Self {
        Self {
            count: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.condvar.notify_one();
    }
}
```

### Módulo `mounter`

Definición del montador (`Mounter`) que sincroniza con los semáforos:

```rust
// src/mounter.rs
use crate::bicycle_semaphore::BicycleSemaphore;

pub struct Mounter {
    semaphore: BicycleSemaphore,
}

impl Mounter {
    pub fn new(semaphore: BicycleSemaphore) -> Self {
        Self { semaphore }
    }

    pub fn assemble_bicycle(&self) {
        self.semaphore.acquire();
        println!("Armando la bicicleta con las piezas necesarias.");
        self.semaphore.release();
    }
}
```

### Integración de Módulos

Para usar estos módulos en el archivo `main.rs`, se debe declarar cada módulo y usarlos:

```rust
// src/main.rs
mod agent;
mod bicycle_semaphore;
mod mounter;

use agent::Agent;
use bicycle_semaphore::BicycleSemaphore;
use mounter::Mounter;

fn main() {
    let agent = Agent;
    let semaphore = BicycleSemaphore::new(1);
    let mounter = Mounter::new(semaphore);

    agent.put_two_ingredients();
    if Agent::ask_to_continue() {
        mounter.assemble_bicycle();
    }
}
```

## Construcción de documentación

La documentación se genera automáticamente a partir de los comentarios en el código, siguiendo el formato estándar de documentación de Rust. Para generar la documentación, utiliza el siguiente comando en la raíz del proyecto:

```bash
cargo doc --open
```

Este comando compilará la documentación basada en los comentarios de cada módulo y abrirá un navegador para visualizarla. Asegúrate de que todos los comentarios de tus funciones, estructuras y módulos sigan el formato adecuado, ya que esta documentación será la referencia oficial del proyecto.

## Problemas a realizar
Se encuentran en el directorio `docs` del directorio raíz.
Se asignaron por sorteo de la siguiente forma: </br>
Mauricio: 3 y 2 </br>
Antonio: 1 y 6 </br>
Werner: 5 y 7 </br>
Dylan: 4

## Contacto
Cualquier duda, sugerencia o recomendación deberá hacerse contactando de manera presencial o por correo a los co-dueños:

- Dylan Uribe - a2022112008@estudiantes.upsa.edu.bo
- Antonio Natusch - a2022111958@estudiantes.upsa.edu.bo
- Werner Holters - a2022114973@estudiantes.upsa.edu.bo
- Mauricio Flores - a20221112750@estudiantes.upsa.edu.bo
