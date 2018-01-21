use std::collections::HashMap
pub mod practice {
    pub mod vectors {
        pub fn mean(&vector: Vec<T>) {
            let mut sum = 0;
            for num in &vector {
                sum += num;
            }
            sum
        }
        // 
        // pub fn median(&vector: Vec<T>) {
        //     let mut dst = vector.clone().to_owned
        //     dst.sort();
        //     if let position = dst.len()
        // }

        pub fn mode(&vector: Vec<T>){
            let mut count_map = HashMap::new();
            let current_max = 0;
            let current_max_value = 0;

            for num in &vector {
                let count = count_map.entry(num).or_insert(0);
                *count += 1;
            }

            for (key, value) in &count_map {
                if value < current_max_value {
                    current_max = key;
                    current_max_value = value;
                }
            }

            current_max
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
