---
sidebar_position: 1.4
---

# Priedes komandrindiņa

Priedes programmas uz datora var izpildīt ar komandrindiņu.

Lai palaistu programmu

```bash
priede fails.pr
```

`fails.pr` vietā ievieto savu koda failu

## Papildus iespējas atkļūdošanai

- `--ast` izdrukā abstraktās sintakses koku.
- `--bytecode` izdrukā programmas baitkodu.
- `--format` neizpilda programmu, bet formatē kodu un to izdrukā. Iespējams papildus pievienot `--ast`.
- `--static-only` nepalaiž programmu, bet izdrukā kompilācijas laika kļūdas, ja tādas ir.
- `--testing-stack` programmas beigās izdrukā testēšanas steku.

