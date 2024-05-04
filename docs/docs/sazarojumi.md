---
sidebar_position: 2
---

# Koda sazarojumi

```priede
ja 2 = 2 {
    izvade("Divi ir vienāds ar divi")
}

ja 2+2 = 4 {
    izvade("Divi plus divi ir vienāds ar četri")
}
```

Pēc vārda `ja` seko izteiksme, kas nosaka vai sazarojums tiks izpildīts. Pēc tam, kvadrātiekavās seko kods, kas tiks izpildīts gadījumā ja izteiksme izrādīsies patiesa

Izteiksme var būt vai nebūt apaļajās iekavās

```priede
ja 2 = 2 {
    izvade("Divi ir vienāds ar divi")
}

ja (2 = 2) {
    izvade("Divi ir vienāds ar divi")
}
```

Pēc šī bloka var sekot vēl viens bloks, kas izpildās, ja izteiksme ir nepatiesa. To ievada ar vārdu `citādi`.

```priede
ja 2 = 2 {
    izvade("Divi ir vienāds ar divi")
}
citādi{
    izvade("Matemātika sabruka")
}
```

## Salīdzinājumi

### Pieejamie operatori:

- `=` - abas vērības ir vienādas
- `>` - vērtība pa kreisi ir lielāka
- `<` - vērtība pa labi ir lielāka
- `>=` - vērtība pa kreisi ir lielāka vai vienāda
- `<=` - vērtība pa labi ir lielāka vai vienāda
- `!=` - vērtības nav vienādas

```priede
ja 3 > 2 {
    izvade("Trīs ir lielāks par divi")
}

ja 1 < 2 {
    izvade("Viens ir mazāks par divi")
}

ja 5 != 2 {
    izvade("Pieci nav vienāds ar divi")
}
```

## Vairāku izteiksmju savienošana

### Pieejamie operatori

- `un` - abām izteiksmēm jābūt patiesām, lai izpildītos kopējā izteiksme
- `vai` - jāizpildās vienai vai abām izteiksmēm, lai izpildītos kopējā izteiksme
- `xvai` - jāizpildās tikai vienai no abām izteiksmēm, lai izpildītos kopējā izteiksme

```priede
ja 3 > 2 un 5 < 7 {
    izvade("Trīs ir lielāks par divi un pieci ir mazāks par septiņi")
}

ja 1 < 2  vai 3 > 1{
    izvade("Viens ir mazāks par divi vai trīs lielāks par viens")
}

ja 1 < 2 xvai 3 < 1{
    izvade("Viens ir mazāks par divi, bet trīs nav mazāks par viens")
}
```
