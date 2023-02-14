use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::env;
use human_format::Formatter;
use human_format::Scales;

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator, IntEnum)]
#[repr(u32)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl std::str::FromStr for ResistorColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: &str = &s.clone().to_lowercase();
        match value {
            "black" => Ok(ResistorColor::Black),
            "brown" => Ok(ResistorColor::Brown),
            "red" => Ok(ResistorColor::Red),
            "orange" => Ok(ResistorColor::Orange),
            "yellow" => Ok(ResistorColor::Yellow),
            "green" => Ok(ResistorColor::Green),
            "blue" => Ok(ResistorColor::Blue),
            "violet" => Ok(ResistorColor::Violet),
            "grey" => Ok(ResistorColor::Grey),
            "white" => Ok(ResistorColor::White),
            _ => Err(format!("'{}' is not a valid value for ResistorColor", s)),
        }
    }
}

pub fn color_to_value(_color: &str) -> u32 {
    let value: ResistorColor = _color.parse().unwrap();
    value as u32
}
pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string()
    }
}
pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}

pub fn help() {
    println!("rescol is a calculator to translate resistor colors to value and vice-versa. \n Author: sandevins (sdevicente@gradiant.org)");
    println!("Please provide a number or three color names to get started.")
}

pub fn main () {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
        },

        2 => {
            let num = &args[1];
            let _number: f64 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: argument not an integer");
                    help();
                    return;
                }
            };
            println!("This still has to be implemented, sorry for the inconvenience.");
        },

        4 => {
            let first = color_to_value(&args[1]);
            let second = color_to_value(&args[2]);
            let third = color_to_value(&args[3]);
            let result = i64::from(first * 10 + second) * i64::from(10).pow(third);
            println!("The value of the resistance is: {}", Formatter::new().with_scales(Scales::SI()).format(result as f64));
        },

        _ => {
            help();
        }
    }

}

