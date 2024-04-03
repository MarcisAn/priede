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

```
skaitlis a : 2

drukāt(a)
```

```
teksts a : "Sveika, pasaule!"

drukāt(a)
```

Būla vērtības ir `1` vai `2` vai arī `PATIESS` un `NEPATIESS`, vai arī saīsinātā versija `PAT` un `NEPAT`.

Būli ir veids kā samazināt nepieciešamo datora atmiņas daudzumu programmas izpildei.

```
būls a : PAT

drukāt(a)
```

```
būls a : PAT

ja a {
    drukāt("Būla mainīgais a ir patiess")
}
```

## Mainīgo pārdefinēšana

Tāpēc jau tos sauc par 'mainīgajiem', jo to vērtība var tikt mainīta programmas izpildes gaitā.

```
sk a : 3

drukāt(a)

a : 4

drukāt(a)
```

Mainīgajam norādot jaunu vērtību, nav nepieciešams atkārtoti minēt tā datu tipu.

### Iekļautās matemātiskās darbības

Bieži nepieciešams jau esošam mainīgam pieskaitīt vērtību. Tā vietā lai to rakstītu šādi:

```
a : a + 3
```

...Priede piedāvā to darīt vienkāršāk

```
a +: 3
```

Abi piemēri dara vienu un to pašu.

Ja nepieciešams mainīgajam pieskaitīt skaitli `1`, tad pieraksts var būt vēl vienkāršāks

```
a++
```

#### Pieejamie operatori:

- `:`
- `+:`
- `++`
- `-:`
- `*:`
- `/:`
