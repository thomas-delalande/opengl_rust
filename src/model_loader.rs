use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indexes: Vec<u16>,
}

pub fn load_file(path: &str) -> Model {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut normals: Vec<Normal> = Vec::new();
    let mut vertex_indexes: Vec<u16> = Vec::new();
    let mut normal_indexes: Vec<u16> = Vec::new();
    // This still does not read UVs from files
    let mut uv_indexes: Vec<u16> = Vec::new();

    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(line) => {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                match split.first() {
                    Some(&"v") => vertices.push(Vertex {
                        position: (
                            split.get(1).unwrap().parse().unwrap(),
                            split.get(2).unwrap().parse().unwrap(),
                            split.get(3).unwrap().parse().unwrap(),
                        ),
                    }),
                    Some(&"vn") => normals.push(Normal {
                        normal: (
                            split.get(1).unwrap().parse().unwrap(),
                            split.get(2).unwrap().parse().unwrap(),
                            split.get(3).unwrap().parse().unwrap(),
                        ),
                    }),
                    Some(&"f") => {
                        let index_1 = split.get(1).unwrap().split('/').collect::<Vec<&str>>();
                        let index_2 = split.get(2).unwrap().split('/').collect::<Vec<&str>>();
                        let index_3 = split.get(3).unwrap().split('/').collect::<Vec<&str>>();

                        if let Some(index) = index_1.get(0) {
                            if let Ok(number) = index.parse::<u16>() {
                                vertex_indexes.push(number);
                            }
                        }

                        if let Some(index) = index_2.get(0) {
                            if let Ok(number) = index.parse::<u16>() {
                                vertex_indexes.push(number);
                            }
                        }

                        if let Some(index) = index_3.get(0) {
                            if let Ok(number) = index.parse::<u16>() {
                                vertex_indexes.push(number);
                            }
                        }

                        if let Some(index) = index_1.get(1) {
                            if let Ok(number) = index.parse::<u16>() {
                                uv_indexes.push(number);
                            }
                        }
                        if let Some(index) = index_2.get(1) {
                            if let Ok(number) = index.parse::<u16>() {
                                uv_indexes.push(number);
                            }
                        }
                        if let Some(index) = index_3.get(1) {
                            if let Ok(number) = index.parse::<u16>() {
                                uv_indexes.push(number);
                            }
                        }


                        if let Some(index) = index_1.get(2) {
                            if let Ok(number) = index.parse::<u16>() {
                                normal_indexes.push(number);
                            }
                        }
                        if let Some(index) = index_2.get(2) {
                            if let Ok(number) = index.parse::<u16>() {
                                normal_indexes.push(number);
                            }
                        }
                        if let Some(index) = index_3.get(2) {
                            if let Ok(number) = index.parse::<u16>() {
                                normal_indexes.push(number);
                            }
                        }
                    }
                    _ => (),
                }
            }
            Err(_) => (),
        }
    }

    let mut out_vertices: Vec<Vertex> = Vec::new();
    let mut out_normals: Vec<Normal> = Vec::new();
    let mut out_index: Vec<u16> = Vec::new();

    for index in &vertex_indexes {
        let a: usize = (index - 1).into();
        out_vertices.push(*vertices.get(a).unwrap());
    }

    for index in &normal_indexes {
        let a: usize = (index - 1).into();
        out_normals.push(*normals.get(a).unwrap());
        out_index.push((out_index.len()).try_into().unwrap());
    }

    return Model {
        vertices: out_vertices,
        normals: out_normals,
        indexes: out_index,
    };
}
