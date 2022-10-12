#[derive(Debug, Clone)]
pub struct Plan {
    sum: u32,
    indexes: Vec<usize>,
}
impl Plan {
    pub fn new(sum: u32, indexes: Vec<usize>) -> Self {
        Self { sum, indexes }
    }

    pub fn add(&mut self, number: u32, index: usize) -> Self {
        self.sum += number;
        self.indexes.push(index);

        self.clone()
    }

    pub fn sum(&self) -> u32 {
        self.sum
    }
}

pub fn algorithm_test1(full_array: Vec<u32>, k: u32) -> Vec<Plan> {
    let mut sorted_array = full_array.clone();
    sorted_array.sort();

    let mut plans: Vec<Plan> = Vec::new();

    if k == 0 {
        return plans;
    }

    if full_array.len() == 1 {
        for _ in 0..k {
            plans.push(Plan::new(sorted_array[0], vec![0]));
        }
        return plans;
    }

    plans.push(Plan::new(sorted_array[0], vec![0]));
    let mut array_pointer: usize = 1;
    let mut smallest_sum_pointer = 1;
    let mut smallest_sum_plan = Plan::new(sorted_array[0] + sorted_array[1], vec![0, 1]);

    for _ in 0..k - 1 {
        if sorted_array[array_pointer] < smallest_sum_plan.sum {
            plans.push(Plan::new(sorted_array[array_pointer], vec![array_pointer]));
            array_pointer += 1;
        } else {
            plans.push(smallest_sum_plan.clone());
            smallest_sum_pointer += 1;
            smallest_sum_plan.add(sorted_array[smallest_sum_pointer], smallest_sum_pointer);
        }
    }

    plans
}

#[derive(Debug, Copy, Clone)]
pub struct CargoWall {
    start: isize,
    end: isize,
    cargos: usize,
}

impl CargoWall {
    pub fn new(start: isize, end: isize, cargos: usize) -> Self {
        Self { start, end, cargos }
    }
}

pub fn algorithm_test2(cargos: String, starts: Vec<isize>, ends: Vec<isize>) -> Vec<CargoWall> {
    let mut sum = 0;
    let mut result: Vec<CargoWall> = Vec::new();
    for start in starts {
        for end in ends.clone() {
            if start >= (cargos.len() - 1) as isize {
                return result;
            }

            if end as isize - start as isize <= 1 {
                return result;
            }

            let start_inner = if start < 0 { 0 } else { start as usize };

            let end_inner = if end > cargos.len() as isize {
                cargos.len()
            } else {
                end as usize
            };
            if end as usize == cargos.len() {
                let sum = get_cargos(String::from(&cargos.clone().as_str()[start_inner..]));
                result.push(CargoWall::new(start, end, sum));
            } else {
                let sum = get_cargos(String::from(
                    &cargos.clone().as_str()[start_inner..end_inner + 1],
                ));
                result.push(CargoWall::new(start, end, sum));
            }
        }
    }

    result
}

fn get_cargos(cargo_sub_string: String) -> usize {
    let mut sum = 0;

    let mut left_wall = false;
    let mut counter = 0;
    for c in cargo_sub_string.chars() {
        if c == '|' {
            if !left_wall {
                left_wall = true;
                continue;
            }
            sum += counter;
            counter = 0;
        } else if c == '*' {
            if left_wall {
                counter += 1;
            }
        } else {
            panic!("unknown character");
        }
    }

    sum
}
