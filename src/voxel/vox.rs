pub trait Vox{
    
    fn bitset(&self)->u64;
    fn children(&self, index:usize)->&Self;
    fn mut_children(&mut self, index:usize)->&mut Self;
    fn num_children(&self)->usize;
    fn or_bitset(&mut self, u64);
    fn new_children(&mut self);
    fn drain(&mut self);
    
    fn get_tree(&self, location:&Vec<u64>)->&Self{
        let mut stack = Vec::new();
        stack.push(self);
        let last = location.len() - 1;
        for i in 0..last{
            let top = match stack.pop(){
                Some(x) => x,
                None => panic!("Oh no's, stack in empty!"),
            };
            let node = top.get_node(location[i]);
            stack.push(node);
        }
        let last_node = match stack.pop(){
             Some(x) => x,
             None => panic!("stack in empty!"),
        };
        last_node.get_node(location[last])
    }
    
    ///
    /// get the child node located  
    ///
    fn get_node(&self, location:u64)->&Self{
        if self.is_occupied(location){
            let index = self.index_of(location);
            return self.children(index);
        }
        else{
            panic!("No Voxtree at this location");
        }
    }
    
    
    ///A much better implementation of setting
    fn set_path(&mut self, location:u64){
        if self.is_empty(location){
            self.new_children();
        }
        self.or_bitset(location);
    }    
    
    
    ///
    ///short method using bitset operations
    /// counting the number of 1's before a certain the first and only 1's in a location
    /// subtracting 1 to base2 number will flip the bits before it
    /// anding this value to the bitset will give the 1's before the first and only 1 in location 64bit integer 
    /// 
    fn index_of(&self, location:u64)->usize{
        let location = location - 1;
        let ones = self.bitset() & location;
        ones.count_ones() as usize
    }
    
    fn is_location_occupied(&self, location:&Vec<u64>)->(usize, bool){
        let mut stack = Vec::new();
        stack.push(self);
        let last = location.len() - 1;
        for i in 0..location.len(){
            let top = match stack.pop(){
                Some(x) => x,
                None => panic!("Empty stack!"),
            };
            if top.is_empty(location[i]){
                return (i, false);
            }
            let node = top.get_node(location[i]);
            stack.push(node);
        }
        let last_node = match stack.pop(){
            Some(x) => x,
            None => panic!("Empty stack!"),
        };
        (last, last_node.is_occupied(location[last]))
    }
    
        ///
    /// checks whether this node is set or not
    ///
    fn is_occupied(&self, location:u64)->bool{
        self.bitset() & location == location
    }
    
    ///
    /// checks whether the node has value or not
    ///
    fn is_empty(&self, location:u64)->bool{
        !self.is_occupied(location)// or self.bitset == 0
    }
    
    
    fn count_leaves(&self)->usize{
        let mut stack = Vec::new();
        stack.push(self);
        let mut cnt = 0;
        while stack.len() > 0{
            let top = match stack.pop(){
                Some(x) => x,
                None => panic!("Error here"),
            };
            for i in 0..top.num_children(){
                let child = top.children(i);
                stack.push(child);
                if child.is_leaf(){
                    cnt += 1;
                }
            }
        }
        println!("There are {} leaf nodes..",cnt);
        cnt
    }
    
    ///
    ///it is a leaf when there is no children
    ///
    ///
    
    fn is_leaf(&self)->bool{
        self.num_children() == 0
    }
    
    ///
    /// Get the node as mutable at this location
    ///
    ///
    
    fn get_as_mut(&mut self, location:u64)->&mut Self{
        if self.is_occupied(location){
            let index = self.index_of(location);
            return self.mut_children(index);
        }
        else{
            panic!("No Voxtree at location: {:8b}",location);
        }
    }
    
}

