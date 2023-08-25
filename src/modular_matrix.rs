/*
 * Defines `ModularMatrix`, a generic type for representing two-dimensional vectors (arrays) whose
 * indices "wrap-around"; in other words, any integral value can be used to get elements, the
 * actual index in the array is calculated from using the mathematical modulo operation.
 *
 * Note that, since we are using `Vec`tors to store the actual matrix values, elements in the
 * matrix require cloning and therefore their type must implement the `std::clone::Clone` trait.
 *
 * Also, because it is expected matrices of simple types (e.g. numbers) will be used more often, a simplification
 * was made by requiring the elements' type to implement the `Copy` trait too.
 */

#[derive(Debug)]
pub struct ModularMatrix<T: std::clone::Clone + Copy> {
    width: usize,
    height: usize,
    elements: Vec<Vec<T>>, // two-dimensional vector
}


// FIXME: How to create a name for the constrained generic T:Foo+Bar to avoid repetition?
impl<T: std::clone::Clone + Copy> ModularMatrix<T> {

    // Constructor
    pub fn new(width: usize, height: usize, init: T) -> ModularMatrix<T> {

        let mut columns: Vec<Vec<T>> = vec![];

        // Initialize all the matrix elements to `init`:
        for _col in 0 .. width {
            // TODO: There must be a more efficient way of doing this. 
            columns.push( vec![init; height] );
        }

        ModularMatrix {
            width,
            height,
            elements: columns,
        }
    }


    // Returns the element at the given position.
    pub fn get(&self, modular_column_idx: isize, modular_row_idx: isize) -> T {
        let column_idx = modulo(modular_column_idx, self.width);
        let row_idx    = modulo(modular_row_idx,    self.height);
        self.elements[column_idx][row_idx]
    }

    // Sets the element at the given position and returns it.
    pub fn set(&mut self, modular_column_idx: isize, modular_row_idx: isize, element_value: T) -> T {
        let column_idx = modulo(modular_column_idx, self.width);
        let row_idx    = modulo(modular_row_idx,    self.height);
        self.elements[column_idx][row_idx] = element_value;
        element_value
    }

    // Returns the number of columns in the matrix:
    pub fn width(&self) -> usize { self.width }

    // Returns the number of rows in the matrix:
    pub fn height(&self) -> usize { self.height }
}

// Calculates the (mathematical) modulo operation.
// The result should always be a value between `0` and `radix - 1`.
fn modulo(integer: isize, radix: usize) -> usize {
    let iradix: isize = radix as isize;
    ( (integer + iradix) % iradix ) as usize 
}

