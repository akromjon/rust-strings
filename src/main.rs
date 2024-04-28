mod strings;

struct StringObject {
    string: String,
    second_string: String,
}

fn count(string: &StringObject,val: String) -> usize {
    
    if val=="string".to_string() {
    
        return string.string.len();     
    
    }

    return string.second_string.len();
}
fn main() {
    
    let my_str: String = "custom string".to_string();

    strings::custom_print(&my_str);
    strings::custom_print(&my_str);
    strings::custom_print(&my_str);

    let string: StringObject = StringObject {
        string: "first string".to_string(),
        second_string: "second string".to_string(),
    };

    println!("first string count: {}",count(&string,"string".to_string()));
    println!("second string count: {}",count(&string,"".to_string()));

}
