# PRIEDE

Interpretēta programmēšanas valoda ar sintaksi latviešu valodā.
Ļoti agrīnā stadijā.

Interpretators rakstīts Rust valodā un izmanto [Hime](https://cenotelie.fr/projects/hime) parsera ģeneratoru.

## Sveika pasaule piemērs:

    drukāt("Sveika, pasaule!")

## Tiešsaistes interpretators izmantojot WebAsembly

https://priede-editor.vercel.app/

## Piezīmes

### Arī, ja vērtība nav ielikta mainīgajā, tai tiek piešķirts datu tips, lai to izmantotu tālāk. Parseris šo datu tipu nosaka pēc teksta simbolu skaita

- Mazāk par 9 simboliem => i32
- 9 - 18 => i64
- 19 un vairāk => u64

Ieliekot vērtību mainīgajā, tās tips tiek pārrakstīts uz to, kas norādīts mainīgā deklarācijā

### Pašlaik netiek atļautas salīdzinājumu un aritmētiskās darbības ar vērtībām kurām ir atšķirīgi datu tipi
