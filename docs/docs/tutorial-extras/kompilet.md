---
sidebar_position: 1
---

# Kompilēt Priedi

## Bez leksera un parsera

Ja nevēlies mainīt Priedes lekseri vai parseri, kas ir iekļauti repozitorijā kā binārie faili, tad kompilēt priedi ir tik vienkārši, kā palaist `cargo build` komandu `cli` direktorijā. Cargo ir rust valodas pakotņu menedžeris, kuru var lejupielādēt [šeit](https://www.rust-lang.org/tools/install).

## Ar lekseri un parseri

Priede izmanto [hime](https://cenotelie.fr/projects/hime) parsera ģeneratoru. Lejupielādējot to, iekopē failus "himmecc.exe" un "Hime.SDK.dll" priedes direktorijā `interpreter/src/hime`. Lai kompilētu lekseri un parseri palaid python skripu `build-gram.py`.
