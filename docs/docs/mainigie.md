---
sidebar_position: 3
---

# Mainīgie

Mainīgie ir veids kā saglabāt informāciju datora atmiņā.

Mainīgajos var uzglabāt dažāda veida informāciju: skaitļus, tekstu un būla vērtību, kas varbūt pozitīva vai negatīva.

## Datu tipi:

- `skaitlis` vai `sk` - skaitliska vērtība
- `teksts` vai `tk` - teksta vērtība
- `būls` vai `bl` - būla vērtība
- `decim` - decimālskaitļa vērtība

```priede
skaitlis a : 2

izvade(a)
```

```priede
teksts a : "Sveika, pasaule!"

izvade(a)
```

Būla vērtības ir `0` vai `1`, vai `Jā` vai `Nē`.

Būli ir veids kā samazināt nepieciešamo datora atmiņas daudzumu programmas izpildei.

```priede
būls a : Jā
būls b : Nē


izvade(a)
izvade(b)
```

```priede
būls a : Jā

ja a {
    izvade("Būla mainīgais a ir patiess")
}
```

## Mainīgo pārdefinēšana

Tāpēc jau tos sauc par 'mainīgajiem', jo to vērtība var tikt mainīta programmas izpildes gaitā.

```priede
sk a : 3

izvade(a)

a : 4

izvade(a)
```

Mainīgajam norādot jaunu vērtību, nav nepieciešams atkārtoti minēt tā datu tipu.

### Iekļautās matemātiskās darbības

Bieži nepieciešams jau esošam mainīgam pieskaitīt vērtību. Tā vietā lai to rakstītu šādi:

```priede
a : a + 3
```

...Priede piedāvā to darīt vienkāršāk

```priede
a +: 3
```

Abi piemēri dara vienu un to pašu.

Ja nepieciešams mainīgajam pieskaitīt skaitli `1`, tad pieraksts var būt vēl vienkāršāks.

```priede
a++
```

## Pieejamie operatori:

- `:` Norāda mainīgajam vērtību
- `+:` Pieskaita mainīgajam vērtību
- `++` Pieskaita mainīgajam `1`
- `-:` Atņem no mainīgā
- `--` Atņem no mainīgā `1`
- `*:` Reizina ar mainīgo
- `/:` Dala ar mainīgo
- `!` Apgriež būla vērtību

## Mainīgo darbības zonas

Mainīgais pastāv tajā blokā, kurā tas ir definēts un šī bloka apakšblokos


```priede
sk a : 4

izvade(a) // Ir iespējams piekļūt mainīgajam a
```

```priede
sk a : 4

ja 1 {
    izvade(a) // Ir iespējams piekļūt mainīgajam a
}
```

Nav iespējams piekļūt mainīgajiem, kas definēti apakšblokos

```priede

ja 1 {
    sk a : 4
}
izvade(a) // Nav iespējams piekļūt mainīgajam a
```

Apakšblokos tiek pārdefinēti mainīgie no augstākiem blokiem

```priede
sk c : 6
ja 1 {
    sk c : 7
    izvade(c) // Tiek izvadīts 7
}
izvade(c) // Tiek izvadīts 6
```