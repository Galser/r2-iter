use std::{result::Iter, string};

fn print_elements(elements:  &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
//        .map(|el| format!("{} {} ", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements:  &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
    ];
    
    print_elements(&colors[1..3]);

    shorten_strings(&mut colors);

    print_elements(&colors);
    // let mut colors_iter = colors.iter();
        
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
}
