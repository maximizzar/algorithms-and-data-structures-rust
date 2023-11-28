pub struct SparseVector {
    pub(crate) elements: Vec<Data>
}
pub struct Data {
    pub index: usize,
    pub value: f64,
}

impl SparseVector {
    pub fn add(&mut self, data: Data) {
        let vector_index: Option<usize> =
            SparseVector::get_vector_index(&self, data.index);

        if vector_index.is_none() {
            self.elements.push(data);
            return;
        }

        let elements_index: usize = vector_index.unwrap();
        if self.elements[elements_index].index == data.index {
            self.elements[elements_index].value = data.value;
            return;
        }
        self.elements.insert(elements_index, data);
    }
    pub fn delete(&mut self, data_index: usize) {
        let vector_index: Option<usize> =
            SparseVector::get_vector_index(&self, data_index);

        if vector_index.is_none() {
            return;
        }
        let vector_index: usize = vector_index.unwrap();

        self.elements.remove(vector_index);
    }
    fn get_vector_index(&self, data_index: usize) -> Option<usize> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].index >= data_index {
                return Some(i);
            }
        }
        None
    }
    pub fn get_vector_length(&self) -> usize {
        let last_index = self.elements.len() - 1;
        return self.elements[last_index].index + 1;
    }
    pub fn get_element(&self, data_index: usize) -> f64 {
        let vector_index: Option<usize> =
            SparseVector::get_vector_index(&self, data_index);

        if vector_index.is_none() {
            return 0.0;
        }
        let vector_index = vector_index.unwrap();
        return self.elements[vector_index].value;
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