---
sidebar_position: 9
---

# Darbs ar vairākiem Priedes failiem

Lai padarītu kodu vieglāk lasāmu, to var sadalīt vairākos failos un tad vienā koda failā iekļaut mainīgos vai funkcijas no citiem failiem.

```priede title="galvenais_fails.pr"
iekļaut {a} no "E:/Dev/priede/examples/other_file.pr"

izvade(a)
```

Pēdiņās ir jānorāda faila nosaukums, no kura tiek importēti mainīgie vai funkcijas.

Bet figūriekavās jānorāda mainīgo vai funkciju nosaukumi, kas tiek importēti.

Savukārt lai eksportētu mainīgo vai funkciju, jāizmanto `eksp`.

```priede title="otrs_fails.pr"
eksp sk a : 3
```

Ar funkcijām ir tāpat.

```priede title="otrs_fails.pr"
eksp funkc b() {
    izvade("a")
}
```
