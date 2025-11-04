# Análisis personal sobre AWS Lambda y ejemplo práctico

Este documento es un resumen y reflexión personal sobre el video *“Funciones Lambda (AWS): ¿Qué son y cómo funciona esta tecnología?”* de *Rubén Carvajal*, el cual explica de manera clara qué son las Lambdas dentro de la nube de Amazon, cómo funcionan internamente, y muestra un ejemplo de despliegue sencillo desde la consola de AWS.

---

## 1. Características principales de una función Lambda (00:36)

En mi interpretación, AWS Lambda representa una forma moderna de ejecutar código sin necesidad de administrar servidores.  
El modelo *serverless* que propone Amazon permite concentrarse únicamente en la lógica del negocio, ya que toda la infraestructura —escalado, mantenimiento y disponibilidad— es gestionada automáticamente por AWS.

Una de las ideas que más destaco del video es la *ejecución por eventos*: las Lambdas no están “encendidas” todo el tiempo, sino que se activan cuando ocurre algo, como una carga en un bucket S3, una solicitud HTTP en API Gateway o un mensaje en una cola.  
Eso las hace altamente *eficientes y rentables*, ya que se paga únicamente por el tiempo en que el código realmente corre.

Además, aprendí que Lambda escala de manera automática y transparente. Si llegan 10 o 10.000 solicitudes, AWS crea tantas instancias como sean necesarias, lo que permite absorber picos sin preocuparse por el rendimiento del servidor.

---

## 2. Casos de uso (03:10)

El video menciona varios ejemplos reales donde Lambda es útil, y coincido totalmente:

- *APIs sin servidor* usando API Gateway y Lambda, ideales para proyectos ligeros o microservicios.  
- *Procesamiento automático de archivos* al subirlos a S3 (por ejemplo, validaciones o generación de miniaturas).  
- *Tareas programadas* mediante EventBridge, como backups o reportes automáticos.  
- *Procesamiento de datos en flujo*, conectando Kinesis o SQS con Lambda para ETL en tiempo real.

Me parece muy interesante cómo estas funciones se pueden combinar para construir arquitecturas completas sin depender de servidores tradicionales.

---

## 3. Cómo orquesta Amazon esta tecnología (06:31)

Algo que me llamó la atención del video es la explicación de *cómo Amazon maneja internamente las Lambdas*.  
Cada vez que una función se ejecuta, AWS crea un entorno de ejecución con el lenguaje elegido (Python, Node.js, Java, etc.) y las dependencias necesarias. Si hay múltiples invocaciones, el servicio *reutiliza esos entornos* para reducir latencia y mejorar el rendimiento.

También comprendí que existen aspectos como la *concurrencia*, que controla cuántas ejecuciones simultáneas puede manejar una función, y que se pueden ajustar para evitar sobrecargar otros servicios.  
Otro punto importante que aprendí es el fenómeno de los *“cold starts”* (arranques en frío), que ocurre cuando una función se ejecuta después de un tiempo inactiva y tarda un poco más en responder. AWS ofrece mecanismos como Provisioned Concurrency para mitigar este problema.

---

## 4. Despliegue rápido de una función Lambda (12:18)

El video finaliza mostrando un despliegue rápido desde la consola de AWS.  
El proceso me pareció muy intuitivo: basta con crear una nueva función desde cero (“Author from scratch”), seleccionar el runtime (por ejemplo, Python 3.x), otorgarle un rol básico de ejecución y pegar el código de la función directamente en el editor en línea.

Luego, se puede crear un evento de prueba —por ejemplo { "ping": true }— y al ejecutarlo, la consola muestra el resultado JSON devuelto por la función.  
También se puede integrar fácilmente con *API Gateway* para exponerla como un endpoint HTTP.

Este enfoque visual que muestra Rubén Carvajal me ayudó a entender de manera práctica cómo funciona Lambda por dentro y cómo se comporta en tiempo real durante la ejecución.

---

## Reflexión personal

Después de analizar el video, entendí que *Lambda no es solo “código sin servidor”*, sino una herramienta que cambia por completo la forma de pensar las aplicaciones: ahora cada evento puede ser una pieza independiente, escalable y económica.  
Sin embargo, también hay que ser consciente de sus límites: no todas las cargas son aptas para Lambdas (por ejemplo, procesos de larga duración o muy dependientes del estado).

En mi opinión, la clave está en *combinar Lambdas con otros servicios de AWS* como S3, DynamoDB o Step Functions, para aprovechar su poder sin caer en un “monolito serverless”.  
En mi caso, usaría Lambda para tareas pequeñas, reactivas y de bajo costo, como automatizar reportes, procesar archivos o responder peticiones API sin mantener un backend completo.

---

## 5. Ejemplo práctico en Python

Para complementar el análisis, creé una función Lambda sencilla escrita en *Python*, similar a la demostración del video, que responde un mensaje en formato JSON cuando se ejecuta.

El código se encuentra en hello_lambda/handler.py.
