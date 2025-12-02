// Programa Mini-Motor de Búsqueda
// Importación de librerías externas y módulos estándar necesarios para el proyecto.
use anyhow::Result;
use eframe::{egui, App, Frame}; // `eframe` y `egui` para la interfaz gráfica.
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex}; // Para la gestión segura de estado concurrente.
use tantivy::collector::TopDocs; // `tantivy` es el motor de búsqueda principal.
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, TEXT, STORED, STRING};
use tantivy::{Index, Score, Term};
use egui::{FontId, RichText};

// Estructuras de datos
// Representa el o los resultados de búsqueda que se mostrarán en la UI.
#[derive(Clone)]
struct SearchResult {
    title: String,
    snippet: String,
    score: Score,
    doc_id: tantivy::DocAddress,
}

// Lógica del motor de búsqueda Tantivy.
// Encapsula el índice y la definición de los campos para evitar duplicados y fallas en el código
struct TantivyEngine {
    index: Index,
    title_field: tantivy::schema::Field,
    body_field: tantivy::schema::Field,
    id_field: tantivy::schema::Field,
}

impl TantivyEngine {
    // Inicializa el motor de búsqueda
    // Abre un índice existente en la carpeta o crea uno, en caso de que este no exista
    fn open_or_create(dir: &str) -> Result<Self> {
        let path = Path::new(dir);
        // El 'schema' define la estructura de los documentos que se van a indexar
        let mut schema_builder = Schema::builder();
    
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let body_field = schema_builder.add_text_field("body", TEXT | STORED);
        let id_field = schema_builder.add_text_field("id", STRING | STORED);

        let schema = schema_builder.build();

        // Crea o abre la carpeta del índice 
        let index = if path.exists() {
            Index::open_in_dir(path)?
        } else {
            std::fs::create_dir_all(path)?;
            Index::create_in_dir(path, schema)?
        };
        
        Ok(Self { index, title_field, body_field, id_field })
    }

    // Añade un nuevo documento al índice o actualiza uno existente
    // Utiliza 'id_val' para identificar de forma única el documento
    fn add_or_update_doc(&self, id_val: &str, title: &str, body: &str) -> Result<()> {
        let mut writer = self.index.writer(50_000_000)?;
        let id_term = Term::from_field_text(self.id_field, id_val);
        // Elimina el documento antiguo para evitar duplicados y luego añade la nueva versión
        writer.delete_term(id_term);
        writer.add_document(doc!(
            self.id_field => id_val.to_string(),
            self.title_field => title.to_string(),
            self.body_field => body.to_string()
        ))?;
        // Confirma los cambios en el índice
        writer.commit()?;
        println!("Commit realizado para el documento: {}", id_val);
        Ok(())
    }

    // Indexa todos los archivos .txt de una carpeta o directorio de forma masiva
    fn index_txt_dir(&self, dir: &str) -> Result<usize> {
        let mut writer = self.index.writer(50_000_000)?;
        let mut count = 0usize;
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return Ok(0),
        };
        
        // Itera sobre los archivos y los añade al índice
        for entry_res in entries {
            if let Ok(entry) = entry_res {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()).map_or(false, |ext| ext.eq_ignore_ascii_case("txt")) {
                    let id_val = path.file_name().and_then(|s| s.to_str()).unwrap_or_default().to_string();
                    let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
                    if let Ok(body) = fs::read_to_string(&path) {
                        let id_term = Term::from_field_text(self.id_field, &id_val);
                        writer.delete_term(id_term);
                        writer.add_document(doc!(
                            self.id_field => id_val.clone(),
                            self.title_field => title.clone(),
                            self.body_field => body.clone()
                        ))?;
                        count += 1;
                    }
                }
            }
        }
        
        // Se hace commit al final para mejorar la eficiencia de la indexación masiva
        writer.commit()?;
        println!("Commit masivo realizado para {} archivos.", count);
        Ok(count)
    }

    // Realiza una búsqueda en el índice según la consulta 'q' y devuelve los 'top_n'
    fn search(&self, q: &str, top_n: usize) -> Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        reader.reload()?; // Asegura que el usuario vea los últimos cambios
        let searcher = reader.searcher();

        // Parsea la consulta del usuario. Busca en los campos de título y cuerpo
        let parser = QueryParser::for_index(&self.index, vec![self.title_field, self.body_field]);
        let query = match parser.parse_query(q) {
            Ok(q) => q,
            Err(_) => return Ok(vec![]),
        };

        // Ejecuta la búsqueda
        let top_docs = searcher.search(&query, &TopDocs::with_limit(top_n))?;

        // Mapea los documentos encontrados a la estructura 'SearchResult' para la UI
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved = searcher.doc(doc_address)?;
            let title = retrieved.get_first(self.title_field).and_then(|v| v.as_text()).unwrap_or_default().to_string();
            let body = retrieved.get_first(self.body_field).and_then(|v| v.as_text()).unwrap_or_default().to_string();
            let snippet = body.chars().take(140).collect();
            results.push(SearchResult { title, snippet, score, doc_id: doc_address });
        }
        Ok(results)
    }
}

// Contiene el estado de la aplicación de la interfaz gráfica (UI)
struct MyApp {
    engine: Arc<Mutex<TantivyEngine>>,
    query: String,
    results: Vec<SearchResult>,
    new_title: String,
    new_body: String,
    docs_dir: String,
}

