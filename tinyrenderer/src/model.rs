use crate::geometry::Vector3F32;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use tgaimage::TGAImage;

pub struct Model {
    verts: Vec<Vector3F32>,
    faces: Vec<Vec<u32>>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let model_file = File::open(filename)?;
        let reader = BufReader::new(model_file);
        let mut verts = vec![];
        let mut faces = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                let mut data = line.splitn(4, " ").into_iter().skip(1);
                let mut coords = [f32::default(); 3];

                coords
                    .iter_mut()
                    .for_each(|elem| *elem = f32::from_str(data.next().unwrap()).unwrap());

                verts.push(Vector3F32::new(coords[0], coords[1], coords[2]));
            } else if line.starts_with("f ") {
                let data = line.split(" ").into_iter().skip(1);
                let mut face_coords: [u32; 3] = [0; 3];
                let mut index = 0;

                data.for_each(|s| {
                    face_coords[index] =
                        u32::from_str(s.split("/").into_iter().nth(0).unwrap()).unwrap() - 1;
                    index += 1;
                });

                assert_eq!(index, 3);
                faces.push(face_coords.into());
            }
        }

        Ok(Model { verts, faces })
    }

    pub fn load_texture(&mut self, _filename: &str, _image: &mut TGAImage) {
        unimplemented!()
    }

    pub fn n_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn n_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vert(&self, index: usize) -> &Vector3F32 {
        &self.verts[index]
    }

    pub fn vert_mut(&mut self, index: usize) -> &mut Vector3F32 {
        &mut self.verts[index]
    }

    pub fn face(&self, index: usize) -> &Vec<u32> {
        &self.faces[index]
    }

    pub fn face_mut(&mut self, index: usize) -> &mut Vec<u32> {
        &mut self.faces[index]
    }
}
