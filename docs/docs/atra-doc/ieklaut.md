---
sidebar_position: 4
---

# Citu priedes failu iekļaušana

Citus Priedes koda failus var iekļaut, izmantojot relatīvo adresi - relatīvu pret failu, kas to iekļauj

Failus nevar iekļaut pārlūka interpretatorā

```
iekļaut "fails.pr"
iekļaut "../fails.pr"  //fails mapē augstāk par failu, kas to iekļauj
```

Iekļautais fails izpilda savu kodu tajā rindiņā, kurā tas ir iekļauts
