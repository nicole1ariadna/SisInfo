# README — Laboratorio BI (Tableau) · TechStore

## 1) Objetivo
Replicar en **Tableau** el dashboard construido en Power BI para el dataset **TechStore**, cumpliendo los requisitos del laboratorio:
- **KPIs destacados (3)**: *Ventas*, *Cumplimiento de meta*, *Ticket Promedio*.
- **Gráficos**: 
  - **Líneas**: Ventas por mes
  - **Barras**: Top 10 productos
  - **Circular**: Ventas por categoría
  - **Tabla**: Performance vendedores
- **Requisitos técnicos (Tableau)**: 
  - **2 campos calculados** (mínimo) → `ImporteSeguro`, `Ventas` (además: `Descuento_num_pre`, `DescuentoSeguro` para limpiar descuento)
  - **1 parámetro de año** → `pAño` + `Year Filter`
  - **Actions entre hojas** → filtros desde *Top 10* y *Ventas por mes*

---

## 2) Datos de entrada
Archivos CSV utilizados (mismos que en Power BI):
- `ventas_techstore.csv`: `fecha`, `producto_id`, `cliente_id`, `cantidad`, `precio_unitario`, `descuento`, `canal_venta`, `vendedor`
- `productos.csv`: `producto_id`, `nombre_producto`, `categoria`, `marca`, `precio_costo`, `precio_venta`, `stock_actual`
- `clientes.csv`: `cliente_id`, `nombre_cliente`, `email`, `telefono`, `ciudad`, `fecha_registro`, `segmento`
- `metas_mensuales.csv`: `año`, `mes`, `meta_ventas`, `meta_unidades`, `meta_nuevos_clientes`

**Tipos clave en Origen de datos:**
- `fecha` como **Fecha**
- `año` y `mes` de metas como **Número entero**
- `descuento` puede venir como **cadena**; se limpia vía cálculos (abajo).

---

## 3) Modelo de datos (Relationships, no joins físicos)
En **Origen de datos**:
- Relacionar **ventas_techstore ↔ productos** por `producto_id = producto_id` (**Muchos→Uno**).
- Relacionar **ventas_techstore ↔ clientes** por `cliente_id = cliente_id` (**Muchos→Uno**).
- Relacionar **ventas_techstore ↔ metas_mensuales** por **dos pares**:  
  `YEAR([fecha]) = [año]` **y** `MONTH([fecha]) = [mes]` (**Muchos→Uno**).

> Usar **Relationships** evita duplicar filas y deja que Tableau resuelva la unión óptima según la vista.

---

## 4) Campos calculados (en español, con limpieza de descuento)
**Análisis → Crear campo calculado…**

### 4.1 Limpieza de descuento
```tableau
// Descuento_num_pre: limpia %, espacios y coma decimal -> número
IFNULL(
    FLOAT(
        REPLACE(
            REPLACE(
                REPLACE( LOWER(TRIM([descuento])) , "%", "" ),
            ",", "." ),
        " ", "" )
    )
, 0)
```

```tableau
// DescuentoSeguro: normaliza 5 -> 0.05; 0.10 queda igual
IF [Descuento_num_pre] > 1 THEN [Descuento_num_pre] / 100 ELSE [Descuento_num_pre] END
```

### 4.2 Métricas base
```tableau
// ImporteSeguro: usa descuento ya limpio
[cantidad] * [precio_unitario] * (1 - [DescuentoSeguro])
```

```tableau
// Ventas: suma del importe seguro
SUM([ImporteSeguro])
```

> Con `ImporteSeguro` y `Ventas` ya cumples **2 calculated fields**. Los de descuento son de apoyo (recomendados).

---

## 5) Parámetro de año + filtro
1. **Parámetro `pAño`**: *Datos → Crear parámetro…* → Entero, Lista (agrega 2024), Mostrar parámetro.  
2. **Year Filter** (campo calculado):
```tableau
YEAR([fecha]) = [pAño]
```
3. **Aplicación**: en **cada hoja**, arrastra `Year Filter` al **Estante Filtros** y marca **True**.

> Si agregas nuevos años, edita `pAño` y **Add values from** `YEAR([fecha])` o agrégalos manualmente.

---

## 6) Hojas del dashboard

### 6.1 KPIs (3 hojas)
- **KPI Ventas (COP)**  
  - **Filas**: `SUM([ImporteSeguro])` → mover a **Marcas → Etiqueta** y dejar **Filas** vacío.  
  - **Marcas**: **Texto**.  
  - **Formato**: **Moneda** (0–1 decimales). Título: *Ventas (COP)*.  
  - **Filtros**: `Year Filter = True`.

- **KPI Cumplimiento de meta (%) – solo 1 mes**  
  - Campo calculado:
  ```tableau
  IF COUNTD(DATETRUNC('month',[fecha])) = 1 THEN
      SUM([ImporteSeguro]) / NULLIF(SUM([meta_ventas]), 0)
  END
  ```
  - **Marcas**: **Texto** → a **Etiqueta**. **Formato**: **Porcentaje** (1–2 decimales).  
  - **Filtros**: `Year Filter = True`.  
  - **Comportamiento**: muestra % **solo** cuando se selecciona **un mes** (por filtro o acción).

- **KPI Ticket promedio (COP)**  
  - **Etiqueta**: `SUM([ImporteSeguro]) / NULLIF(SUM([cantidad]), 0)`  
  - **Marcas**: **Texto**. **Formato**: **Moneda**.  
  - **Filtros**: `Year Filter = True`.

