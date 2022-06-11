use num::complex::Complex;

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, Hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region)
    }
}

fn main() {
    greet_world();
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65,
    Fiordland penguin, 60
    Invalid, data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明是一个fields变量，类型是Vec
        // Vec是vector的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示Vec中的元素类型由编译器自行推断
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.2, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
