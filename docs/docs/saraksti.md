---
sidebar_position: 5
---

# Saraksti

Reizēm nepieciešams uzglabāt vairākas vērtības ar vienādu nozīmi.

```priede
tk auglis1 : "ābols"
tk auglis2 : "bumbieris"
tk auglis3 : "banāns"
tk auglis4 : "apelsīns"
```

Šis piemērs nebūtu vēlams, jo, pirmkārt, jāraksta daudz teksta, otrkārt, nevar vienkārši pievienot vai noņemt vērtības.

Tā vietā var izmantot sarakstus.

## Sarakstu definēšana

```priede
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]
```

Sarakstu definējot, vispirms jānorāda vārds `saraksts`, tad datu tips vērtībām sarakstā, kam seko saraksta nosaukums. Tad, kvadrātiekavās saraksta sākotnējās vērtības atdalītas ar semikoliem.

## Datu nolasīšana no saraksta

```priede
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]


izvade(augļi[0])
```

Lai nolasītu saraksta konkrētu pozīciju izmanto saraksta nosaukumu, kuram seko kvadrātiekavas ar šī elementa pozīciju sarakstā, **sākot skaitīt no nulles**.

## Elementa pārdefinēšana

```priede
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]


izvade(augļi[0])

augļi[0] : "kivi"

izvade(augļi[0])

```

## Saraksta garuma atrašana

```priede
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]

izvade(garums(augļi))
```

## Visu elementu izdruka

```priede
saraksts [teksts] augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]


sk skaitītajs : 0

kamēr skaitītajs < garums(augļi) {
    izvade(augļi[skaitītajs])
    skaitītajs++
}
```
