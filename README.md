# PRIEDE

Interpretēta programmēšanas valoda ar sintaksi latviešu valodā.
Ļoti agrīnā stadijā.

Interpretators rakstīts Rust valodā un izmanto [Hime](https://cenotelie.fr/projects/hime) parsera ģeneratoru.

## Sveika pasaule piemērs:

    drukāt("Sveika, pasaule!")

## Tiešsaistes interpretators izmantojot WebAsembly

https://priede.vercel.app/

## Jautājumi, kuros labprāt dzirdētu citus viedokļus

- Vai salīdzinājumiem izmantot divas vienādības zimēs '==', kā daudzās citās valodās, vai vienu '='?
- Vai sazarojumiem ("ja" blokiem) nosacījumu likt iekavās? (tas pats attiecās uz cikliem un citām valodas funkcijām, kuras pieraksta līdzīgi)
- Novērtēšu jebkādas atsauksmes un ieteikumus.

## Piezīmes

### Arī, ja vērtība nav ielikta mainīgajā, tai tiek piešķirts datu tips, lai tā varētu tikt izmantota tālāk. Parseris šo datu tipu nosaka pēc teksta simbolu skaita

Varētu arī noteikt precīzāk, skatoties uz skaitļiem, bet jāatstāj rezerve, tāpēc šis varbūt nav no sliktākajiem variantiem.

- Mazāk par 9 simboliem => i32
- 9 - 18 => i64
- 19 un vairāk => u64

Ieliekot vērtību mainīgajā, tās tips tiek pārrakstīts uz to, kas norādīts mainīgā deklerācijā

### Pašlaik netiek atļautas salīdzinājumu un aritmētiskās darbības ar vērtībām kurām ir atšķirīgi datu tipi
