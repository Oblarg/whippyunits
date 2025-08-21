#![feature(generic_const_exprs)]

trait ConstCheck<const CHECK: bool> {}
impl ConstCheck<true> for () {}

fn test_constraint<const A: i32, const B: i32>()
where
    (): ConstCheck<{ A == B }>,
{
    println!("A = {}, B = {}", A, B);
}

fn main() {
    // This should work
    test_constraint::<5, 5>();
    
    // This should fail
    test_constraint::<5, 6>();
} 