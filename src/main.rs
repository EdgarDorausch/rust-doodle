use std::collections::HashSet;


fn main() {

    let mut node = HashSet::<i32>::new();
    let mut edge = HashSet::<(i32, i32)>::new();
    let mut reachable = HashSet::<(i32, i32)>::new();

    edge.insert((1,2));
    edge.insert((2,3));
    edge.insert((3,4));
    edge.insert((3,5));


    loop {
        let mut change = false;

        // r1
        for (x,_) in &edge {
            change |= node.insert(*x);
        }

        // r2
        for (_,y) in &edge {
            change |= node.insert(*y);
        }

        // r3
        let mut reachable_changes = HashSet::<(i32, i32)>::new();
        for (x,z1) in &reachable {
            for (z2,y) in &edge {
                if z1 == z2 {
                    reachable_changes.insert((*x,*y));
                }
            }
        }
        
        reachable.extend(&reachable_changes);
        change |= !reachable_changes.is_subset(&reachable);

        // r4
        for (x,y) in &edge {
            change |= reachable.insert((*x,*y));
        }

        if !change {
            break;
        }
    }
    
    println!("{:?}", reachable);
}