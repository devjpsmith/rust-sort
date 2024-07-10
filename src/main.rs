mod bubble;
mod selection;
mod insertion;
mod merge;

fn main() {
    let mut vec = vec![ 4, 4, 1, 0, 0, 0, 8, 1, 9, 1 ];

    println!("unsorted --> {:?}", vec);
    vec = merge::sort(vec);
    println!("sorted ----> {:?}", vec);
}
