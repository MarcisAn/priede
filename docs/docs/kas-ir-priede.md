---
sidebar_position: 1
---

# Kas ir Priede?

Priede ir vienkārši lasāma un rakstāma programmēšanas valoda ar sintaksi latviešu valodā.

## Pāris piemēri:

```priede
izvade("Sveika, pasaule!")

```

```priede

skaitlis a : 2

izvade(a)

```

```priede

skaitlis a : 2

ja a > 1 {
    izvade(a)
}

```

```priede
ja 2 = 2 {
    izvade("Divi ir vienāds ar divi")
}
citādi{
    izvade("Matemātika sabruka")
}
```

```priede

izvade("Sveika, pasaule!")

//šis ir komentārs, kas tiks ignorēts izpildot programmu

/*
Šis arī ir komentārs, bet tas
var aizņemt vairākas rindiņas
*/

```

```priede
teksts a : "Sveika, pasaule!"

izvade(a)
```

```priede
funkc piesk_un_reiz(sk : saskaitamais; sk : reizinatajs) {
    atgriezt(saskaitamais + 5 * reizinatajs)
}

izvade(piesk_un_reiz(4;5))
```

```priede
skaitlis a : 0

kamēr (a < 5) {
    izvade("Teksts tiks printēts 5 reizes")
    a++
}
```
