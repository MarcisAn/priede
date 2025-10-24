---
sidebar_position: 8
---

# Objekti

Objekti ļauj vienā mainīgajā glabāt vairākus datus. Piemēram, ja mēs vēlamies uzglabāt datus par kāda cilvēku vārdu, uzvārdu un nodarbošanos, tad varētu, protams to darīt ar atsevišķiem mainīgajiem.

```priede
tk vārds: "Miķelis"
tk vārds: "Jaunlēcējs"
tk nodarbošanās: "Bezdarbnieks"
```

Bet ja mēs vēlamies uzglabāt datus par vairākiem cilvēkiem, 

```priede
tk vārds1: "Miķelis"
tk vārds1: "Jaunlēcējs"
tk nodarbošanās1: "Bezdarbnieks"

tk vārds2: "Emīls"
tk vārds2: "Veclēcējs"
tk nodarbošanās2: "Slepenais aģents"
```

tad šādi neies krastā. Nepieciešams kāds smalkāks veids, kā uzglabāt vairākus datu punktus par vienu vienību(šajā gadījumā cilvēku). Priede to piedāvā darīt ar objektiem.

## Objekta sagataves definēšana

Objekts satur vairākus mainīgos vienā mainīgajā. Objektam ir lauki, kuri katrs ir kā mainīgais, bet tomēr visi atrodās objektā. Iepriekšējā piemērā, objekta lauciņi būtu: "vārds", "uzvārds", "nodarbošanās".

Lai varētu izmantot objektu, vispirms jādefinē tā sagatave. Iepriekš minēts par mainīgo definēšanu. Tajā gadījumā, definējot mainīgo, tam tiek iedots nosaukums un vērtība. Definējot mainīgā sagatavi, tiek norādīts tās nosaukums, šajā gadījumā "cilvēks", un visi objekta lauki un to datu tipi.

```priede
objekts cilvēks {
    tk: vārds
    tk: uzvārds
    tk: nodarbošanās
}
```

## Objekta izmantošana

Un tālāk varam definēt mainīgos, kur šis objekts, ar nosaukumu `cilvēks` ir datu tips.

`cilvēks`, tagad var tikt izmantots kā datu tips, tāpāt, kā Priedes iebūvētie datu tipi.

```priede
cilvēks cilvēks1 : {vārds: "Juris" uzvārds: "Veclēcējs" nodarbošanās: "Džeimss Bonds"}
```

Varam ielikt objektus sarakstos.

`cilvēks saraksts cilvēki` ir tāds dulls vārdu savirpinājums, bet tas norāda uz sarakstu, kura elementi ir ar datu tipu `cilvēks` un saraksta mainīgā nosaukums ir `cilvēki`.

```priede
objekts cilvēks {
    tk: vārds
    tk: uzvārds
    tk: nodarbošanās
}


cilvēks saraksts cilvēki : [{vārds: "Juris" uzvārds: "Veclēcējs" nodarbošanās: "Džeimss Bonds"}; {vārds: "Miķelis" uzvārds: "Jaunlēcējs" nodarbošanās: "Čūsku dresētājs"}]

izvade(cilvēki)
```

## Objekta lauku izmantošana

Nolasīt lauku no objekta, varam izmantojot punktu, un tad lauka nosaukumu.

`objekts.lauks`

Lauka nosaukums būs tāds, kāds tas ir definēts objekta sagatavē.

```priede
cilvēks cilvēks1 : {vārds: "Juris" uzvārds: "Veclēcējs" nodarbošanās: "Džeimss Bonds"}

izvade(cilvēks1.vārds)
```