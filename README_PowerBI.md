# README — Laboratorio BI (Power BI) · TechStore

## 1) Objetivo
Crear en **Power BI** un dashboard para el dataset **TechStore** que cumpla **exactamente** con los requisitos del laboratorio:
- **KPIs destacados (3)**: *Ventas*, *Cumplimiento de meta*, *Ticket Promedio*.
- **Gráficos**: 
  - **Líneas**: Ventas por mes
  - **Barras**: Top 10 productos
  - **Circular**: Ventas por categoría
  - **Tabla**: Performance vendedores
- **Técnico (Power BI)**: 
  - **2 medidas DAX** (mínimo) → `Ventas`, `Cumplimiento %` (se añadió `Ticket Promedio` como 3er KPI)
  - **1 filtro de fecha** → Slicer Año→Mes (selección única)
  - **Colores corporativos** → Tema **TechStore Theme (Minimal)**

---

## 2) Datos de entrada
Archivos CSV utilizados:
- `ventas_techstore.csv` (hechos): `fecha`, `producto_id`, `cliente_id`, `cantidad`, `precio_unitario`, `descuento`, `canal_venta`, `vendedor`
- `productos.csv` (dimensión): `producto_id`, `nombre_producto`, `categoria`, `marca`, `precio_costo`, `precio_venta`, `stock_actual`
- `clientes.csv` (dimensión): `cliente_id`, `nombre_cliente`, `email`, `telefono`, `ciudad`, `fecha_registro`, `segmento`
- `metas_mensuales.csv` (metas): `año`, `mes`, `meta_ventas`, `meta_unidades`, `meta_nuevos_clientes`

**Tipos clave:**
- `ventas_techstore[fecha]` → **Date**
- `metas_mensuales[año]`, `[mes]` → **Whole Number**

---

## 3) Modelo de datos (relaciones)
- `ventas_techstore[producto_id]` → `productos[producto_id]` (Many-to-One)
- `ventas_techstore[cliente_id]` → `clientes[cliente_id]` (Many-to-One)
- `ventas_techstore[fecha]` → `Calendario[Date]` (Many-to-One)
- `Calendario[AñoMes]` ↔ `metas_mensuales[AñoMes]` (1:1)

**Tabla de calendario (fija 2024 para este lab):**
```DAX
Calendario = CALENDAR ( DATE(2024,1,1), DATE(2024,12,31) )

Año    = YEAR ( Calendario[Date] )
MesNum = MONTH ( Calendario[Date] )
Mes    = FORMAT ( Calendario[Date], "MMM" )
AñoMes = Calendario[Año]*100 + Calendario[MesNum]
```

**Columna en metas para enlazar:**
```DAX
AñoMes = metas_mensuales[año]*100 + metas_mensuales[mes]
```

> Nota: si a futuro se agregan otros años, sustituir por un **Calendario dinámico** (en este lab no es requerido).

---

## 4) Medidas DAX (KPIs)
**Ubicación:** tabla **Medidas**.  
**Formato:** `Ventas` → Moneda; `Cumplimiento %` → Porcentaje; `Ticket Promedio` → Moneda.

### 4.1 Ventas (robusta frente a descuentos 5/10/15 vs 0.05/0.10/0.15)
```DAX
Ventas =
SUMX (
    ventas_techstore,
    VAR dRaw = COALESCE( ventas_techstore[descuento], 0 )
    VAR d    = IF( dRaw > 1, dRaw / 100.0, dRaw )
    RETURN
        ventas_techstore[cantidad] *
        ventas_techstore[precio_unitario] *
        ( 1 - d )
)
```

### 4.2 Cumplimiento % (según el mes seleccionado)
```DAX
Cumplimiento % =
VAR _anyomes = SELECTEDVALUE ( Calendario[AñoMes] )
VAR _meta =
    CALCULATE (
        SUM ( metas_mensuales[meta_ventas] ),
        metas_mensuales[AñoMes] = _anyomes
    )
RETURN
    DIVIDE ( [Ventas], _meta )
```

> Para que la tarjeta muestre siempre valor, el slicer debe dejar **un único mes** seleccionado.

### 4.3 Ticket Promedio (3er KPI)
```DAX
Ticket Promedio =
DIVIDE( [Ventas], SUM( ventas_techstore[cantidad] ) )
```

---

