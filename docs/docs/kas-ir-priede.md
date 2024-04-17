---
sidebar_position: 1
---

# Kas ir Priede?

Priede ir vienkārši lasāma un rakstāma programmēšanas valoda ar sintaksi latviešu valodā.

## Pāris piemēri:

```priede
drukāt("Sveika, pasaule!")

```

```priede

skaitlis a : 2

drukāt(a)

```

```priede

skaitlis a : 2

ja a > 1 {
    drukāt(a)
}

```

```priede
ja 2 = 2 {
    drukāt("Divi ir vienāds ar divi")
}
citādi{
    drukāt("Matemātika sabruka")
}
```

```priede

drukāt("Sveika, pasaule!")

//šis ir komentārs, kas tiks ignorēts izpildot programmu

/*
Šis arī ir komentārs, bet tas
var aizņemt vairākas rindiņas
*/

```

```priede
teksts a : "Sveika, pasaule!"

drukāt(a)
```

```priede
funkc piesk_un_reiz(sk : saskaitamais; sk : reizinatajs) {
    atgirest (saskaitamais + 5 * reizinatajs)
}

drukāt(piesk_un_reiz(4;5))
```

```priede
skaitlis a : 0

kamēr (a < 5) {
    drukāt("Teksts tiks printēts 5 reizes")
    a++
}
```
