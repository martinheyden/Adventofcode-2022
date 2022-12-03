pub mod read_input {
    use std::fs;

    pub fn read_file_to_string(file_name: &str) -> String {
        let input = fs::read_to_string(file_name);
        match input {
            Ok(s) => s,
            Err(_) => panic!("Couldnt read file"),
        }
    }

    pub fn read_file_to_int_vec(file_name: &str) -> Vec<i64> {
        read_file_to_string(file_name)
            .split("\n")
            .map(|i| i.parse::<i64>().unwrap())
            .collect()
    }

    pub fn read_line_to_int_vec(file_name: &str) -> Vec<i64> {
        let str = read_file_to_string(file_name);
        str.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_file_to_string_vec(file_name: &str) -> Vec<String> {
        read_file_to_string(file_name)
            .split("\n")
            .map(|i| i.to_string())
            .collect()
    }

    pub fn read_file_to_matrix_compact(file_name: &str) -> Vec<Vec<i64>> {
        let str_vec = read_file_to_string_vec(file_name);
        let y_len = str_vec.len();
        let mut data: Vec<Vec<i64>> = Vec::new();
        for y in 0..y_len {
            let data_vec = str_vec[y]
                .chars()
                .map(|i| i.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>();
            data.push(data_vec);
        }
        data
    }
}