## 5) Visualizaciones
### 5.1 KPI Cards (encabezado)
- **Ventas (COP)** → medida `[Ventas]` (Moneda)
- **Cumplimiento de meta (%)** → medida `[Cumplimiento %]` (Porcentaje)
- **Ticket Promedio (COP)** → medida `[Ticket Promedio]` (Moneda)

### 5.2 Gráfico de líneas — Ventas por mes
- **Axis**: `Calendario[Date]` (granularidad **Month** o `Mes` ordenado por `MesNum`)
- **Values**: `[Ventas]`
- **Tipo de eje**: Categórico para ver “Ene, Feb, …”

### 5.3 Barras — Top 10 productos
- **Axis**: `productos[nombre_producto]`
- **Values**: `[Ventas]`
- **Orden**: Desc por Ventas
- **Filtro del visual** → **Top N = 10** por `[Ventas]`

### 5.4 Circular — Ventas por categoría
- **Legend**: `productos[categoria]`
- **Values**: `[Ventas]`
- Etiquetas: % y/o valor

### 5.5 Tabla — Performance vendedores
Columnas (con agregaciones):
- `vendedor`
- `[Ventas]` (medida)
- `cantidad` → **Suma** (Unidades)
- `producto_id` → **Recuento** (Pedidos)
- `descuento` → **Promedio** (formato %)

Orden por **Ventas** desc. Totales activados.

---

## 6) Filtro de fecha (requisito técnico)
- **Un solo slicer** con `Calendario[Año]` y `Calendario[Mes]` (jerárquico).  
- **Single select = On** (un único mes) para que `[Cumplimiento %]` tenga un `AñoMes` único.  
- Título del slicer: “Selecciona Año y Mes”.

---

## 7) Colores corporativos (requisito técnico)
Tema **TechStore Theme (Minimal)** aplicado: `dataColors`, `background`, `foreground`, `tableAccent`.  
Archivo de ejemplo: `techstore_theme_min.json` (sin `visualStyles` para evitar validación).

Paleta:
- `#1F6FEB` (azul primario)
- `#FE7A00` (naranja)
- `#10B981` (verde)
- `#7C3AED` (morado)
- `#F59E0B` (ámbar)
- `#6B7280` (gris pizarra)

---

## 8) Validación con datos de muestra
Con los CSV proporcionados (enero–febrero 2024):
- **Enero 2024** → Ventas ≈ **25,667,600**; Cumplimiento ≈ **57.04%**
- **Febrero 2024** → Ventas ≈ **17,670,400**; Cumplimiento ≈ **42.07%**

> Si el slicer tiene más de un mes, la tarjeta de **Cumplimiento** puede mostrar **(Blank)** (por `SELECTEDVALUE`).

---

## 9) Checklist de requisitos (Power BI)
- [x] **KPIs destacados** (al menos 3): *Ventas*, *Cumplimiento %*, *Ticket Promedio*
- [x] **Líneas**: Ventas por mes
- [x] **Barras**: Top 10 productos
- [x] **Circular**: Ventas por categoría
- [x] **Tabla**: Performance vendedores
- [x] **2 medidas DAX** (mínimo): `Ventas`, `Cumplimiento %`
- [x] **1 filtro de fecha**: Slicer Año→Mes (selección única)
- [x] **Colores corporativos**: Tema TechStore aplicado

---

## 10) Uso y mantenimiento
- **Actualizar datos**: **Inicio → Actualizar** (asegura rutas a CSV).  
- **Agregar meses/años**: si excede 2024, cambiar a **Calendario dinámico**.  
- **Evitar negativos**: medida `Ventas` ya normaliza descuentos >1 → divide entre 100.  
- **Rendimiento**: limitar Top N y desactivar interacciones innecesarias si el dataset crece.

---

## 11) Problemas comunes
- **Meses desordenados** → `Mes` ordenado por `MesNum` (Modelado → Ordenar por columna).  
- **(Blank) en Cumplimiento** → el slicer tiene varios meses; usar selección **única**.  
- **Slice (Blank) en categoría/producto** → claves huérfanas; revisar integridad `producto_id`.  
- **Top 10 no aplica** → configurarlo en **Filtros del visual** (no de la página).

---

## 12) Créditos
- Autor: Nicole Ariadna Celemin Triana (ncelemin@unal.edu.co) 
