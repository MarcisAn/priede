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
