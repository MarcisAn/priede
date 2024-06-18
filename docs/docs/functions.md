---
sidebar_position: 6
---

# Funkcijas

Reizēm nepieciešams kādu funkcionalitāti atkārtot vairākas reizes programmā. Tā vietā lai rakstītu kodu vairākas reizes, Priede piedāvā izmantot funkcijas.

```priede
funkc sasveicināties() {
    izvade("Sveika pasaule!")
}
sasveicināties()
izvade("Tagad padaram kaut ko citu")
sasveicināties()
```

Šajā piemērā mēs varam divreiz nerakstīt rindiņu `izvade("Sveika pasaule!")`, bet rakstīt to funkcijā, ko tālāk varam izsaukt pēc vajadzības. Šis padara kodu vieglāk labojamu. Ja mēs pēkšņi vēlētos izmainīt sveiciena tekstu uz "Sveiciens no Prides", tad nevajadzētu tekstu mainīt vairākās vietās, bet gan tikai vienreiz, funkcijas definīcijā.



## Funkcijas argumenti

Gadījumu, kad mēs gribam lai funkcija dara pilnīgi to pašu vairākas reizes, ir maz. Biežāk mēs vēlamies funkcijai iedot kādus datus apstrādāšanai. Tādejādi, funkciju var izmantot visā kodā, bet ar nelielām izmaiņām.

Definējot funkciju, iekavās jānorāda tās arugmenti, atdalīti ar semikolu. Argumentam vispirms jānorāda tā datu tips, tad nosaukums. Šo nosaukumu funkcijas ietvaros varēs izmantot kā mainīgo.

```priede
funkc piesk5(sk a) {
    izvade(a + 5)
}

piesk5(4)
piesk5(2)
```

Šajā piemērā mēs izveidojam funkciju, kura pieskaita argumentam skaitli `5` un izdrukā rezultātu. Pēc tam mēs varam izsaukt šo funkciju ar dažādiem skaitļiem un funkcija veiks to pašu darbību ar tiem abiem.

### Vairāki argumenti

Funkcijas var saņemt vairākus argumentus, tos atdalot ar semikoliem gan definīcijā, gan izsaucot funkciju.

```priede
funkc piesk_un_reiz(sk saskaitamais; sk reizinatajs) {
    izvade(saskaitamais + 5 * reizinatajs)
}

piesk_un_reiz(4;5)
piesk_un_reiz(2;3)
```

## Vērtību atgriešana

Līdz šim mēs tikai printējām vērtības no funkcijas, lai gan, bieži vien, mēs vēlētos saņemt vērtības atpakaļ.

```priede
funkc piesk_un_reiz(sk : saskaitamais; sk : reizinatajs) {
    atgriezt (saskaitamais + 5 * reizinatajs)
}

izvade(piesk_un_reiz(4;5))
```

Atšķirībā no iepriekšējā piemēra, šeit funkcija pati neko neprintē, bet gan atgriež vērtību, kas tiek izpirntēta, atsevišķi, izsaucot funkciju.

