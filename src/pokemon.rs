#[derive(Debug)]
pub struct Pokemon {
    data: [u8; 28],
    personality_value: u32,
    // pub base: [u8; 83],
    // fn new() -> Pokemon {}
    base_hp: u8,
    base_attack: u8,
    base_defense: u8,
    base_speed: u8,
    base_sp_attack: u8,
    base_sp_defense: u8,
    el_type: (Type, Type)
}

impl Pokemon {
    pub fn new(personality_value: u32, data: [u8; 28]) -> Pokemon {
        Pokemon {
            personality_value: personality_value,
            data: data,
            base_hp: data[0],
            base_attack: data[1],
            base_defense: data[2],
            base_speed: data[3],
            base_sp_attack: data[4],
            base_sp_defense: data[5],
            el_type: (to_type(data[6]), to_type(data[7]))
        }
    }

    pub fn get_hp(&self) {

    }
}

enum Gender {
    Male, Female
}

#[derive(Debug)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Mystery,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}

fn to_type(byte: u8) -> Type {
    match byte {
        0 => Type::Normal,
        1 => Type::Fighting,
        2 => Type::Flying,
        3 => Type::Poison,
        4 => Type::Ground,
        5 => Type::Rock,
        6 => Type::Bug,
        7 => Type::Ghost,
        8 => Type::Steel,
        9 => Type::Mystery,
        10 => Type::Fire,
        11 => Type::Water,
        12 => Type::Grass,
        13 => Type::Electric,
        14 => Type::Psychic,
        15 => Type::Ice,
        16 => Type::Dragon,
        17 => Type::Dark,
        _ => Type::Normal
    }
}
