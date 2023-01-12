/**
 * In this kata you have to correctly return who is the "survivor",
 * ie: the last element of a Josephus permutation.
 * Basically you have to assume that n people are put into a circle
 * and that they are eliminated in steps of k elements, like this:

josephus_survivor(7,3) => means 7 people in a circle;
one every 3 is eliminated until one remains
[1,2,3,4,5,6,7] - initial sequence
[1,2,4,5,6,7] => 3 is counted out
[1,2,4,5,7] => 6 is counted out
[1,4,5,7] => 2 is counted out
[1,4,5] => 7 is counted out
[1,4] => 5 is counted out
[4] => 1 counted out, 4 is the last element - the survivor!
 * The above link about the "base" kata description will give you
 * a more thorough insight about the origin of this kind of permutation,
 * but basically that's all that there is to know to solve this kata.
 */

pub fn josephus_survivor(people: usize, step: usize) -> usize {
    let mut alive: Vec<usize> = Vec::new();
    let mut executed: Vec<usize> = Vec::new();

    // store alive people
    for person in 1..=people {
        alive.push(person);
    }

    // count alive people
    let mut len = alive.len();
    let mut killed: usize = 0;

    // show people alive before survival process
    println!("{:?}", alive);

    // start survival process
    // just one person must survive (presumably, the old Josephus)
    while len > 1 {
        // the person to be killed will be in this position
        killed = (killed + step - 1) % len;

        // this already dead person is included in our executed list
        executed.push(alive.remove(killed));

        // since someone is now dead, the length of alive people list must decrease
        len -= 1;

        // show rest of survivors
        println!("{:?}", alive);
    }

    // return the last person alive
    *alive.get(0).unwrap()
}
