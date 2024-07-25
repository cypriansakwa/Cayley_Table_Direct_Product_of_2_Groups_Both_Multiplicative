use num::integer::gcd;

// Function to find elements of Z_n* (multiplicative group)
fn zn_star(n: u32) -> Vec<u32> {
    (1..n).filter(|&x| gcd(x, n) == 1).collect()
}

// Function to compute the direct product of Z_n* and Z_m*
fn direct_product(n: u32, m: u32) -> Vec<(u32, u32)> {
    let zn_star_elements = zn_star(n);
    let zm_star_elements = zn_star(m);
    
    let mut product = Vec::new();
    for &a in &zn_star_elements {
        for &b in &zm_star_elements {
            product.push((a, b));
        }
    }
    product
}

// Function to compute the product (a, b) * (e, f) mod (n, m)
fn product_mod((a, b): (u32, u32), (e, f): (u32, u32), n: u32, m: u32) -> (u32, u32) {
    ((a * e) % n, (b * f) % m)
}

// Function to print the Cayley table
fn print_cayley_table(n: u32, m: u32) {
    let elements = direct_product(n, m);

    // Print the header row
    print!("       ");
    for &elem in &elements {
        print!("{:?} ", elem);
    }
    println!();

    // Print each row of the table
    for &elem1 in &elements {
        print!("{:?} ", elem1);
        for &elem2 in &elements {
            let result = product_mod(elem1, elem2, n, m);
            print!("{:?} ", result);
        }
        println!();
    }
}

fn main() {
    let n = 6; // Change this to the desired value of n
    let m = 8; // Change this to the desired value of m

    println!("Cayley table for Z_{}^* x Z_{}^*:", n, m);
    print_cayley_table(n, m);
}

