pub fn build_proverb(items: Vec<&str>) -> String {
    let mut proverb_vec: Vec<String> = Vec::new();
    let number_of_items = items.len();

    let mut has_horse = false;

    if number_of_items <= 0 {
        return String::new();
    } else {
        for i in 0..(number_of_items - 1) {
            if items[i] == "horse" || items[i+1] == "horse" {
                has_horse = true;
            }

            proverb_vec.push(format!("For want of a {0} the {1} was lost.", items[i], items[i+1]).trim().to_string());
        }
    }

    if has_horse {
        proverb_vec.push("And all for the want of a horseshoe nail.".to_string());
    } else {
        proverb_vec.push("And all for the want of a nail.".to_string());
    }    

    proverb_vec.join("\n")
}