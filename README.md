# Implementación de TSP con Recocido Simulado: La Venganza
## Esta vez fue personal.

Este proyecto fue creado porque [su encarnación anterior no consiguió obtener la solución óptima](https://github.com/dixego/simulated_annealing)
a la instancia del problema presentada. Esta lo hace. Y lo hace rápido.

A decir verdad, no estoy seguro de por qué lo hace tan rápido.

Los cambios con respecto al proyecto arriba presentado son los siguientes:

* Se eliminó por completo la representación de las ciudades, ya que no proveían información útil para la resolución del problema. Es decir, *se redujo la grasa*.
* Se cambió la implementación de la Solución como un camino de ids de ciudades junto a su información más importante, como su costo,
la distancia promedio de las ciudades así como la distancia máxima, junto con la matriz de distancias entre ciudades.
* Se implementó la búsqueda de distancias entre ciudades como una matriz de adyacencias en vez de una serie de diccionarios asociados a cada ciudad.
* Se realizaron correcciones menores sobre el estilo del código.
* No se proveen comentarios sobre el código.

Por lo demás, el proceso de recocido es casi una copia exacta del presentado en el proyecto de arriba, así como el uso del archivo de configuración.

Para correr el proyecto, solo hace falta hacer `cargo run --release` y ver la magia ocurrir.
La configuración actual guardada en el repositorio está configurada para encontrar la solución óptima. En mi computadora, acaba como en 5 segundos.
