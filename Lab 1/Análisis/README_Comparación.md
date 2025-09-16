# README — Comparativo rápido: Power BI vs Tableau (TechStore)

## Resumen
Para este laboratorio (3 KPIs, 4 visuales, metas mensuales y colores corporativos), **Power BI** resulta **ligeramente mejor** para principiantes y para una entrega reproducible: requiere **menos pasos**, permite aplicar **tema corporativo** en 1 archivo y el **slicer Año→Mes** hace que el KPI de **Cumplimiento** funcione sin lógica extra.  
**Tableau** brilla en **interactividad** y exploración con **acciones de filtro** muy naturales, pero en este caso demandó más ajustes (limpieza de descuento, parámetro + filtro de año y acción para asegurar “1 mes”).

## Criterios y hallazgos
- **Velocidad de construcción**
  - **Power BI**: + Rápido para KPIs/Top 10; + Tema JSON global.
  - **Tableau**: + Visual; − Necesita parámetro de año y acciones para el KPI de Cumplimiento.
- **Modelo y calidad de datos**
  - **Power BI**: + Tabla Calendario + `AñoMes` para metas; DAX claro (`SELECTEDVALUE`).
  - **Tableau**: + Relationships evitan duplicados; − Limpieza de `descuento` si viene como texto/%.
- **Interactividad**
  - **Tableau**: + Acciones entre hojas muy potentes.
  - **Power BI**: interacciones por defecto suficientes para el lab.
- **Colores corporativos**
  - **Power BI**: + Tema `.json` centralizado.
  - **Tableau**: paleta vía `Preferences.tps` (paso extra).
- **Entrega**
  - **Power BI (.pbix)**: medidas/tema centralizados, fácil de replicar.
  - **Tableau (.twbx)**: portable, pero más dependiente de acciones/hojas.

## Veredicto
- **Para este lab**: **Power BI** — por **simplicidad**, **tema corporativo** inmediato y **KPI de Cumplimiento** sin acciones.
- **Cuándo elegir Tableau**: cuando el foco sea **exploración ad‑hoc** y **filtros por interacción** más sofisticados.

> Ambos llegan al mismo resultado; la diferencia está en los **pasos**. Entrega ambos y explica por qué Power BI fue más directo y qué valor aporta Tableau en interactividad.
