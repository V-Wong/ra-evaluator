use ra_evaluator::*;

fn main() {
    let result = ExpressionBuilder::new(Terminal::new(&[(1, "a"), (2, "b"), (3, "c")]))
    .select(|x| x.0 > 1)
    .project(|x| x.1)
    .cartesian_product(&[1, 2], |x, y| (*x, *y))
    .join(&[(1, "Join1"), (2, "Join2")], |x, y| x.1 == y.0, |x, y| (x.0, y.0, y.1))
    .union(&[("d", 3, "Union")])
    .intersect(&[
        ("c", 1, "Join1"),
        ("c", 2, "Join2"),
        ("d", 3, "Union"),
        ("e", 4, "Removed"),
    ])
    .eval();

println!("{:?}", result);
}
