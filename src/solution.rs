pub struct Solution {
    pathA: WagonPath,
    pathB: WagonPath,
}

struct WagonPath {
    routes: [Vec<usize>; 5],
}
