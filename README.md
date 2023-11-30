# algorithms-and-data-structures-rust
 Algorithms and Data-structures but in rust

## abstract_data_types
Has implementations of various ADTs in it

### Sparse Vector
Just saves non-Zero values to use less space. 
Extremely useful when working with big vectors with many Zero-Values.

Implemented with an std::vector as the underlying Data-Structure, each Value in that Vector is a Struct with an index and it's value as f64. 
Every other value is implicitly Zero.

Important to differ is between the vector_index and the sparse_vector_index.
The first one are just the non-Zero Values, so the indices from the underlying vector.
The second one is the index inside the Struct.
That index is the "real" index.
