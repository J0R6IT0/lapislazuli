use std::rc::Rc;

#[derive(Clone)]
pub struct ProgressContext {
    pub(super) value: f32,
    pub(super) max_value: f32,
    pub(super) min_value: f32,
    pub(super) value_label: Option<Rc<Box<dyn Fn(&ProgressContext) -> String>>>,
}

impl ProgressContext {
    pub fn percentage(&self) -> f32 {
        if self.max_value > self.min_value {
            ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn percentage_of(&self, value: f32) -> f32 {
        if self.max_value > self.min_value {
            ((value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn string_percentage(&self) -> String {
        format!("{:.2}%", self.percentage() * 100.0)
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn min_value(&self) -> f32 {
        self.min_value
    }

    pub fn max_value(&self) -> f32 {
        self.max_value
    }

    pub fn value_label(&self) -> String {
        if let Some(label_fn) = &self.value_label {
            label_fn(self)
        } else {
            self.string_percentage()
        }
    }
}
