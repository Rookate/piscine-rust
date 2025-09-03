use json::JsonValue;
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut res = JsonValue::new_object();
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
    let round = 10_f64.powi(2);

    for food in foods {
        let x = food
            .calories
            .1
            .chars()
            .filter(|&c| c.is_numeric() || c == '.')
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        cals = cals + x * food.nbr_of_portions;
        carbs = carbs + (food.carbs * food.nbr_of_portions);
        fats = fats + (food.fats * food.nbr_of_portions);
        proteins = proteins + (food.proteins * food.nbr_of_portions);
    }
    res.insert("cals", (cals * round).round() / round).unwrap();
    res.insert("carbs", (carbs * round).round() / round)
        .unwrap();
    res.insert("proteins", (proteins * round).round() / round)
        .unwrap();
    res.insert("fats", (fats * round).round() / round).unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foods = [
            Food {
                name: "big mac".to_owned(),
                calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
                proteins: 27.,
                fats: 26.,
                carbs: 41.,
                nbr_of_portions: 2.,
            },
            Food {
                name: "pizza margherita".to_owned(),
                calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];

        println!("{:#}", calculate_macros(&foods));
    }
}
