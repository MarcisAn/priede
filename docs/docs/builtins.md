---
sidebar_position: 7
---

# Iebūvētās funkcijas

Funkcijas Priedē var definēt lietotājs, bet daļa funkciju jau ir iekļautas Priedē.

`izvade()` funcija jau parādījās iepriekš dokumentācijā, bet ir vēl vairākas citas iebūvētās funkcijas.

## Izvade bez jaunas rindiņas

`izvadetp()` funkcija neizvada jaunu rindiņu aiz printējamā teksta. Noderīgi tad, ja jāizvada vairāk teksta vienā rindiņā.

## Ievade

```priede
izvade("Kāds ir tavs vārds?")

teksts vards : ievade()
```

Ievades funkcija vienmēr atgriezīs tekstu, pat ja ievadītie dati šķietami ir skaitlis.

```priede
izvade("Kāds ir tavs vecums?")

teksts vecums : ievade()

ja vecums = "0" {
    izvade("Kaut kas neiet kopā!")
}
```

Ievades funkcijas nedarbojās online redaktorā.

## Nejaušs skaitlis

Funkcija `jukums()` atgriež nejaušu skaitli noteiktās robežās.

```priede
sk skaitlis : jukums(0;10)

izvade(skaitlis)
//tiks izvadīts nejaušs skaitlis robežās 0-10
```
