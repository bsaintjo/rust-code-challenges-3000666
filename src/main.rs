use std::cmp::Ordering;

fn median(mut a: Vec<f32>) -> Option<f32> {
    let mut contains_nan = false;
    a.sort_by(|a, b| {
        let cmp = a.partial_cmp(b);
        if let Some(ord) = cmp {
            ord
        } else {
            contains_nan = true;
            Ordering::Equal
        }
    });

    if contains_nan {
        return None;
    }

    let len = a.len();
    let idx = len / 2;

    if a.is_empty() {
        None
    } else if len % 2 == 0 {
        let avg = (a[idx] + a[idx - 1]) / 2.;
        Some(avg)
    } else {
        Some(a[idx])
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn single_list() {
    let input = vec![1.0];
    let expected_output = Some(1.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
