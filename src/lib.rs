pub mod geom;

use std::fs;

pub fn read_vector_of_string(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("File not found: {}", path))
        .split("\n")
        .map(str::trim)
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn read_vector_of_i32(path: &str) -> Vec<i32> {
    read_vector_of_string(path).iter()
        .map(|s| to_i32(s))
        .collect()
}

pub fn read_matrix_of_string(path: &str, separator: &str) -> Vec<Vec<String>> {
    read_vector_of_string(path).iter()
        .map(|s| s
            .split(separator)
            .map(str::trim)
            .map(String::from)
            .collect::<Vec<String>>())
        .collect()
}

pub fn read_matrix_of_i32(path: &str, separator: &str) -> Vec<Vec<i32>> {
    to_matrix_of_i32(&read_matrix_of_string(path, separator))
}

/// Reads a file of grouped strings, separated by blank lines.
pub fn read_blank_separated_matrix_of_string(path: &str) -> Vec<Vec<String>> {
    let lines: Vec<String> = fs::read_to_string(path)
        .expect(&*format!("File not found: {}", path))
        .split("\n")
        .map(str::trim)
        .map(String::from)
        .collect();

    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() && !group.is_empty() {
            matrix.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }

    matrix
}

/// Reads a file of grouped integers, separated by blank lines.
pub fn read_blank_separated_matrix_of_i32(path: &str) -> Vec<Vec<i32>> {
    to_matrix_of_i32(&read_blank_separated_matrix_of_string(path))
}

/// Sums each row in the matrix, producing a vector of sums.
pub fn sum_rows(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    matrix.iter()
        .map(|row| row.iter().sum())
        .collect()
}

/// Converts a matrix of string to a matrix of type i32.
pub fn to_matrix_of_i32(matrix: &Vec<Vec<String>>) -> Vec<Vec<i32>> {
    matrix.iter().map(to_vector_of_i32).collect()
}

/// Converts a vector of string to a vector of type i32.
pub fn to_vector_of_i32(vec: &Vec<String>) -> Vec<i32> {
    vec.iter().map(|s| to_i32(s)).collect::<Vec<i32>>()
}

pub fn to_i32(s: &str) -> i32 {
    s.trim().parse().expect(&*format!("Not an integer: {}", s))
}

pub fn to_i64(s: &str) -> i64 {
    s.trim().parse().expect(&*format!("Not an integer: {}", s))
}

/// Converts a vector of String to a vector of &str.
pub fn to_vector_of_str(vec: &Vec<String>) -> Vec<&str> {
    vec.iter().map(|s| &s[..]).collect()
}

/// Splits the given vector into a vector of vector, where each sub vector has the given size.
/// This function expects the number of items in 'v' to be evely dividable by 'size.
pub fn split_into_groups<T>(v: Vec<T>, size: usize) -> Vec<Vec<T>>
    where T: Clone
{
    v.chunks(size).map(|c| c.to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_vector_of_string() {
        let result = read_vector_of_string("input/test01.txt");
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_can_read_vector_of_i32() {
        let result = read_vector_of_i32("input/test01.txt");
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_can_read_matrix_of_string() {
        let result = read_matrix_of_string("input/test02.txt", ",");
        assert_eq!(result, vec![vec!["1", "11", "111"], vec!["2", "22", "222"], vec!["3", "33", "333"]]);
    }

    #[test]
    fn test_can_read_matrix_of_i32() {
        let result = read_matrix_of_i32("input/test02.txt", ",");
        assert_eq!(result, vec![vec![1, 11, 111], vec![2, 22, 222], vec![3, 33, 333]]);
    }

    #[test]
    fn test_can_read_separated_matrix_of_string() {
        let result = read_blank_separated_matrix_of_string("input/test03.txt");
        assert_eq!(result, vec![vec!["1", "2"], vec!["3"], vec!["4", "5", "6"]]);
    }

    #[test]
    fn test_can_read_separated_matrix_of_i32() {
        let result = read_blank_separated_matrix_of_i32("input/test03.txt");
        assert_eq!(result, vec![vec![1, 2], vec![3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_can_sum_rows() {
        let result = sum_rows(&vec![vec![1, 2], vec![3], vec![0, 0, 4, 5, 6, 0, 0]]);
        assert_eq!(result, vec![3, 3, 15]);
    }

    #[test]
    fn test_can_convert_vector_of_string() {
        let vec = vec![String::from("foo"), String::from("bar")];
        let result = to_vector_of_str(&vec);
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn test_can_split_into_groups() {
        let result = split_into_groups(vec!["a", "b", "c", "d", "e", "f"], 3);
        assert_eq!(result, vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    }
}
