# Integer Overflow

`en`

Integer Overflow


In this example, we first create a mutable variable of type unsigned integer `u8` called **n** . This type only admits numbers in the range from 0 to 255. Then we mutate this variable by assigning it a value greater than 255 (256). The compiler detects the error and goes into panic mode.


It is very important to take into account the “integer overflow” error. The error will be detected by the compiler **only** if the compiler is in **debug mode** . On the contrary, **the compiler in release mode does not check this type of error** , so our program will fail, and we will not have any error from the compiler.

<br />

`de`

Integer-Überlauf


In diesem Beispiel erstellen wir zunächst eine veränderbare Variable vom Typ unsigned integer `u8` namens n. Dieser Typ lässt nur Zahlen im Bereich von 0 bis 255 zu. Dann verändern wir diese Variable, indem wir ihr einen Wert größer als 255 (256) zuweisen. Der Compiler erkennt den Fehler und geht in den Panikmodus über.


Es ist sehr wichtig, den „Integer Overflow“-Fehler zu berücksichtigen. Dieser Fehler wird vom Compiler **nur** erkannt, wenn er sich im **Debug-Modus** befindet. Im Gegensatz dazu **prüft der Compiler im Freigabemodus diese Art von Fehler nicht** ,sodass unser Programm fehlschlägt und wir keine Fehler vom Compiler erhalten.

<br />

`es`

En este ejemplo creamos primero una variable mutable de tipo entero sin signo `u8` llamada **n** . Este tipo solo admite números en el rango que va desde 0 hasta 255. Después mutamos dicha variable asignándole un valor superior a 255 (256). El compilador detecta el error y entra en modo pánico.

Es muy importante tener en cuenta el error "desbordamiento de enteros". El error será detectado por el compilador **solo** si este está en **modo debug** . Por el contrario, **el compilador en modo release no chequea este tipo de error** , por lo que nuestro programa fallará y no tendremos ningún aviso de error por parte del compilador.   