### 6.2 Líneas — Ventas por mes
- **Columnas**: `DATETRUNC('month',[fecha])` (continuo).  
- **Filas**: `SUM([ImporteSeguro])` (o `ZN(SUM([ImporteSeguro]))` para ver 0 en meses sin ventas).  
- **Marcas**: **Línea**.  
- **Filtros**: `Year Filter = True`.  
- **Mostrar valores faltantes**: clic derecho en el eje → *Mostrar valores faltantes*.  
- Título: *Ventas por mes*.

### 6.3 Barras — Top 10 productos
- **Filas**: `productos[nombre_producto]`  
- **Columnas**: `SUM([ImporteSeguro])`  
- **Orden**: descendente por ventas.  
- **Filtro Top**: arrastrar `productos[nombre_producto]` a **Filtros** → pestaña **Top** → **Los 10 principales por** `SUM([ImporteSeguro])`.  
- **Filtros**: `Year Filter = True`.  
- Título: *Top 10 productos*.  
- **Contexto**: en esta hoja, **clic derecho** en `Year Filter` → *Agregar al contexto* (para recalcular Top 10 dentro del año/selección).

### 6.4 Circular — Ventas por categoría
- **Marcas**: **Circular**.  
- **Ángulo** y **Tamaño**: `SUM([ImporteSeguro])`.  
- **Color**: `productos[categoria]`.  
- **Etiqueta**: categoría + **%** (y valor si se desea).  
- **Filtros**: `Year Filter = True`.  
- Título: *Ventas por categoría*.

### 6.5 Tabla — Performance vendedores
- **Filas**: `vendedor` (o `Vendedor (NN)` = `IFNULL([vendedor],'Sin vendedor')`).  
- **Columnas**: usar **Nombres de medida**; en **Texto**: **Valores de medida**.  
- **Medidas incluidas** (y formato):  
  - **Ventas (COP)** → `ZN(SUM([ImporteSeguro]))` → Moneda  
  - **Unidades** → `ZN(SUM([cantidad]))` → Número (0)  
  - **Pedidos** → `ZN(COUNT([producto_id]))` → Número (0)  
  - **Descuento promedio** → `IFNULL(AVG([DescuentoSeguro]),0)` → %  
- **Orden**: por Ventas desc.  
- **Totales**: **Análisis → Totales → Mostrar totales de fila**.  
- **Filtros**: `Year Filter = True`.  
- Título: *Performance vendedores*.

---

## 7) Panel (dashboard) y Acciones
1. **Panel → Nuevo panel**. Tamaño: **Fijo** (p.ej., 1280×800) o **Automático**.  
2. Disposición:
   - Arriba: **KPI Ventas**, **KPI Cumplimiento**, **KPI Ticket**.  
   - Medio: **Top 10 productos** (izq) y **Ventas por mes** (der).  
   - Abajo: **Ventas por categoría** y **Performance vendedores**.  
3. **Parámetro `pAño`**: **Mostrar parámetro** y colócalo arriba.  
4. **Acciones**: **Panel → Acciones…**
   - **Filtro 1**: Origen = *Top 10 productos* → Destino = resto de hojas → **Seleccionar** → Al borrar = *Mostrar todos*.  
   - **Filtro 2**: Origen = *Ventas por mes* → Destino = *KPI Cumplimiento* (y opcional resto) → **Seleccionar** → Al borrar = *Mostrar todos*.

---

## 8) Colores corporativos (opcional)
Puedes usar una paleta con los colores de TechStore creando `Preferences.tps` en **Mis documentos → My Tableau Repository**, y luego **Color → Paleta** = *TechStore*.

---

## 9) Validación rápida
- `pAño = 2024`.  
- La línea muestra **ene–dic**; al clicar un mes, el **KPI Cumplimiento** aparece.  
- **Top 10** muestra exactamente 10 (o menos si hay <10 con ventas) y **ordena** desc.  
- El **Total** de la **tabla** coincide con **KPI Ventas (COP)** para el mismo filtro.  
- Sin selección de mes, el **KPI Cumplimiento** queda **vacío** (según lo pedido).

---

## 10) Checklist de requisitos (Tableau)
- [x] **2 calculated fields**: `ImporteSeguro`, `Ventas` (y limpieza de descuento opcional).  
- [x] **Parámetro de año**: `pAño` + `Year Filter` aplicado.  
- [x] **Actions**: desde *Top 10* y *Ventas por mes*.  
- [x] **KPIs (3)**: Ventas, Cumplimiento (solo 1 mes), Ticket promedio.  
- [x] **Líneas**: Ventas por mes.  
- [x] **Barras**: Top 10 productos.  
- [x] **Circular**: Ventas por categoría.  
- [x] **Tabla**: Performance vendedores con **Totales**.

---

## 11) Problemas comunes y solución
- **Descuento en texto / %** → usar `Descuento_num_pre` y `DescuentoSeguro`.  
- **KPI Cumplimiento vacío** → no hay **1 mes** seleccionado; activa **acción** desde la línea o agrega un **filtro de mes** (un solo valor).  
- **Top 10 inconsistente** → convierte filtros (año/mes/acciones) a **Contexto** en esa hoja.  
- **(En blanco)** en categoría o vendedor → claves huérfanas; usa `IFNULL` o corrige la dimensión.  
- **Meses faltantes** en la línea → usa fecha **continua** + *Mostrar valores faltantes* + `ZN()`.

---

## 12) Guardado y entrega
- **Archivo → Guardar como… → Libro empaquetado (*.twbx)**: `TechStore.twbx`.  
- Incluye este README en tu entrega, junto con el README de Power BI y los CSV (si tu profe los pide).

**Autor:** Nicole Ariadna Celemin Triana
