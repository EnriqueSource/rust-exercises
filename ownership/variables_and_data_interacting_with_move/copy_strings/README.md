# Copy Strings

`en`

`String` values are more complex values than integers, so their contents are stored in the Heap. The pointer with the address of the content is stored in the Stack. This is because the length of the` String` (not to be confused with string literals) is not fixed or known at compile time.

Not knowing the length of the variable, the copy of a `String` variable could have a high memory usage cost. This is why Rust does not allow copying `String` . Instead of copying, Rust **moves** the variable property, i.e. the value of the original variable, to another owner, **eliminating the previous reference** which is the original variable.

In the exercise, it is impossible to obtain the original variable because it has already been deleted, and its value has been moved to the new variable.

<br />

`de`

`String` Werte sind komplexere Werte als Integer-Werte, daher wird ihr Inhalt im Heap gespeichert. Der Zeiger mit der Adresse des Inhalts wird im Stack gespeichert. Dies liegt daran, dass die Länge des `Strings` (nicht zu verwechseln mit String-Literalen) zur Kompilierzeit nicht festgelegt oder bekannt ist.

Da die Länge der Variablen nicht bekannt ist, könnte das Kopieren einer `String` Variablen einen hohen Speicherverbrauch verursachen. Aus diesem Grund erlaubt Rust das Kopieren von `String` nicht. Anstelle des Kopierens **verschiebt** Rust die Variableneigenschaft, d.h. den Wert der Originalvariablen, zu einem anderen Eigentümer, **wodurch die vorherige Referenz, die Original variable eliminiert wird** .

In der Übung ist es unmöglich, die ursprüngliche Variable zu erhalten, da sie bereits gelöscht wurde und ihr Wert auf die neue Variable verschoben wurde.

<br />

`es`

Los valores de tipo `String` son valores más complejos que los enteros, por lo que el contenido de estos es guardado en el Heap (memoria de montón). El puntero con la dirección del contenido es guardado en el Stack. Esto es así porque la longitud de los `String` (no confundir con string literales) no es fija ni conocida en tiempo de compilación.

Al no conocer la longitud de la variable, la copia de una variable de tipo `String` podría tener un costo de uso de memoria elevado. Es por esto que Rust no permite copiar `String` . En lugar de copiar, Rust **mueve** la variable de propiedad, es decir, el valor de la variable original, pasa a tener otro propietario, **eliminando la referencia anterior** que es la variable original.

En el ejercicio, es imposible poder obtener la variable original porque esta ya ha sido borrada, y su valor se ha _movido_ a la nueva variable.
