pub struct UF {
    qtd_components: i32,
    components: Vec<i32>,
    size_root: Vec<i32>,
}

impl UF {
    pub fn new(x: i32) -> UF {
	let mut components = Vec::new();
	let mut size_root = Vec::new();
	for i in 0..x {
	    components.push(i);
	    size_root.push(1);
	}

	UF {
	    qtd_components: x,
	    components,
	    size_root,
	}
    }

    pub fn count(&self) -> i32 {
	self.qtd_components
    }

    pub fn connected(&mut self, x: i32, y: i32) -> bool {
	self.find(x) == self.find(y)
    }

    pub fn find(&mut self, mut x: i32) -> i32 {
	let mut aux = Vec::new();
	while x != self.components[x as usize] {
	    aux.push(x);
	    x = self.components[x as usize];
	}
	for i in aux.iter() {
	    self.components[*i as usize] = x;
	}

	return x;
    }

    pub fn union(&mut self, x: i32, y: i32) {
	let i = self.find(x);
	let j = self.find(y);

	if i == j {
	    return;
	} else if self.size_root[i as usize] < self.size_root[j as usize] {
	    self.components[i as usize] = j;
	    self.size_root[j as usize] = self.size_root[j as usize] + self.size_root[i as usize];
	} else {
	    self.components[j as usize] = i;
	    self.size_root[i as usize] = self.size_root[i as usize] + self.size_root[j as usize];
	}
	self.qtd_components = self.qtd_components - 1;
    }

    // pub fn find(&self, x: i32) -> i32 {
    //	self.components[x as usize]
    // }

    // pub fn union(&mut self, x: i32, y: i32) {
    //	if self.connected(x, y) {
    //	    return;
    //	} else {
    //	    let class_y = self.components[y as usize];
    //	    for i in 0..self.components.len() {
    //		if self.components[i as usize] == class_y {
    //		    self.components[i as usize] = self.components[x as usize];
    //		}
    //	    }
    //	    self.qtd_components = self.qtd_components - 1;
    //	}
    // }
}
