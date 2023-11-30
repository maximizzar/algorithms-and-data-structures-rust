mod search;
mod abstract_data_types;
mod sort_algorithms;

fn main() {
    {
        use abstract_data_types::SparseVector;

        let mut sparse: SparseVector = SparseVector {
            elements: vec![],
        };
        sparse.add(42,25.16);
        sparse.add(45,29.16);
        sparse.add(43,27.16);
        sparse.add(48,32.16);
        sparse.add(41,24.16);
        sparse.add(46,30.16);
        sparse.add(48,32.16);

        println!("{}", sparse.to_string_sparse().expect("Vector is empty"));
        sparse.add(44,64.56);
        println!("{}", sparse.to_string_sparse().expect("Vector is empty"));
        sparse.delete(43);

        println!("{}\n{}\n",
                 sparse.to_string_sparse().expect("Vector is empty"),
                 sparse.to_string_dense().expect("Vector is empty")
        );
    }
}