pub(crate) struct Data {
    index: usize,
    value: f64,
}

pub(crate) fn sparse_vector() {
    let mut sparse_vector:Vec<Data> = vec![];
    sparse_vector.push(Data {
        index: 41,
        value: 24.16,
    });
    sparse_vector.push(Data {
        index: 42,
        value: 25.16,
    });
    sparse_vector.push(Data {
        index: 43,
        value: 27.16,
    });
    sparse_vector.push(Data {
        index: 45,
        value: 29.16,
    });
    sparse_vector.push(Data {
        index: 46,
        value: 30.16,
    });
    sparse_vector.push(Data {
        index: 48,
        value: 32.16,
    });
    println!("{}", to_string(&sparse_vector).expect("None"));
    add(&mut sparse_vector, Data {
        index: 44,
        value: 64.56,
    });
    println!("{}", to_string(&sparse_vector).expect("None"));
    let length = get_vector_length(&sparse_vector);
    println!("{}", length);
}
fn add(sparse_vector: &mut Vec<Data>, data: Data) {
    let vector_index: Option<usize> = get_vector_index(&sparse_vector, data.index);

    if vector_index.is_none() {
        sparse_vector.push(data);
        return;
    }

    let vector_index: usize = vector_index.unwrap();
    if sparse_vector[vector_index].index == data.index {
        sparse_vector[vector_index].value = data.value;
        return;
    }
    sparse_vector.insert(vector_index,data);
}
fn get_vector_index(sparse_vector: &Vec<Data>, data_index: usize) -> Option<usize> {
    for i in 0 .. sparse_vector.len() {
        if sparse_vector[i].index >= data_index {
            return Some(i);
        }
    }
    None
}
fn get_vector_length(sparse_vector: &Vec<Data>) -> usize {
    let last_index = sparse_vector.len() - 1;
    return sparse_vector[last_index].index + 1;
}
fn get_element(sparse_vector: &mut Vec<Data>, data_index: usize) -> f64 {
    let vector_index: Option<usize> = get_vector_index(&sparse_vector, data_index);

    if vector_index.is_none() {
        return 0.0;
    }
    let vector_index = vector_index.unwrap();
    return sparse_vector[vector_index].value;
}

fn to_string(sparse_vector: &Vec<Data>) -> Option<String> {
    if sparse_vector.is_empty() {
        return None;
    }

    let mut sparse_vector_string: String = String::from("[");
    for element in sparse_vector {
        sparse_vector_string += format!("({},{}), ",
                                        element.index.to_string().as_str(),
                                        element.value.to_string().as_str()
        ).as_str();
    }
    sparse_vector_string += ",";
    Some(sparse_vector_string.replace(", ,","]"))
}