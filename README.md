![PRIEDE](./banner.png)

https://priede.andersons-m.lv/

Viegli rakstāma programmēšanas valoda ar sintaksi latviski

## Funkcijas

- Saraksti
- Objekti
- Funkcijas
- Interpretators var darboties WebAssembly
- Integrēta testēšanas sistēma, kas nav saistīta ar programmas standartizvadi
- Automātiska koda formatēšana (WIP)
- VSCode paplašinājums ar koda iekrāsošanu
- Datu tipu pārbaudes kompilācijas laikā

## Piemēri

### Sveika pasaule

```
izvade("Sveika, pasaule!")
```

### Sazarojumi
```
ja 2 = 2 {
    izvade("Divi ir vienāds ar divi")
}
citādi{
    izvade("Matemātika sabruka")
}
```


### Mainīgie
```
skaitlis a : 3

izvade(a)

teksts b : "aaaa"

izvade(b)
```

### Cikli
```
skaitlis a : 0

kamēr (a < 5) {
    izvade("Teksts tiks printēts 5 reizes")
    a++
}
```
### Saraksti
```
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]


izvade(augļi[0])
```
### Funkcijas
```
funkc piesk5(sk a) {
    izvade(a + 5)
}

piesk5(4)
piesk5(2)
```
### Objekti
```
objekts Koks {
    tk suga
    sk augstums
    sk diametrs
}

Koks koks1 : {suga: "priede" augstums: 10 diametrs: 4}
```

Baitkoda kompilators un virtuālā mašīna ir atdalīti atsevišķā bibliiotēkā [Celsium](https://github.com/MarcisAn/celsium)
