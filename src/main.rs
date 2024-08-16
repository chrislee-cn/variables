fn main() {
    let x = 5;
    // x = 6;
    // println!("The value of x is: {x}");
    // const NUM_CLASS: u32 = 60 * 60 * 8;
    // println!("The value of x is: {NUM_CLASS}");
    let x = x + 1;
    println!("The value of x is: {x}");
    //作用域，概念明确
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let a: i32 = -1;
    // let b: f64 = -0.89;
    // let c: char = '我';
    //println!("{a},{b},{c}");

    //元组
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    //数组，元素类型必须相同
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
}
