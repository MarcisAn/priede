---
sidebar_position: 7
---

# "Stumbrs" datu formāts

Priede piedāvā programmas ievaddatus definēt atsevišķā failā, kura nolasīšana ir integrēta Priedē.

## Stumbra datu pieraksts

Stumbra fails sastāv no divām daļām: datu shēmas definīcijas un pašiem datiem.
Shēma tiek definēta tāpat kā Priedē, bet bez datu vērtībām

```priede
skaitlis a
```

Stumbra shēmas bloks sastāv no vienas vai vairākām definīcijām un ir iekļauts figūriekavās.

```priede
{
    teksts vārds
    skaitlis vecums
}
```

Tālāk seko datu bloks, kas sastāv no vērtībām, bez nosaukumiem, tādā pašā secībā, kā tās ir definētas shēmā

```priede
{
    teksts vārds
    skaitlis vecums
}
{
    "Miķelis"
    64
}
```

## Stumbra datu ielasīšana Priedē

Priedē iespējams, izsaucot vienu funkciju, iestatīt vairākus mainīgos, nosakot tiem nosaukumus.
Funkcijai argumentā jāpadod Stumbra datu faila adrese.

```priede
{vārds; vecums} : STUMBRS("testadati.st")


ja vecums < 18 {
    drukāt("Neraža! Nevar piedalīties vēlēšanās.")
}
```

Mainīgo nosaukumi tiek noteikti Priedes kodā un it neatkarīgi no tiem, kas noteikti Stumbra datu shēma. Datu tipi tiek noteikti no shēmas.