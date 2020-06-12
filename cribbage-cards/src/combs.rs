pub(crate) trait CombExt<T> {
    fn combs(&self, size: usize) -> Combinations<T>;
}

impl<T> CombExt<T> for [T] {
    fn combs(&self, size: usize) -> Combinations<'_, T> {
        Combinations::new(self, size)
    }
}

pub struct Combinations<'a, T> {
    underlying: &'a [T],
    pos: Vec<usize>,
}

impl<'a, T> Combinations<'a, T> {
    fn new(buf: &'a [T], size: usize) -> Combinations<'a, T> {
        let underlying = buf;
        let size = size;

        let mut pos: Vec<usize> = (0..size).rev().collect();
        pos[0] -= 1;

        Combinations { underlying, pos }
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos[0] == self.underlying.len() {
            return None;
        }

        for i in 0..self.pos.len() {
            self.pos[i] += 1;
            if self.pos[i] != self.underlying.len() - i {
                for sub_i in (0..i).rev() {
                    self.pos[sub_i] = self.pos[i] + i - sub_i;
                }

                break;
            }
        }

        if self.pos[0] == self.underlying.len() {
            return None;
        }

        Some(
            self.pos
                .iter()
                .rev()
                .map(|i| &self.underlying[*i])
                .collect::<Vec<&'a T>>(),
        )
    }
}

#[test]
fn test_usage() {
    let arr = vec![1u32, 2, 3];
    let combs: Vec<Vec<&u32>> = arr.combs(2).collect();

    assert_eq!(combs, vec![vec![&1u32, &2], vec![&1, &3], vec![&2, &3],]);

    let arr = vec![1u32, 2, 3, 4];
    let combs: Vec<Vec<&u32>> = arr.combs(2).collect();

    assert_eq!(
        combs,
        vec![
            vec![&1u32, &2],
            vec![&1, &3],
            vec![&1, &4],
            vec![&2, &3],
            vec![&2, &4],
            vec![&3, &4],
        ]
    );

    let arr = vec![1u32, 2, 3];
    let combs: Vec<Vec<&u32>> = arr.combs(3).collect();

    assert_eq!(combs, vec![vec![&1u32, &2, &3]]);

    let arr = vec![1u32, 2, 3, 4];
    let combs: Vec<Vec<&u32>> = arr.combs(3).collect();

    assert_eq!(
        combs,
        vec![
            vec![&1u32, &2, &3],
            vec![&1, &2, &4],
            vec![&1, &3, &4],
            vec![&2, &3, &4]
        ]
    );

    let arr = vec![1u32, 2, 3, 4];
    let combs: Vec<Vec<&u32>> = arr.combs(4).collect();

    assert_eq!(combs, vec![vec![&1u32, &2, &3, &4],]);
}
