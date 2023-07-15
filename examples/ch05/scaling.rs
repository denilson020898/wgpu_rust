use cgmath::{Matrix4, Vector4};

fn main() {
    let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);
    let my_mat = Matrix4::from_nonuniform_scale(0.5, 0.5, 1.5);
    let scaled_vec = my_mat * my_vec;
    println!("\noriginal vector: \n{:?}", my_vec);
    println!("scaling matrix: \n{:?}", my_mat);
    println!("scaled_vec: my_may * my_vec = \n{:?}", scaled_vec);

    let scaling_mat = my_mat * Matrix4::from_nonuniform_scale(1.0, 0.5, 0.3);
    let final_vec = scaling_mat * my_vec;
    println!("\nscaling may: \n{:?}", scaling_mat);
    println!("final vec: \n{:?}", final_vec);

}
