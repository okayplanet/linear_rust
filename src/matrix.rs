use vector::Vector;
use std::ops::Add;
use std::ops::Mul;

pub struct RowMatrix {
    numRows: usize,
    numColumns: usize,
}

impl RowMatrix {
    pub fn new(v: Vec<Vector>) -> RowMatrix {
        RowMatrix { numRows: v.len(), numColumns: v[0].len(), }
    }
}

pub struct ColumnMatrix {
    numRows: usize,
    numColumns: usize,
}

impl ColumnMatrix {
    pub fn new(v: Vec<Vector>) -> ColumnMatrix {
        ColumnMatrix { numRows: v.len(), numColumns: v[0].len(), }
    }
}

pub trait Matrix {
    fn get(&self, r: i64, c: i64) -> f64;
    fn rowCount(&self) -> usize; 
}

impl Matrix for RowMatrix {
    fn get(&self, r: i64, c: i64) -> f64 {
        0.0
    }
    fn rowCount(&self) -> usize {
        self.numRows
    }
}

impl Matrix for ColumnMatrix {
    fn get(&self, r: i64, c: i64) -> f64 {
        0.0
    }
    fn rowCount(&self) -> usize {
        self.numRows
    }
}

// Addition Operator Overloads 
add_overload!(RowMatrix, RowMatrix, i64, self, object, { 0 });
add_overload!(RowMatrix, ColumnMatrix, i64, self, object, { 0 });

// Multiplication Operator Overloads 
mul_overload!(RowMatrix, ColumnMatrix, i64, self, object, { 0 });
mul_overload!(RowMatrix, RowMatrix, i64, self, object, { 0 });
