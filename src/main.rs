use my_parser_test::list_parser;

pub fn main() {
    let l = list_parser::list("[1,1,2,3,5,8]");
    println!("var = {:?}", l);
    assert_eq!(l, Ok(vec![1, 1, 2, 3, 5, 8]));
}

