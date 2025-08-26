#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: (alias.to_owned()),
            brightness: (0),
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    if let Some(l) = lights.iter_mut().find(|l| alias == l.alias) {
        l.brightness = value;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);

        println!("brightness = {}", lights[0].brightness);

        change_brightness(&mut lights, "living_room", 200);

        println!("new brightness = {}", lights[0].brightness);
    }
}
