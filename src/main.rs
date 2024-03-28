fn main() {
    let letter_keys = [
        ['a', 'z'],
        ['b', 'y'],
        ['c', 'x'],
        ['d', 'w'],
        ['e', 'v'],
        ['f', 'u'],
        ['g', 't'],
        ['h', 's'],
        ['i', 'r'],
        ['j', 'q'],
        ['k', 'p'],
        ['l', 'o'],
        ['m', 'n'],
        ['n', 'm'],
        ['o', 'l'],
        ['p', 'k'],
        ['q', 'j'],
        ['r', 'i'],
        ['s', 'h'],
        ['t', 'g'],
        ['u', 'f'],
        ['v', 'e'],
        ['w', 'd'],
        ['x', 'c'],
        ['y', 'b'],
        ['z', 'a'],
    ];
    let message = "abcde";
    let result = encrypt(message, letter_keys);
    println!("{}", result);
}

// Encrypts the given text using the given letter keys.
fn encrypt(text: &str, letter_keys: [[char; 2]; 26]) -> String {
    let mut result = String::new();
    for character in text.chars() {
        let mut found = false;
        for key in letter_keys.iter() {
            if character == key[0] {
                result.push(key[1]);
                found = true;
                break;
            }
        }
        if !found {
            result.push(character);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let letter_keys = [
            ['a', 'z'],
            ['b', 'y'],
            ['c', 'x'],
            ['d', 'w'],
            ['e', 'v'],
            ['f', 'u'],
            ['g', 't'],
            ['h', 's'],
            ['i', 'r'],
            ['j', 'q'],
            ['k', 'p'],
            ['l', 'o'],
            ['m', 'n'],
            ['n', 'm'],
            ['o', 'l'],
            ['p', 'k'],
            ['q', 'j'],
            ['r', 'i'],
            ['s', 'h'],
            ['t', 'g'],
            ['u', 'f'],
            ['v', 'e'],
            ['w', 'd'],
            ['x', 'c'],
            ['y', 'b'],
            ['z', 'a'],
        ];

        // Test case 1: Encrypting "abcde" should return "zyxwv"
        assert_eq!(encrypt("abcde", letter_keys), "zyxwv");

        // Test case 2: Encrypting "hello" should return "svool"
        assert_eq!(encrypt("hello", letter_keys), "svool");

        // Test case 3: Encrypting "xyz" should return "cba"
        assert_eq!(encrypt("xyz", letter_keys), "cba");

        // Test case 4: Encrypting "rust" should return "ifhg"
        assert_eq!(encrypt("rust", letter_keys), "ifhg");

        // Test case 5: Encrypting "123" should return "123" (no changes)
        assert_eq!(encrypt("123", letter_keys), "123");
    }
}
