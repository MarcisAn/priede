---
sidebar_position: 2
---

# Koda sazarojumi

```
ja 2 = 2 {
    drukāt("Divi ir vienāds ar divi")
}

ja 2+2 = 4 {
    drukāt("Divi plus divi ir vienāds ar četri")
}
```

Pēc vārda `ja` seko izteiksme, kas nosaka vai sazarojums tiks izpildīts. Pēc tam, kvadrātiekavās seko kods, kas tiks izpildīts gadījumā ja izteiksme izrādīsies patiesa

Izteiksme var būt vai nebūt apaļajās iekavās

```
ja 2 = 2 {
    drukāt("Divi ir vienāds ar divi")
}

ja (2 = 2) {
    drukāt("Divi ir vienāds ar divi")
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

```
ja 3 > 2 {
    drukāt("Trīs ir lielāks par divi")
}

ja 1 < 2 {
    drukāt("Viens ir mazāks par divi")
}

ja 5 != 2 {
    drukāt("Pieci nav vienāds ar divi")
}
```

## Vairāku izteiksmju savienošana

### Pieejamie operatori

- `un` - abām izteiksmēm jābūt patiesām, lai izpildītos kopējā izteiksme
- `vai` - jāizpildās vienai vai abām izteiksmēm, lai izpildītos kopējā izteiksme
- `xvai` - jāizpildās tikai vienai no abām izteiksmēm, lai izpildītos kopējā izteiksme

```
ja 3 > 2 un 5 < 7 {
    drukāt("Trīs ir lielāks par divi un pieci ir mazāks par septiņi")
}

ja 1 < 2  vai 3 > 1{
    drukāt("Viens ir mazāks par divi vai trīs lielāks par viens")
}

ja 1 < 2 xvai 3 < 1{
    drukāt("Viens ir mazāks par divi, bet trīs nav mazāks par viens")
}
```
