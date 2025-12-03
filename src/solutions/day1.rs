use crate::helpers;

#[derive(Eq, PartialEq)]
enum RotationDirections {
    L,
    R,
    Null,
}

fn update_pointer(operation: RotationDirections, mut point: i32, rotation_value: i32) -> i32 {
    if operation == RotationDirections::R {
        point += rotation_value;
    } else {
        point -= rotation_value;
        if point < 0 {
            point += 100;
        };
    }

    point % 100
}

pub fn solve() {
    let contents = helpers::io::get_input("../inputs/day1");
    let mut point: i32 = 50;
    let mut zero_times: i32 = 0;

    for content in contents {
        let operation = match content.chars().nth(0) {
            None => break,
            Some(operation) => operation,
        };

        let rotation = match operation {
            'L' => RotationDirections::L,
            'R' => RotationDirections::R,
            _ => RotationDirections::Null,
        };

        if rotation == RotationDirections::Null {
            break;
        }

        // TODO: tratar corretamente (remover expect)
        let rotation_value: i32 = content[1..].parse().expect("value should exist");
        point = update_pointer(rotation, point, rotation_value);

        if point == 0 {
            zero_times += 1;
        }
    }

    println!("{zero_times}");
}
