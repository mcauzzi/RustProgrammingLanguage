enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}
fn main() {
    let mut v: Vec<i32> =Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut v_with_macro=vec![1,2,3];
    v_with_macro.push(4);
    let third=&v_with_macro[2];
    println!("The third element is {third}");

    let third=v_with_macro.get(2);
    match third {
        Some(third)=> println!("THe third element is {third}"),
        None=>println!("There is no third element")
    }
    for val in &mut v{
        *val+=50;
    }
    for val in &v{
        println!("Value is {val}");
    }

    let row=vec![SpreadSheetCell::Int(64), SpreadSheetCell::Float(0.2),SpreadSheetCell::Text(String::from("Text"))];
    for cell in row{
        match cell{
            SpreadSheetCell::Float(f)=>println!("Value is {f}"),
            SpreadSheetCell::Int(i)=>println!("Value is {i}"),
            SpreadSheetCell::Text(str)=>println!("Value is {str}")
        };
    }
}
