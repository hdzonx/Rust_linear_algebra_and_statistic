struct Array_Sort {
    arr: [i32; 6],
}
//implementatação da Struct Array_Sort
impl Array_Sort {
    fn sort(arr: &mut [i32]) {
        println!("the slice has {} elements", arr.len());

        for i in 0..arr.len() {
            println!(" o i-ésimo valor no array é {}", arr[i]);
        }
    }
}
//function main para rodas os dados do problema
fn main() {
    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    Array_Sort::sort(&mut arr);
}
