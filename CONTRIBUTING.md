# Contribuir

Gracias por ayudar a mejorar el código de *Rust a fondo*.

## Erratas y fallos

Al abrir una incidencia, incluye:

- el identificador del bloque o solución, por ejemplo `ch35-b004` o `C35-E04`;
- la salida de `rustc --version`;
- el sistema operativo;
- el comportamiento observado y el esperado;
- un ejemplo mínimo cuando sea posible.

## Cambios de código

1. Crea una rama desde `main`.
2. Conserva el identificador `SOLUTION` correspondiente.
3. Añade o actualiza los tests que demuestren el cambio.
4. Ejecuta `verify.ps1` en Windows o `verify.sh` en Linux/macOS.
5. Explica en el pull request qué contrato mejora o corrige el cambio.

Los archivos `listings/`, `doctests/book.md` y `manifest.json` se sincronizan
con la edición del libro. No se aceptarán cambios aislados en esos archivos sin
la correspondiente errata editorial.
