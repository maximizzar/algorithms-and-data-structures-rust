pub struct SparseVector {
    pub elements: Vec<Element>
}
pub struct Element {
    index: usize,
    value: f64,
}

impl SparseVector {
    pub fn add(&mut self, index: usize, value: f64) {
        let elements_index: Option<usize> =
            SparseVector::get_vector_index(&self, index);

        if elements_index.is_none() {
            self.elements.push(Element{index, value});
            return;
        }

        let elements_index: usize = elements_index.unwrap();
        if self.elements[elements_index].index == index {
            self.elements[elements_index].value = value;
            return;
        }
        self.elements.insert(elements_index, Element{index, value});
    }
    pub fn delete(&mut self, index: usize) {
        let elements_index: Option<usize> =
            SparseVector::get_vector_index(&self, index);

        if elements_index.is_none() {
            return;
        }
        let elements_index: usize = elements_index.unwrap();

        self.elements.remove(elements_index);
    }
    pub fn get(&self, index: usize) -> f64 {
        let vector_index: Option<usize> =
            SparseVector::get_vector_index(&self, index);

        if vector_index.is_none() {
            return 0.0;
        }
        let vector_index = vector_index.unwrap();
        return self.elements[vector_index].value;
    }
    fn get_vector_index(&self, index: usize) -> Option<usize> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].index >= index {
                return Some(i);
            }
        }
        None
    }
    pub fn get_vector_length(&self) -> usize {
        let last_index = self.elements.len() - 1;
        return self.elements[last_index].index + 1;
    }
    pub fn to_string_sparse(&self) -> Option<String> {
        if self.elements.is_empty() {
            return None;
        }
        let mut sparse_vector_string: String = String::from("[");
        for element in self.elements.iter().clone() {
            sparse_vector_string += format!("({},{}), ",
                                            element.index.to_string().as_str(),
                                            element.value.to_string().as_str()).as_str();
        }
        sparse_vector_string += ",";
        Some(sparse_vector_string.replace(", ,","]"))
    }
    pub fn to_string_dense(&self) -> Option<String> {
        if self.elements.is_empty() {
            return None;
        }
        let mut sparse_vector_string: String = String::from("[");
        let mut last_index: usize = 0;

        for element in self.elements.iter().clone() {
            let mut zero_values = String::new();
            for _i in 0 .. element.index - last_index {
                zero_values += "0.0, ";
            }
            last_index = element.index;
            sparse_vector_string += format!("{}{}, ", zero_values,
                                            element.value.to_string().as_str()).as_str();
        }
        sparse_vector_string += ",";
        Some(sparse_vector_string.replace(", ,","]"))
    }
}