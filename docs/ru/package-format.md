# Пакеты ARX

Пакеты ARX — это архивы .tar.zst с определённой структурой.

## Структура
- `arx-meta.toml` — обязательный файл с метаинформацией
- `usr/` — стандартная иерархия для файлов
- `arx/` — служебные файлы ARX
- `build/` — исходники (опционально, для пакетов, которые собираются из исходников)

## Метаинформация (arx-meta.toml)
```toml
[package]
name = "firefox"
version = "115.3.0"

[dependencies]
libvpx = { version = ">=11", policy = "rolling-patch" }
libxcb = { version = "1.13", policy = "lts" }
```