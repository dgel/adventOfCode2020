use ndarray::prelude::*;

fn step(current: &Array<u8, Ix3>, dest: &mut Array<u8, Ix3>) {
    let depth = current.shape()[0];
    let height = current.shape()[1];
    let width = current.shape()[2];
    for z in 1..depth - 1 {
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let all = current
                    .slice(s![z - 1..z + 2, y - 1..y + 2, x - 1..x + 2])
                    .sum();
                let own = current[[z, y, x]];
                let surround = all - own;
                if surround == 3 || (own == 1 && surround == 2) {
                    dest[[z, y, x]] = 1;
                } else {
                    dest[[z, y, x]] = 0;
                }
            }
        }
    }
}

fn simulate(start_slice: &Array<u8, Ix2>, n_steps: usize) -> Array<u8, Ix3> {
    let size_increase = (n_steps + 1) * 2; // add 2 for to index around the edges
    let height = start_slice.shape()[0];
    let width = start_slice.shape()[1];
    let depth = 1;
    let mut start = Array::<u8, Ix3>::zeros((
        depth + size_increase,
        height + size_increase,
        width + size_increase,
    ));
    start
        .slice_mut(s![
            n_steps + 1,
            n_steps + 1..n_steps + 1 + height,
            n_steps + 1..n_steps + 1 + width
        ])
        .assign(&start_slice.view());
    let mut dest = Array::<u8, Ix3>::zeros((
        depth + size_increase,
        height + size_increase,
        width + size_increase,
    ));
    for _ in 0..n_steps {
        step(&start, &mut dest);
        std::mem::swap(&mut start, &mut dest);
    }
    start
}

fn step2(current: &Array<u8, Ix4>, dest: &mut Array<u8, Ix4>) {
    let hyper = current.shape()[0];
    let depth = current.shape()[1];
    let height = current.shape()[2];
    let width = current.shape()[3];
    for h in 1..hyper - 1 {
        for z in 1..depth - 1 {
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let all = current
                        .slice(s![h - 1..h + 2, z - 1..z + 2, y - 1..y + 2, x - 1..x + 2])
                        .sum();
                    let own = current[[h, z, y, x]];
                    let surround = all - own;
                    if surround == 3 || (own == 1 && surround == 2) {
                        dest[[h, z, y, x]] = 1;
                    } else {
                        dest[[h, z, y, x]] = 0;
                    }
                }
            }
        }
    }
}

fn simulate2(start_slice: &Array<u8, Ix2>, n_steps: usize) -> Array<u8, Ix4> {
    let size_increase = (n_steps + 1) * 2; // add 2 for to index around the edges
    let height = start_slice.shape()[0];
    let width = start_slice.shape()[1];
    let depth = 1;
    let hyper = 1;
    let mut start = Array::<u8, Ix4>::zeros((
        hyper + size_increase,
        depth + size_increase,
        height + size_increase,
        width + size_increase,
    ));
    start
        .slice_mut(s![
            n_steps + 1,
            n_steps + 1,
            n_steps + 1..n_steps + 1 + height,
            n_steps + 1..n_steps + 1 + width
        ])
        .assign(&start_slice.view());
    let mut dest = Array::<u8, Ix4>::zeros((
        hyper + size_increase,
        depth + size_increase,
        height + size_increase,
        width + size_increase,
    ));
    for _ in 0..n_steps {
        step2(&start, &mut dest);
        std::mem::swap(&mut start, &mut dest);
    }
    start
}

fn main() {
    let start_slice: Array<u8, Ix2> = array![
        [1, 1, 0, 0, 1, 1, 1, 1],
        [0, 1, 1, 1, 0, 0, 0, 0],
        [1, 0, 1, 1, 1, 0, 1, 1],
        [1, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 0, 1, 0],
        [1, 0, 1, 0, 0, 0, 1, 1],
        [0, 0, 1, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 0, 0, 1, 0],
    ];
    let result_1 = simulate(&start_slice, 6);
    println!("Part 1: {}", result_1.sum());
    let result_2 = simulate2(&start_slice, 6);
    println!("Part 2: {}", result_2.iter().map(|&v| v as u64).sum::<u64>());
}
