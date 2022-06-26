#[derive()]
pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruits: String,
}

#[derive()]
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    /// Creates a new [`Breakfast`].
    pub fn new(toast: String, seasonal_fruits: String) -> Self {
        Self {
            toast,
            seasonal_fruits,
        }
    }

    /// .
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruits: String::from("peaches"),
        }
    }

    /// Returns a mutable reference to the seasonal fruits of this [`Breakfast`].
    pub fn seasonal_fruits_mut(&mut self) -> &mut String {
        &mut self.seasonal_fruits
    }

    /// Returns a mutable reference to the toast of this [`Breakfast`].
    pub fn toast_mut(&mut self) -> &mut String {
        &mut self.toast
    }
}
