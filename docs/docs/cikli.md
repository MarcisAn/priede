---
sidebar_position: 4
---

# Cikli

Reizēm, kodu ir vajadzības atkārtot vairākas reizes, tomēr tā vietā lai to rakstītu vairākas reizes, var izmantot ciklus

## Vienkāršs cikls

```
drukāt("Teksts tiks izprintēts 3 reizes")
drukāt("Teksts tiks izprintēts 3 reizes")
drukāt("Teksts tiks izprintēts 3 reizes")
```

Mēs varam aizvietot ar:

```
atkārtot 3 {
    drukāt("Teksts tiks izprintēts 3 reizes")
}
```

```
skaitlis atkartojumi : 3
atkārtot atkartojumi {
    drukāt("Teksts tiks izprintēts 3 reizes")
}
```

## 'Kamēr' cikli

Ar `kamēr` ciklu mēs varam kodu atkārtot, kamēr izteiksme ir patiesa

```
kamēr 2+2 = 4{
    drukāt("Teksts tiks printēts bezgalīgi")
}
```

Šāds cikls darbosies bezgalīgi, jo 2+2 vienmēr būs 4, tātad šī izteiksme nemainīies laika gaitā.

```
skaitlis a : 0

kamēr (a < 5) {
    drukāt("Teksts tiks printēts 5 reizes")
    a++
}
```

Šis cikls darbosies tikai 5 reizes, jo katru reizi mainīgais `a` tiek palielināts par vienu, kamēr tas vairs nav mazāks par 5 un tad cikls apstājās.

Izteiksmes var būt, vai nebūt pēdiņās.
