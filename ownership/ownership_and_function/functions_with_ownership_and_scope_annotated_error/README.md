# Functions with ownership and scope annotated (error)

`en`

In this example we have tried to use the variable `s` once its value had been moved to the takes_ownership() function. Now, the owner of the value bound before to s is takes_ownership .

<br />

**There can only be one owner at a time** (Ownership rule 2 of 3).

`de`

In diesem Beispiel wollten wir die Werte von dem Variable `s` benutzen. Aber ist dieser Werte nach dem Function takes_ownership verschoben.

<br />

**Es kann immer nur einen Eigentümer zur gleichen Zeit geben** (Eigentumsregel 2 von 3).

`es`

En este ejemplo hemos intentado utilizar la variable `s` una vez que su valor había sido movido a la función `takes_ownership()` . Ahora, el propietario del valor enlazado antes a `s` es `takes_ownership` .

**Solo puede haber un propietario al mismo tiempo** (regla 2 de 3 del Ownership).
