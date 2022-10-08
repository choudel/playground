struct OurIterator<'a> {
    nums: &'a [i32],
    i: usize,
    x: i32,
}

impl<'a> OurIterator<'a> {
    fn new(nums: &'a [i32], x: i32) -> Self {
        Self { nums, i: 0, x }
    }
}

impl<'a> Iterator for OurIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.nums.len() {
            let value = self.nums[self.i];
            self.i += 1;
            Some(value * self.x)
        } else {
            None
        }
    }
}
fn main() {
    let mut nums = vec![1, 2, 3];

    let mut nums2 = OurIterator::new(&nums, 2);
    let mut nums3 = OurIterator::new(&nums, 3);
    while let Some(n) = nums2.next() {
        println!("{}", n);
    }

    while let Some(n) = nums3.next() {
        println!("{}", n);
    }
    nums.push(5);
}
