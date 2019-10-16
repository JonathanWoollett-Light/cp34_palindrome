use std::env;
fn main() {
    let arguments:Vec<String> = env::args().collect();
    
    // Presumes non utf8 string
    let input:&[u8] = arguments[1].as_bytes();
    println!("{}",String::from_utf8(input.to_vec()).unwrap());

    let mut best_centre = 0usize;
    let mut largest_block = 0usize;

    //Finds best centre position with largest mirrored blocks of adjacent characters
    for char_index in 0..input.len() {
        //Max number of characters between this character and the end of the string
        let max_adjustment = 
        if char_index > input.len() - char_index {
            input.len() - char_index
        } else {
            char_index
        };

        //Iterators through characters adjacent to centre character to find how large a block of mirrored character there is
        for adjustment in 0..max_adjustment {
            let forward_char = input[char_index+adjustment];
            let backward_char = input[char_index-adjustment];

            if forward_char != backward_char { break; }

            if adjustment > largest_block {
                best_centre = char_index;
                largest_block = adjustment;
            }
        }
    }

    //Sets slices to insert at start and end
    let start = &input[0..best_centre-largest_block];
    let end = &input[best_centre+1usize+largest_block..input.len()];

    //Sets slices to insert after start and before end
    let mut start_reverse:Vec<u8> = start.to_vec();
    start_reverse.reverse();
    let mut end_reverse = end.to_vec();
    end_reverse.reverse();

    //Sets blocks of mirrored characters to assert adjacent to centre character
    let start_block = &input[best_centre-largest_block..best_centre];
    let end_block = &input[best_centre+1usize..best_centre+largest_block+1usize];

    //`.push(45u8)` is simply to highlight the blocks to more easily understand what the program is doing
    let mut palindrome:Vec<u8> = Vec::with_capacity(input.len()+start.len()+end.len());
    palindrome.extend_from_slice(start);
    palindrome.push(45u8);
    palindrome.append(&mut end_reverse);
    palindrome.push(45u8);
    palindrome.extend_from_slice(start_block);
    palindrome.push(45u8);
    palindrome.push(input[best_centre]);
    palindrome.push(45u8);
    palindrome.extend_from_slice(end_block);
    palindrome.push(45u8);
    palindrome.extend_from_slice(end);
    palindrome.push(45u8);
    palindrome.append(&mut start_reverse);

    //Outputs the vec converted to a String
    println!("{}",String::from_utf8(palindrome).unwrap());
}