impl Default for MyApp {
    // Crea el estado inicial de la aplicación
    fn default() -> Self {
        let engine = TantivyEngine::open_or_create("tantivy_index").expect("No se pudo crear/abrir el índice");
        Self {
            engine: Arc::new(Mutex::new(engine)),
            query: String::new(),
            results: vec![],
            new_title: String::new(),
            new_body: String::new(),
            docs_dir: "docs".to_string(),
        }
    }
}

// Interfaz Gráfica
// Estilo visual personalizado a la aplicación egui
fn configure_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(28.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Button, FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, FontId::new(14.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Small, FontId::new(12.0, egui::FontFamily::Proportional)),
    ].into();

    let mut visuals = egui::Visuals::light();
    
    // Paleta de colores 
    visuals.override_text_color = Some(egui::Color32::from_rgb(40, 40, 60));
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(243, 240, 255); 
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 210, 240));

    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 220, 255); 
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 60));

    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(255, 215, 230);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 195, 215); 
    
    visuals.window_rounding = egui::Rounding::same(6.0);
    visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    visuals.widgets.active.rounding = egui::Rounding::same(4.0);
    
    ctx.set_visuals(visuals);
    ctx.set_style(style);
}

// Implementación de la App con la interfaz gráfica
impl App for MyApp {
    // Crea la interfaz gráfica y gestiona los eventos del usuario en cada frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Encabezado
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Googlecole");
                ui.add_space(5.0);
                ui.label("Una herramienta para indexar y buscar en tus documentos locales.");
                ui.add_space(10.0);
            });
            ui.separator();

            // Barra de búsqueda
            egui::Frame::none().inner_margin(egui::Margin::symmetric(20.0, 10.0)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let search_box = egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Ingresa tu búsqueda...")
                        .desired_width(f32::INFINITY);
                    ui.add(search_box);

                    if ui.add_sized([80., 30.], egui::Button::new("🔍 Buscar")).clicked() {
                        let eng = self.engine.lock().unwrap();
                        match eng.search(&self.query, 20) {
                            Ok(r) => self.results = r,
                            Err(e) => eprintln!("Error buscando: {}", e),
                        }
                    }
                });
            });

            // Acciones desplegables
            egui::Frame::none().inner_margin(egui::Margin::symmetric(20.0, 10.0)).show(ui, |ui| {
                ui.collapsing("📝 Crear y guardar nuevo .txt", |ui| {
                    // UI para crear y guardar un nuevo documento
                    egui::Frame::none().inner_margin(egui::Margin::symmetric(15.0, 10.0)).show(ui, |ui| {
                        let title_widget = egui::TextEdit::singleline(&mut self.new_title).hint_text("Título del archivo");
                        ui.add(title_widget);
                        ui.add_space(5.0);
                        
                        ui.add_sized(
                            [ui.available_width(), 100.0],
                            egui::TextEdit::multiline(&mut self.new_body)
                                .hint_text("Contenido del archivo...")
                        );

                        ui.add_space(10.0);
                        
                        if ui.add_sized([ui.available_width(), 30.], egui::Button::new("💾 Guardar e Indexar")).clicked() {
                            if !self.new_title.is_empty() {
                                let filename = format!("{}.txt", self.new_title.trim());
                                let file_path = Path::new(&self.docs_dir).join(&filename);
                                fs::create_dir_all(&self.docs_dir).ok();

                                if fs::write(&file_path, &self.new_body).is_ok() {
                                    let eng = self.engine.lock().unwrap();
                                    if let Err(e) = eng.add_or_update_doc(&filename, &self.new_title, &self.new_body) {
                                        eprintln!("Error al guardar/indexar documento: {}", e);
                                    } else {
                                        println!("Operación de guardado e indexado completada para '{}'.", filename);
                                        self.new_title.clear();
                                        self.new_body.clear();
                                    }
                                }
                            }
                        }
                    });
                });

                ui.collapsing("📂 Indexar carpeta de .txt", |ui| {
                    // UI para indexar una carpeta completa
                    egui::Frame::none().inner_margin(egui::Margin::symmetric(15.0, 10.0)).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Carpeta de .txt:");
                            ui.text_edit_singleline(&mut self.docs_dir);
                        });
                        ui.add_space(10.0);
                        if ui.add_sized([ui.available_width(), 30.], egui::Button::new("🔄 Indexar archivos .txt")).clicked() {
                            let eng = self.engine.lock().unwrap();
                            if let Ok(count) = eng.index_txt_dir(&self.docs_dir) {
                                println!("Se indexaron/actualizaron {} archivos.", count);
                            }
                        }
                    });
                });
            });

            ui.separator();

            // Resultados de búsqueda
            egui::Frame::none().inner_margin(egui::Margin::symmetric(20.0, 10.0)).show(ui, |ui| {
                ui.label(RichText::new(format!("Resultados encontrados: {}", self.results.len())).text_style(egui::TextStyle::Body));
                ui.add_space(10.0);

                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    for r in &self.results {
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.label(RichText::new(&r.title).text_style(egui::TextStyle::Monospace).strong());                        
                            ui.label(format!("Puntuación: {:.2}", r.score));                           
                            ui.add_space(5.0);
                            ui.label(&r.snippet);
                        });
                        ui.add_space(5.0);
                    }
                });
            });
        });
    }
}

// Función main para ejecutar la aplicación
fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        follow_system_theme: false, 
        default_theme: eframe::Theme::Light, // Tema claro
        ..Default::default()
    };
    eframe::run_native(
        "Mini Motor de Búsqueda",
        native_options,
        Box::new(|cc| {
            // Aplica los estilos personalizados al iniciar la app
            configure_styles(&cc.egui_ctx);
            // Crea e inicializa el estado de la app
            Box::new(MyApp::default())
        }),
    )
}