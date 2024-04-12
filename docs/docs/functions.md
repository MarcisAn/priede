---
sidebar_position: 6
---

# Funkcijas

Reizēm nepieciešams kādu funkcionalitāti atkārtot vairākas reizes programmā.

```
sk aa : 2

aa * 2 + 1
drukāt(aa)
aa * 2 + 1
drukāt(aa)
aa * 2 + 1
drukāt(aa)
```

Tā vietā lai rakstītu kodu vairākas reizes, Priede piedāvā izmantot funkcijas

```
sk aa : 2

funkc reizinat_un_saskaitit() {
    aa : aa * 2 + 1
}

reizinat_un_saskaitit()
drukāt(aa)
reizinat_un_saskaitit()
drukāt(aa)

```
