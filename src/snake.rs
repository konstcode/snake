struct Snake {
    head: Section,
    tail: Vec<Section>,
}

struct Section (usize, usize);