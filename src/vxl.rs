use std::fs::File;
use std::io::Read;
use crate::map::Color;

pub fn load_map(filename: String, size: (u16, u16, u16)) -> Vec<Vec<Vec<Option<Color>>>> {
    let f = File::open(filename).unwrap();
    let map_file: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();

    let mut result : Vec<Vec<Vec<Option<Color>>>> = vec![vec![vec![None; size.0 as usize]; size.1 as usize]; size.2 as usize];

    let mut byte_position: u32 = 0;
    for x in 0..512 {
        for y in 0..512 {
            loop {
                let number_of_chunks = map_file[byte_position as usize];
                let top_color_start = map_file[(byte_position + 1) as usize];
                let top_color_end = map_file[(byte_position + 2) as usize];
                let length_of_bottom = top_color_end + 1 - top_color_start;
                let mut color_position = byte_position + 4;

                for z in top_color_start..top_color_end +1 {
                    let b = color_position;
                    color_position = color_position + 1;
                    let g = color_position;
                    color_position = color_position + 1;
                    let r = color_position;
                    color_position = color_position + 1;
                    result[x as usize][y as usize][z as usize] = Some(
                        Color {
                            r: map_file[r as usize],
                            g: map_file[g as usize],
                            b: map_file[b as usize],
                        }
                    );
                    color_position = color_position + 1
                }

                if number_of_chunks == 0 {
                    byte_position = byte_position + (4 * (length_of_bottom as u32 + 1));
                    break;
                }

                byte_position += number_of_chunks as u32 * 4;

                let length_of_top = (number_of_chunks - 1) - length_of_bottom;
                let bottom_color_end = map_file[(byte_position + 3) as usize];
                let bottom_color_start = bottom_color_end - length_of_top;

                for z in bottom_color_start..bottom_color_end {
                    let b = color_position;
                    color_position = color_position + 1;
                    let g = color_position;
                    color_position = color_position + 1;
                    let r = color_position;
                    color_position = color_position + 1;

                    result[x as usize][y as usize][z as usize] = Some(
                        Color {
                            r: map_file[r as usize],
                            g: map_file[g as usize],
                            b: map_file[b as usize],
                        }
                    );
                    color_position = color_position + 1;
                }
            }
        }
    }

    result
}
