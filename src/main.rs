
fn recurse_up(str: &String, depth: u8)
{
    // iterate over last character
    if depth == 0 {
        let prefix_last_char = str.split_at(str.len() - 1);
        
        for byte in (prefix_last_char.1.as_bytes()[0] - 'a' as u8)..26 {
            println!("{}{}",prefix_last_char.0, (byte + ('a' as u8) ) as u8 as char);
        }
    // iterate over initial characters and call recursion for the next character
    } else {
        recurse_up(str, depth - 1);

        let prefix_char = str.split_at(str.len() - depth as usize - 1);
        
        for byte in (prefix_char.1.as_bytes()[0] - 'a' as u8 + 1)..26 {
            let mut new_prefix = prefix_char.0.to_string().clone();
            new_prefix.push((byte + ('a' as u8) ) as u8 as char);
            for _ in 0..depth {
                new_prefix.push('a');
            }
            recurse_up(&new_prefix, depth - 1);
        }
    }
} 

fn recurse_down(str: &String, depth: u8, print_last: bool)
{
    // iterate over last character
    if depth == 0 {
        let prefix_last_char = str.split_at(str.len() - 1);
        
        for byte in 0..(prefix_last_char.1.as_bytes()[0] - 'a' as u8 + if print_last { 1 } else { 0 }) {
            println!("{}{}",prefix_last_char.0, (byte + ('a' as u8) ) as u8 as char);
        }
    // iterate over initial characters and call recursion for the next character
    } else {
        let prefix_char = str.split_at(str.len() - depth as usize);
        
        for byte in 0..(prefix_char.0.as_bytes().last().unwrap() - 'a' as u8) {
            let prefix_last_char = prefix_char.0.split_at(prefix_char.0.len() - 1);
            let mut new_prefix = String::new();
            new_prefix.push_str(prefix_last_char.0);
            new_prefix.push((byte + ('a' as u8) ) as u8 as char);
            
            for _ in 0..depth {
                new_prefix.push('z');
            }
            
            recurse_down(&new_prefix, depth - 1, true);
        }
        recurse_down(str, depth - 1, false);
    }
} 


fn main() {
    let mut line = String::new();
    let inputs: Vec<&str>;
    
    println!("Enter string :");
    std::io::stdin().read_line(&mut line).unwrap();
    
    let input = line.trim().clone();
    inputs = input.split_whitespace().collect();
    if inputs.len() < 2 { 
        println!("Input should consist of two strings. Ex: zx aac");
    } else {
        println!("You entered: {}, len: {}, {}, len: {}", inputs[0], inputs[0].len() as u8, inputs[1], inputs[1].len() as u8);
        recurse_up(&inputs[0].to_string(),inputs[0].len() as u8 - 1);
        recurse_down(&inputs[1].to_string(),inputs[1].len() as u8 - 1, false);
    }
}
