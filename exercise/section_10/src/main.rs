#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

#[derive(Debug)]
struct Item {
    name: String,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl std::iter::Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn sort_pop(city: &mut Vec<City>) {
    city.sort_by_key(pop_helper)
}

fn pop_helper(pop: &City) -> u64 {
    pop.population
}

fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population)
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|i| i.name == product).collect()
}

fn main() {
    let a = City {
        city: String::from("A"),
        population: 100,
    };
    let b = City {
        city: String::from("B"),
        population: 50,
    };
    let c = City {
        city: String::from("C"),
        population: 70,
    };
    let d = City {
        city: String::from("D"),
        population: 10,
    };
    let e = City {
        city: String::from("E"),
        population: 78,
    };

    let mut vec: Vec<City> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);
    vec.push(e);
    sort_pop(&mut vec);
    println!("{:?}", vec);

    sort_pop_closure(&mut vec);
    println!("{:?}", vec);

    let vec = vec![1, 2, 3];
    for val in vec.iter() {
        println!("{val}");
    }

    let vec2 = vec![1, 2, 3, 4, 5];
    let mut iter = (&vec2).into_iter();

    while let Some(v) = iter.next() {
        println!("using while:  {v}");
    }

    let mut vec3 = Vec::new();
    vec3.push(Item {
        name: String::from("coat"),
    });
    vec3.push(Item {
        name: String::from("shirt"),
    });
    vec3.push(Item {
        name: String::from("sorts"),
    });
    vec3.push(Item {
        name: String::from("shoes"),
    });

    let checked = check_inventory(vec3, String::from("shirt"));
    println!("{:?}", checked);

    let mut range = Range { start: 0, end: 10 };
    for r in range {
        println!("custom implement iterator trait: {r}")
    }
    let vec4: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("using filtered {:?}", vec4);
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // any default methods
}
