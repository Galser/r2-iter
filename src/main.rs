fn print_elements(elements:  &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}


fn main() {
    
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
    ];
    
    print_elements(&colors);

    // let mut colors_iter = colors.iter();
        
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
}
