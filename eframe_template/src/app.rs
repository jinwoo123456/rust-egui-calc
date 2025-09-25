use ::evalexpr::Value;
use ::evalexpr::eval;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct TemplateApp {
    #[serde(skip)] 
     calc: CalcValues,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CalcValues {
    value1: String,
    value2: String,
    value3: String,
    value4: String,
    value5: String,
    value6: String,
    value7: String,
    value8: String,
    value9: String,
    value0: String,
    valuePlus: String,
    valueMinus: String,
    valueMulti: String,
    valueDiv: String,
    valueEqual: String,
    valueAC: String,
    valuePercent: String,
    valueDot: String,
    valueLeftParen: String,
    valueRightParen: String,
    result: String,
}
trait CalcTrait {
    fn doCalc(&mut self) -> String;
    fn getResult(&mut self) -> Value;
}
impl Default for CalcValues {
    fn default() -> Self {
        Self {
            value1: "1".to_owned(),
            value2: "2".to_owned(),
            value3: "3".to_owned(),
            value4: "4".to_owned(),
            value5: "5".to_owned(),
            value6: "6".to_owned(),
            value7: "7".to_owned(),
            value8: "8".to_owned(),
            value9: "9".to_owned(),
            value0: "0".to_owned(),
            valuePlus: "+".to_owned(),
            valueMinus: "-".to_owned(),
            valueMulti: "*".to_owned(),
            valueDiv: "/".to_owned(),
            valueEqual: "=".to_owned(),
            valueAC: "AC".to_owned(),
            valuePercent: "%".to_owned(),
            valueDot: ".".to_owned(),
            valueLeftParen: "(".to_owned(),
            valueRightParen: ")".to_owned(),
            result: "".to_owned(),
        }
    }
}
impl CalcTrait for CalcValues {
    fn doCalc(&mut self) -> String {
        String::from("0")
    }
    fn getResult(&mut self) -> Value {
        Value::Empty
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            calc: CalcValues::default(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("egui 김진우의 계산기");

            ui.separator();
            // let mut calc = CalcValues::default();
            ui.vertical(|ui| {
                ui.label("계산기 만들기 2");
                ui.label(&self.calc.result);
            });

            ui.horizontal(|ui| {
                let valueLeftParen = ui.button(&self.calc.valueLeftParen);
                let valueRightParen = ui.button(&self.calc.valueRightParen);
                let valuePercent = ui.button(&self.calc.valuePercent);
                let valueAC = ui.button(&self.calc.valueAC);
                if valueLeftParen.clicked() {
                    &self.calc.result.push_str(&self.calc.valueLeftParen);
                }
                if valueRightParen.clicked() {
                    &self.calc.result.push_str(&self.calc.valueRightParen);
                }
                if valuePercent.clicked() {
                    &self.calc.result.push_str(&self.calc.valuePercent);
                }
                if valueAC.clicked() {
                    &self.calc.result.clear();
                }
            });
            ui.horizontal(|ui| {
                let value7 = ui.button(&self.calc.value7);
                let value8 = ui.button(&self.calc.value8);
                let value9 = ui.button(&self.calc.value9);
                let valueDiv = ui.button(&self.calc.valueDiv);
                if value7.clicked() {
                    &self.calc.result.push_str(&self.calc.value7);
                }
                if value8.clicked() {
                    &self.calc.result.push_str(&self.calc.value8);
                }
                if value9.clicked() {
                    &self.calc.result.push_str(&self.calc.value9);
                }
                if valueDiv.clicked() {
                    &self.calc.result.push_str(&self.calc.valueDiv);
                }
            });
            ui.horizontal(|ui| {
                let value4 = ui.button(&self.calc.value4);
                let value5 = ui.button(&self.calc.value5);
                let value6 = ui.button(&self.calc.value6);
                let valueMulti = ui.button(&self.calc.valueMulti);
                if value4.clicked() {
                    &self.calc.result.push_str(&self.calc.value4);
                }
                if value5.clicked() {
                    &self.calc.result.push_str(&self.calc.value5);
                }
                if value6.clicked() {
                    &self.calc.result.push_str(&self.calc.value6);
                }
                if valueMulti.clicked() {
                    &self.calc.result.push_str(&self.calc.valueMulti);
                }
            });
            ui.horizontal(|ui| {
                let value3 = ui.button(&self.calc.value3);
                let value2 = ui.button(&self.calc.value2);
                let value1 = ui.button(&self.calc.value1);
                let valueMinus = ui.button(&self.calc.valueMinus);
                if value3.clicked() {
                    &self.calc.result.push_str(&self.calc.value3);
                    //println!("calc.result: {}", &self.calc.result);
                }
                if value2.clicked() {
                    &self.calc.result.push_str(&self.calc.value2);
                    //println!("calc.result: {}", &self.calc.result);
                }
                if value1.clicked() {
                    &self.calc.result.push_str(&self.calc.value1);
                }
                if valueMinus.clicked() {
                    &self.calc.result.push_str(&self.calc.valueMinus);
                }
            });
            ui.horizontal(|ui| {
                let value0 = ui.button(&self.calc.value0);
                let valueDot = ui.button(&self.calc.valueDot);
                let valueEqual = ui.button(&self.calc.valueEqual);
                let valuePlus = ui.button(&self.calc.valuePlus);

                if value0.clicked() {
                    &self.calc.result.push_str(&self.calc.value0);
                    //println!("calc.result: {}", &self.calc.result);
                }
                if valueDot.clicked() {
                    &self.calc.result.push_str(&self.calc.valueDot);
                    //println!("calc.result: {}", &self.calc.result);
                }
                if valuePlus.clicked() {
                    &self.calc.result.push_str(&self.calc.valuePlus);
                    //println!("calc.result: {}", &self.calc.result);
                }
                if valueEqual.clicked() {
                    self.calc.result = match eval(&self.calc.result){
                        Ok(value) => value.to_string(),
                        Err(_) => "".into(),
                    }
                }
            });
             ui.separator();
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
