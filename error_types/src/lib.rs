use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name".to_string(),
                self.name.to_string(),
                format!("Username is empty"),
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                format!("Password should be at least 8 characters long"),
            ));
        }

        let has_alphabetic = self.password.chars().any(|c| c.is_alphabetic());
        let has_numeric = self.password.chars().any(|c| c.is_numeric());
        let has_non_alphanumeric = self.password.chars().any(|c| !c.is_alphanumeric());
        if !has_alphabetic || !has_non_alphanumeric || !has_numeric {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                format!("Password should be a combination of ASCII numbers, letters and symbols"),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut form_output = Form {
            name: "Lee".to_owned(),
            password: "qwqwsa1dty_".to_owned(),
        };

        println!("{:?}", form_output);
        println!("{:?}", form_output.validate());

        form_output.name = "".to_owned();
        println!("{:?}", form_output.validate());

        form_output.name = "as".to_owned();
        form_output.password = "dty_1".to_owned();
        println!("{:?}", form_output.validate());

        form_output.password = "asdasASd(_".to_owned();
        println!("{:?}", form_output.validate());

        form_output.password = "asdasASd123SA".to_owned();
        println!("{:?}", form_output.validate());
    }
}
