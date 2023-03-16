struct Matrix {
  n: i32,
  m: i32, 
  arr: Vec<Vec<i32>>
}

struct Vector {
  space: i32,
  arr: Vec<i32>
}

fn build_vector(n: i32, ele: Vec<i32>) -> Vector {
  return Vector { space: n, arr: ele };
}

fn print_vector(v: Vector) -> (){
  for i in &(v.arr)  {
    println!("{i}");
  }
}

fn dot_vector(v1: Vector, v2: Vector) ->  Vector {
  let mut result : Vector = todo!();
  result.space = v1.space; // could also be v2.space
  // let mut temp: i32;
  let mut index: usize = 0;

  while index <= v1.space as usize{
    result.arr.push(&v1.arr[index] * &v2.arr[index]);
    index+=1;

  }
  
  return result;
}

// fn matrix_vec_mult(m: Matrix, v: Vector) -> Vector {
//
//
// }

fn main() {
  let test = build_vector(2, vec![1, 2]);
  let test2 = build_vector(2, vec![4, 5]);
  let dot_product = dot_vector(test, test2);
  print_vector(dot_product);
}


