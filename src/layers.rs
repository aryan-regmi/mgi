use raylib::prelude::Rectangle;

// TODO: Add tint as a field
#[derive(PartialEq, Debug)]
pub struct Layer<'l, T> {
    pub(crate) id: usize,
    pub(crate) objects: Vec<&'l T>,
    pub(crate) object_srcs: Vec<Option<Rectangle>>,
    pub(crate) object_dests: Vec<Rectangle>,
    pub(crate) object_rotations: Vec<f32>,
    pub(crate) visible: bool,
}

impl<'l, T> Default for Layer<'l, T> {
    fn default() -> Self {
        Self {
            id: 0,
            objects: Vec::new(),
            object_srcs: Vec::new(),
            object_dests: Vec::new(),
            object_rotations: Vec::new(),
            visible: true,
        }
    }
}

impl<'l, T> Layer<'l, T> {
    pub fn init(id: usize) -> Self {
        Self {
            id,
            objects: Vec::new(),
            object_srcs: Vec::new(),
            object_dests: Vec::new(),
            object_rotations: Vec::new(),
            visible: true,
        }
    }

    /// The objects are rendered in the order that they are added to the layer.
    pub fn add_obj(
        mut self,
        obj: &'l T,
        src: Option<Rectangle>,
        dest: Rectangle,
        rotation: f32,
    ) -> Self {
        self.objects.push(obj);
        self.object_srcs.push(src);
        self.object_dests.push(dest);
        self.object_rotations.push(rotation);

        self
    }

    /// The vector of layer objects will be rendered in the order of the vector.
    pub fn add_objects(mut self, objects: Vec<(&'l T, Option<Rectangle>, Rectangle, f32)>) -> Self {
        for obj in objects {
            self.objects.push(obj.0);
            self.object_srcs.push(obj.1);
            self.object_dests.push(obj.2);
            self.object_rotations.push(obj.3);
        }

        self
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub type TextureLayer<'l> = Layer<'l, &'l str>;
