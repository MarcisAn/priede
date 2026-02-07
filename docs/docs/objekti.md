---
sidebar_position: 8
---

# Objekti

Objekti ļauj vienā mainīgajā glabāt vairākus datus. Piemēram, ja mēs vēlamies uzglabāt datus par kādu koku, tad varētu, protams to darīt ar atsevišķiem mainīgajiem.

```priede
tk suga: "priede"
sk augstums_metros: 20
sk diametrs_metros: 1
```

Bet ja mēs vēlamies uzglabāt datus par vairākiem cilvēkiem, 

```priede
tk suga1: "priede"
sk augstums_metros1: 20
sk diametrs_metros1: 1

tk suga2: "egle"
sk augstums_metros2: 15
sk diametrs_metros2: 0,5
```

tad šāds risinājums nav elegants. Nepieciešams veids, kā uzglabāt vairākus datu punktus par vienu vienību(šajā gadījumā - koku). Priede to piedāvā darīt ar objektiem.

## Objekta sagataves definēšana

Objekts satur vairākus mainīgos vienā mainīgajā. Objektam ir lauki, kuri katrs ir kā mainīgais, bet tomēr visi atrodās objektā. Iepriekšējā piemērā, objekta lauciņi būtu: "vārds", "uzvārds", "nodarbošanās".

Lai varētu izmantot objektu, vispirms jādefinē tā sagatave. Iepriekš minēts par mainīgo definēšanu. Tajā gadījumā, definējot mainīgo, tam tiek iedots nosaukums un vērtība. Definējot mainīgā sagatavi, tiek norādīts tās nosaukums, šajā gadījumā "cilvēks", un visi objekta lauki un to datu tipi.

```priede
objekts koks {
    tk suga
    sk augstums
    sk diametrs
}
```

## Objekta izmantošana

Un tālāk varam definēt mainīgos, kur šis objekts, ar nosaukumu `koks` ir datu tips un var tikt izmantots tāpāt, kā Priedes iebūvētie datu tipi.

```priede
koks koks1 : {suga: "priede" augstums: a diametrs: 4}
```

Varam ielikt objektus sarakstos.

`saraksts [koks] koki` norāda uz sarakstu, kura elementi ir ar datu tipu `koks` un saraksta mainīgā nosaukums ir `koki`.

```priede
objekts koks {
    tk suga
    sk augstums
    sk diametrs
}


saraksts [koks] koki : [koks1; {suga: "egle" augstums: 6 diametrs: 5}]


izvade(koki)
```

## Objekta lauku izmantošana

Nolasīt lauku no objekta, varam ar kvadrātiekavām, kurās jāieraksta lauka nosaukums pēdiņās vai ar punktu un lauka nosaukumu.

`objekts["lauka nosaukums"]`

vai

`objekts.lauka_nosaukums`

Lauka nosaukums būs tāds, kāds tas ir definēts objekta sagatavē.

```priede
koks koks1 : {suga: "priede" augstums: a diametrs: 4}


izvade("Koka suga ir šāda:")
izvade(koks1["suga"])
izvade(koks1.suga)

```