---
sidebar_position: 4
---

# Cikli

Reizēm, kodu ir vajadzības atkārtot vairākas reizes, tomēr tā vietā lai to rakstītu vairākas reizes, var izmantot ciklus.

## Vienkāršs cikls

```priede
izvade("Teksts tiks izprintēts 3 reizes")
izvade("Teksts tiks izprintēts 3 reizes")
izvade("Teksts tiks izprintēts 3 reizes")
```

Mēs varam aizvietot ar:

```priede
atkārtot 3 {
    izvade("Teksts tiks izprintēts 3 reizes")
}
```

```priede
skaitlis atkartojumi : 3

atkārtot atkartojumi {
    izvade("Teksts tiks izprintēts 3 reizes")
}
```

## 'Kamēr' cikli

Ar `kamēr` ciklu mēs varam kodu atkārtot, kamēr izteiksme ir patiesa.

```priede
kamēr 2+2 = 4{
    izvade("Teksts tiks printēts bezgalīgi")
}
```

Šāds cikls darbosies bezgalīgi, jo `2+2` vienmēr būs `4` un šī izteiksme nemainīies laika gaitā.

```priede
skaitlis a : 0

kamēr (a < 5) {
    izvade("Teksts tiks printēts 5 reizes")
    a++
}
```

Šis cikls darbosies tikai `5` reizes, jo katru reizi mainīgais `a` tiek palielināts par vienu, kamēr tas vairs nav mazāks par `5` un tad cikls apstājās.

Izteiksmes var būt, vai nebūt apaļajās iekavās.
