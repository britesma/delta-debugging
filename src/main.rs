use regex::Regex;
use std::fs;

fn split_tests(tests: &str, n: usize) -> Vec<String> {
    let mut splitted_tests: Vec<String> = Vec::new();
    let mut start: usize = 0;

    for _i in 0..n {
        let end: usize = start + tests.len() / n;
        let sub_test: String = String::from(&tests[start..end]);
        splitted_tests.push(sub_test);
        start += tests.len() / n;
    }
    return splitted_tests;
}

fn get_nabla(data: &str, delta: &str) -> String {
    let mut nabla: String = String::new();

    for c in data.chars() {
        if !delta.contains(c) {
            nabla.push(c);
        }
    }
    return nabla;
}

fn check(test: &str) -> bool {
    let re = Regex::new(r"<SELECT(.*?)>").unwrap();
    return re.is_match(&test);
}

fn find_one_minimal(test: &str) -> String {
    let mut tmp_str = String::from(test);
    for i in 0..tmp_str.len() {
        let mut truncated_str: String = String::from(&tmp_str);
        truncated_str.remove(i);
        if check(&truncated_str) {
            tmp_str = truncated_str;
            return find_one_minimal(&tmp_str);
        }
    }

    return tmp_str;
}
fn main() {
    let mut data: String = fs::read_to_string("./resources/htmlPage.txt").unwrap();
    let mut granularity: usize = 2;

    while granularity <= data.len() {
        let tests: Vec<String> = split_tests(&data, granularity);

        for delta in tests {
            let nabla: String = get_nabla(&data, &delta);

            //check delta
            if check(&delta) {
                data = delta;
                break;
            } else if check(&nabla) {
                data = nabla;
                granularity = granularity - 1;
                break;
            } else {
                granularity = granularity * 2;
                break;
            }
        }
    }

    let one_minimal: String = find_one_minimal(&data);
    println!("{}", one_minimal);
}
