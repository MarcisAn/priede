---
sidebar_position: 3
---

# Darbības ar skaitļiem

## Aritmētiskās darbības

```
drukāt(2 + 2)
drukāt(2 - 2)
drukāt(2 * 2)
drukāt(2 / 2)
```

## Mainīgo iestatīšanā iekļautās darbības

Šie divi koda fragmenti ir vienādi. Tie abi palielina mainīgo `a` par 2.

```
a =: a + 2
```

```
a =+ 2
```

Visas pieejamās darbības:
`=+`
`=-`
`=*`
`=/`

## Salīdzināšana

### Vai divas vērtības ir vienādas

`=`

```
sk a =: 2
sk b =: 2

ja a = b {
    drukāt("vienādi")
}
citādi {
    drukāt("nav vienādi")
}
```

### Vai divas vērtības ir atšķirīgas

`!=`

```
sk a =: 2
sk b =: 2

ja a != b {
    drukāt("nav vienādi")
}
citādi {
    drukāt("vienādi")
}
```

### Vai viena vērtība lielāka, vai mazāka par otru

`>`, `<`

```
sk a =: 2
sk b =: 2

ja a > b {
    drukāt("a ir lielāks par b")
}
citādi {
    drukāt("a ir mazāks par b")
}
```

### Vai viena vērtība lielāka, vai mazāka par otru - ieskaitot

`>=`, `<=`

```
sk a =: 2
sk b =: 2

ja a >= b {
    drukāt("a ir lielāks vai vienāds par b")
}
citādi {
    drukāt("a ir mazāks par b")
}
```
