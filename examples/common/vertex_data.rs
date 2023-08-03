#![allow(dead_code)]
pub fn cube_data() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[i8; 2]>, Vec<[i8; 3]>) {
    let positions = [
        // front (0, 0, 1)
        [-1, -1, 1], [1, -1, 1], [-1, 1, 1], [-1, 1, 1], [ 1, -1, 1], [ 1, 1, 1],
        // right (1, 0, 0)
        [ 1, -1, 1], [1, -1, -1], [ 1, 1, 1], [ 1, 1, 1], [ 1, -1, -1], [ 1, 1, -1],
        // back (0, 0, -1)
        [ 1, -1, -1], [-1, -1, -1], [1, 1, -1], [ 1, 1, -1], [-1, -1, -1], [-1, 1, -1],
        // left (-1, 0, 0)
        [-1, -1, -1], [-1, -1, 1], [-1, 1, -1], [-1, 1, -1], [-1, -1, 1], [-1, 1, 1],
        // top (0, 1, 0)
        [-1, 1, 1], [ 1, 1, 1], [-1, 1, -1], [-1, 1, -1], [ 1, 1, 1], [ 1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1], [ 1, -1, -1], [-1, -1, 1], [-1, -1, 1], [ 1, -1, -1], [ 1, -1, 1],
    ];
    let colors = [
        // front - blue
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        // right - red
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
        // back - yellow
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0],
        // left - aqua
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1],
        // top - green
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        // bottom - fuchsia
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1],
    ];
    let uvs= [
        // front
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // right
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // back
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // left
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // top
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
        // bottom
        [0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
    ];
    let normals = [
        // front
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        // right
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
        // back
        [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1],
        // left
        [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0],
        // top
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        // bottom
        [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0],
    ];
    (positions.to_vec(), colors.to_vec(), uvs.to_vec(), normals.to_vec())
}
