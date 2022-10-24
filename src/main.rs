struct RGB(u8, u8, u8);
struct Hex(String, String, String);

fn main() {
   let rgb_one = RGB(104, 30, 69);
   
   println!("RGB - Red: {}, Green: {}, Blue: {}", rgb_one.0, rgb_one.1, rgb_one.2);

   let hex_one = convert_to_hex(rgb_one);
   
   println!("HEX - #{}{}{}", hex_one.0, hex_one.1, hex_one.2);
}

fn convert_to_hex(rgb: RGB) -> Hex {
   let red_group = calculate_group(rgb.0);
   let green_group = calculate_group(rgb.1);
   let blue_group = calculate_group(rgb.2);
   let full_hex = Hex(red_group, green_group, blue_group);
   full_hex
}

fn calculate_group(color_value: u8) -> String {
    let first_number: u8 = color_value / 16;
    let second_number: u8 = color_value - (16 * first_number);

    let first_number: &str = match first_number {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "A",
        11 => "B",
        12 => "C",
        13 => "D",
        14 => "E",
        15 => "F",
        _ => "0"
    };

    let second_number: &str = match second_number {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "A",
        11 => "B",
        12 => "C",
        13 => "D",
        14 => "E",
        15 => "F",
        _ => "0"
    };

    let mut color_group: String = String::new();
    color_group += first_number;
    color_group += second_number;
    color_group
}
