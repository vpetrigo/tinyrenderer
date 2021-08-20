use crate::geometry::Vector3F32;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
    str::{FromStr, SplitWhitespace},
};
use tgaimage::TGAImage;

#[derive(Default)]
struct ModelFace {
    verts_index: [u32; 3],
    uv: [u32; 3],
    norm: [u32; 3],
}

impl ModelFace {
    fn new(verts_index: [u32; 3]) -> Self {
        ModelFace {
            verts_index,
            uv: [0; 3],
            norm: [0; 3],
        }
    }
}

pub struct Model {
    verts: Vec<Vector3F32>,
    faces: Vec<ModelFace>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let model_file = File::open(filename)?;
        let reader = BufReader::new(model_file);
        let mut verts = vec![];
        let mut faces = vec![];

        for line in reader.lines() {
            let (_line, mut words) = match line {
                Ok(ref line) => (&line[..], line[..].split_whitespace()),
                Err(_e) => return Err(_e),
            };
            // let line = line.unwrap();

            match words.next() {
                Some("v") => Model::process_vertice(&mut words, &mut verts),
                Some("f") => Model::process_face(&mut words, &mut faces),
                Some("#") | None | Some(_) => continue,
            }
        }

        Ok(Model { verts, faces })
    }

    fn process_vertice(words: &mut SplitWhitespace, vertices: &mut Vec<Vector3F32>) {
        let mut coords = [f32::default(); 3];
        coords
            .iter_mut()
            .zip(words.into_iter())
            .enumerate()
            .for_each(|(i, (c, w))| {
                *c = f32::from_str(w).unwrap();
                assert!(i < 3);
            });
        vertices.push(Vector3F32::new_from_array(&coords));
    }

    fn process_face(words: &mut SplitWhitespace, faces: &mut Vec<ModelFace>) {
        let mut model_face = ModelFace::default();

        words.into_iter().enumerate().for_each(|(i, word)| {
            let mut it = word
                .split("/")
                .map(|n| u32::from_str(n).unwrap() - 1)
                .into_iter();

            model_face.verts_index[i] = it.next().unwrap();
            model_face.uv[i] = it.next().unwrap();
            model_face.norm[i] = it.next().unwrap();
            assert!(i < 3);
        });

        faces.push(model_face);
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

    pub fn face(&self, index: usize) -> &[u32; 3] {
        &self.faces[index].verts_index
    }

    pub fn face_mut(&mut self, index: usize) -> &mut [u32; 3] {
        &mut self.faces[index].verts_index
    }
}
