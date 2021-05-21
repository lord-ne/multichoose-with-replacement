use multiset::HashMultiSet;
use std::iter::FromIterator;
use num_integer;
use integer_partitions::Partitions;

fn main() {
    let n : u128 = 81;
    let k : u128 = 5;
    
    let (num, den) = prob(n, k);
    print_prob(num, den);

    println!("");

    let (count, expected) = count_num_perms(n, k);
    print_num_perms(count, expected);
    
}

// Count total number of permutations, for verification
fn count_num_perms(n : u128, k : u128) -> (u128, u128) {
    let mut count = 0;

    let mut partitions = Partitions::new(k as usize);
    while let Some(partition) = partitions.next() {
        let p_set : HashMultiSet<u128> = FromIterator::from_iter(partition.iter().map(|x| *x as u128));
        let p_arr : Vec<u128> = FromIterator::from_iter(p_set.iter().map(|x| *x));
        let p_mul : Vec<u128> = FromIterator::from_iter(p_set.distinct_elements().map(|x| p_set.count_of(x) as u128));
        
        count += num_integer::binomial(n, p_arr.len() as u128) * num_integer::multinomial(&p_mul) * num_integer::multinomial(&p_arr);
    }

    let expected : u128 = n.pow(k as u32);

    return (count, expected);
}

fn print_num_perms(count : u128, expected : u128) {
    println!("Counted {} total permutations, expected {}", count, expected);
    if count == expected {
        println!("No error");
    } else {
        println!("Error of {}", expected - count);
    }
}

// Probability that two players will choose the same k items with repetition out of n items
// Returns (numerator, denominator)
fn prob(n : u128, k : u128) -> (u128, u128) {
    let mut num = 0;

    let mut partitions = Partitions::new(k as usize);
    while let Some(partition) = partitions.next() {
        let p_set : HashMultiSet<u128> = FromIterator::from_iter(partition.iter().map(|x| *x as u128));
        let p_arr : Vec<u128> = FromIterator::from_iter(p_set.iter().map(|x| *x));
        let p_mul : Vec<u128> = FromIterator::from_iter(p_set.distinct_elements().map(|x| p_set.count_of(x) as u128));
        
        num += num_integer::binomial(n, p_arr.len() as u128) * num_integer::multinomial(&p_mul) * num_integer::multinomial(&p_arr).pow(2);
    }

    let den : u128 = n.pow(2 * k as u32);

    return (num, den);
}

fn print_prob(num : u128, den : u128) {
    println!("Result is {} / {}", num, den);
    let appr = num as f64 / den as f64;
    println!("Which is approximately {}, or 1 out of {}", appr, (1.0 / appr) as u128);
}