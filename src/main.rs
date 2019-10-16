use std::env;
fn main() {
    let input:Vec<String> = env::args().collect();
    
    // Presumes non utf8 string
    let palindrome:&[u8] = input[1].as_bytes();
    println!("{}",String::from_utf8(palindrome.to_vec()).unwrap());

    let mut best_centre = 0usize;
    let mut largest_block = 0usize;

    //Finds best centre position with largest mirrror blocks of adjacent characters
    for char_index in 0..palindrome.len() {
        //Max number of characters between this character and end of string
        let max_adjustment = 
        if char_index > palindrome.len() - char_index {
            palindrome.len() - char_index
        } else {
            char_index
        };

        //Iterators through characters adjacent to centre character
        for adjustment in 0..max_adjustment {
            let forward_char = palindrome[char_index+adjustment];
            let backward_char = palindrome[char_index-adjustment];

            if forward_char != backward_char { break; }

            if adjustment > largest_block {
                best_centre = char_index;
                largest_block = adjustment;
            }
        }
    }

    let start = &palindrome[0..best_centre-largest_block];
    let end = &palindrome[best_centre+1usize+largest_block..palindrome.len()];

    let mut start_reverse:Vec<u8> = start.to_vec();
    start_reverse.reverse();
    let mut end_reverse = end.to_vec();
    end_reverse.reverse();

    let start_block = &palindrome[best_centre-largest_block..best_centre];
    let end_block = &palindrome[best_centre+1usize..best_centre+largest_block+1usize];

    let mut new_palindrome:Vec<u8> = Vec::with_capacity(palindrome.len()+start.len()+end.len());
    new_palindrome.extend_from_slice(start);
    new_palindrome.push(45u8);
    new_palindrome.append(&mut end_reverse);
    new_palindrome.push(45u8);
    new_palindrome.extend_from_slice(start_block);
    new_palindrome.push(45u8);
    new_palindrome.push(palindrome[best_centre]);
    new_palindrome.push(45u8);
    new_palindrome.extend_from_slice(end_block);
    new_palindrome.push(45u8);
    new_palindrome.extend_from_slice(end);
    new_palindrome.push(45u8);
    new_palindrome.append(&mut start_reverse);

    println!("{}",String::from_utf8(new_palindrome).unwrap());
}