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

```priede
skaitlis a : 2

izvade(a)
```

```priede
teksts a : "Sveika, pasaule!"

izvade(a)
```

Būla vērtības ir `1` vai `2` vai arī `PATIESS` un `NEPATIESS`, vai arī saīsinātā versija `PAT` un `NEPAT`.

Būli ir veids kā samazināt nepieciešamo datora atmiņas daudzumu programmas izpildei.

```priede
būls a : PAT
būls b : NEPAT


izvade(a)
izvade(b)
```

```priede
būls a : PAT

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
