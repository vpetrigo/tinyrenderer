use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
    str::{FromStr, SplitWhitespace},
};

use tgaimage::{TGAColor, TGAImage};

use crate::geometry::{UVMapF32, Vector2Int, Vector3F32, XAxis, YAxis};

#[derive(Default)]
struct ModelFace {
    verts_index: [u32; 3],
    uv_index: [u32; 3],
    norm_index: [u32; 3],
}

pub struct Model {
    verts: Vec<Vector3F32>,
    faces: Vec<ModelFace>,
    normals: Vec<Vector3F32>,
    uvs: Vec<UVMapF32>,
    diffusemap: Option<TGAImage>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let model_file = File::open(filename)?;
        let reader = BufReader::new(model_file);
        let mut verts = vec![];
        let mut faces = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let diffusemap = None;

        for line in reader.lines() {
            let (_line, mut words) = match line {
                Ok(ref line) => (&line[..], line[..].split_whitespace()),
                Err(_e) => return Err(_e),
            };

            match words.next() {
                Some("v") => Model::process_vertice(&mut words, &mut verts),
                Some("f") => Model::process_face(&mut words, &mut faces),
                Some("vn") => Model::process_normal(&mut words, &mut normals),
                Some("vt") => Model::process_texture(&mut words, &mut uvs),
                Some("#") | None | Some(_) => continue,
            }
        }

        Ok(Model {
            verts,
            faces,
            normals,
            uvs,
            diffusemap,
        })
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
            model_face.uv_index[i] = it.next().unwrap();
            model_face.norm_index[i] = it.next().unwrap();
            assert!(i < 3);
        });

        faces.push(model_face);
    }

    fn process_normal(words: &mut SplitWhitespace, normals: &mut Vec<Vector3F32>) {
        let mut normal_vals = [0.0f32; 3];

        words
            .into_iter()
            .zip(normal_vals.iter_mut())
            .enumerate()
            .for_each(|(i, (w, c))| {
                *c = f32::from_str(w).unwrap();
                assert!(i < 3);
            });

        normals.push(Vector3F32::new_from_array(&normal_vals))
    }

    fn process_texture(words: &mut SplitWhitespace, uv: &mut Vec<UVMapF32>) {
        let mut texture_uv = [0.0f32; 3];

        words
            .into_iter()
            .zip(texture_uv.iter_mut())
            .enumerate()
            .for_each(|(i, (w, c))| {
                *c = f32::from_str(w).unwrap();
                assert!(i < 3);
            });

        uv.push(UVMapF32 {
            u: texture_uv[0],
            v: texture_uv[1],
            w: texture_uv[2],
        })
    }

    pub fn load_texture(&mut self, filename: &str) -> io::Result<()> {
        if self.diffusemap.is_some() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }

        self.diffusemap = Some(TGAImage::read_tga_file(filename).expect("Unable to read file"));

        Ok(())
    }

    pub fn n_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn n_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn n_textures(&self) -> usize {
        self.uvs.len()
    }

    pub fn n_normals(&self) -> usize {
        self.normals.len()
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

    pub fn diffuse(&self, uv: Vector2Int) -> Option<TGAColor> {
        if let Some(ref diffusemap) = self.diffusemap {
            return Some(diffusemap.get(uv.get_x() as u32, uv.get_y() as u32));
        }

        None
    }

    pub fn uv(&self, face_index: usize, vert_index: usize) -> Vector2Int {
        if let Some(ref diffusemap) = self.diffusemap {
            let index = self.faces[face_index].uv_index[vert_index] as usize;

            return Vector2Int::new(
                (self.uvs[index].u * diffusemap.get_width() as f32) as i32,
                (self.uvs[index].v * diffusemap.get_height() as f32) as i32,
            );
        }

        panic!("Invalid access to UV buffer");
    }
}
