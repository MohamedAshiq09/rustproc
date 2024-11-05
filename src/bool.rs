fn main() {
    let para = String::from("my name is ashiq");
    let first_word = get_first_word(para);

    print!("{}",first_word)
}

fn get_first_word(para : String) -> String {
    let mut ans = String::from ("");
    for char in para.chars{
        ans.push_str(char.to_string().as_str());
        if char = ' ' {
          break;
        }
    } 
  return ans;
}