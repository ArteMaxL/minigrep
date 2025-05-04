# Minigrep

Minigrep (inspirada en Grep de Unix) es una aplicación de línea de comandos (CLI) escrita en Rust que permite buscar texto dentro de archivos. Es una herramienta simple pero poderosa para realizar búsquedas rápidas en archivos de texto.

## Características

- Busca líneas que contengan un texto específico dentro de un archivo.
- Muestra los resultados directamente en la terminal.
- Manejo de errores para archivos inexistentes o problemas de lectura.

## Instalación

1. Asegúrate de tener instalado [Rust](https://www.rust-lang.org/).
2. Clona este repositorio:

   ```sh
   git clone https://github.com/ArteMaxL/minigrep.git
   cd minigrep
   ```

3. Compila el proyecto:

   ```sh
   cargo build --release
   ```

4. El ejecutable estará disponible en `target/release/minigrep`.

## Uso

Ejecuta el programa desde la terminal con el siguiente formato:

```sh
minigrep <filename> <query>
```

- `<filename>`: El archivo donde deseas realizar la búsqueda.
- `<query>`: El texto que deseas buscar.

### Ejemplo

Supongamos que tienes un archivo llamado `poema.txt` con el siguiente contenido:

```
En un lugar de la Mancha,
de cuyo nombre no quiero acordarme,
no ha mucho tiempo que vivía un hidalgo...
```

Si ejecutas:

```sh
minigrep poema.txt Mancha
```

El programa mostrará:

```
En un lugar de la Mancha,
```

## Manejo de Errores

- Si el archivo no existe o no se puede leer, el programa mostrará un mensaje de error.
- Si no se encuentran resultados, se mostrará un mensaje indicando que no hay coincidencias.

## Contribuciones

¡Las contribuciones son bienvenidas! Si tienes ideas para mejorar esta herramienta, no dudes en abrir un issue o enviar un pull request.

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo `LICENSE` para más detalles.
