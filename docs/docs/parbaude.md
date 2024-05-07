---
sidebar_position: 8
---

# Ievades pārbaudīšana

Priede piedāvā pārbaudīt lietotāja ievadi izmantojot funkciju `Ja`

## Pamācība soļos kā izveidot vienkāršu ievades parbaudītāju

### 1. Solis ievades pieprasīšana
1. solī mēs pieprasam lietotājam ievadīt datus kurus pēctam mēs apstrādāsim.

```priede
sk ievade : ievade()
```
### 2. Solis Ievades apstrādāšana
Otrajā solī lietotāja ievade tiek pārbaudīta(salīdzināta) ar šajā gadijumā atbilstību "123"

```priede
ja ievade = "123"{
    izvade("ievade atbilst")
}
```
### 3. Solis
Trešajā solī nosakām izvadi, ja lietotāja atbilde nesaktrīt ar "123"

```priede
citādi{
    izvade("Ievade neatbilst")
}
```

### Pilns kods

Šeit ir pieejams pilns kods

```priede
sk ievade : ievade()
ja ievade = "123"{
    izvade("ievade atbilst")
}
citādi{
    izvade("Ievade neatbilst")
}
```
