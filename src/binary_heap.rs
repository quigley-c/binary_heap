#[derive(Clone)]
pub struct Heap {
    indicies: Vec<usize>,
    data: Vec<Vert>,
}

#[derive(Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    val: usize,
}

#[derive(Clone)]
pub struct Vert {
    name: usize,
    edges: Vec<usize>,
    label: i32,
}

pub fn init_heap(lines: Vec<String>) -> (Heap, Vec<Edge>) {
    // builds heap struct from a Vec of (space separated ints) as Strings
    let parts: Vec<usize> = lines[0]
        .split(" ")
        .filter_map(|w| w.parse().ok())
        .collect();

    let len_v: usize = parts[0];
    let len_e: usize = parts[1];
    let mut heap: Heap = Heap {
        indicies: vec![],
        data: vec![],
    };

    for i in 0..len_v {
        let v: Vert = Vert {
            name: i,
            edges: <Vec<usize>>::new(),
            label: i32::MAX
        };
        heap_insert(&mut heap, v);
    }

    let mut edges: Vec<Edge> = Vec::with_capacity(len_e);
    let mut edge_index: usize = 0;
    // rust syntax still confounds me
    for l in lines {
        let nums: Vec<_> = l.split(" ")
            .filter_map(|w| w.parse().ok())
            .collect();
        if nums.len() != 3 {
            // we don't need the first line
            continue;
        }
        let e: Edge = Edge {
            from: nums[0],
            to: nums[1],
            val: nums[2],
        };

        heap.data[e.from].edges.push(edge_index);
        edges.push(e);
        edge_index += 1;
    }

    return (heap, edges);
}

pub fn decrease_key(heap: &mut Heap, key: usize, val: i32) {
    // updates the node[key] in the tree with the new value.
    // ensures that the tree is rebalanced afterwrads with perc_up()
    let index = heap.indicies[key].clone();
    heap.data[index].label = val;
    perc_up(heap, index);
}

pub fn heap_extract(heap: &mut Heap) -> Result<Vert, &str>{
    // preps the heap for extraction
    // by swapping the smallest node to the front of the queue
    // this simplifies popping from the queue so we don't have
    // to recompute the indexes, letting the pop() function handle
    // the Vec management for us
    if heap.data.len() == 0 { return Err("nope"); };

    let start_i = 0;
    let len = heap.data.len();

    heap_swap(heap, start_i, len - 1);
    let v = heap.data.pop().unwrap();
    return Ok(v);
}

pub fn heap_insert(heap: &mut Heap, v: Vert) {
    // helper that manages the heap fields
    let index = heap.data.len();
    heap.indicies.push(index);
    heap.data.push(v.clone());

    perc_up(heap, index);
}

pub fn sift_down(heap: &mut Heap, i: usize) {
    // check the left and right children for smaller nodes, reverse of perc_up
    // useful after popping a node off the Vec structure.
    // Since only the front is guaranteed to be smallest the tree must be
    // rebalanced from the top down
    fn left(i:usize) -> usize {2*i+1}
    fn right(i:usize) -> usize {2*i+2}

    let mut smallest = i;
    if left(i) < heap.data.len() &&
        heap.data[left(i)].label < heap.data[smallest].label {
        smallest = left(i);
    }
    if right(i) < heap.data.len() &&
        heap.data[right(i)].label < heap.data[smallest].label {
        smallest = right(i);
    }
    if smallest != i {
        heap_swap(heap, i, smallest);
        sift_down(heap, smallest);
    }
}

fn heap_swap(heap: &mut Heap, vi: usize, pi: usize) {
    // simple swap func that updates the indexes Vec
    let tmp = heap.data[vi].clone();
    heap.data[vi] = heap.data[pi].clone();
    heap.data[pi] = tmp;

    let tmp = heap.indicies[heap.data[vi].name].clone();
    heap.indicies[heap.data[vi].name] = heap.indicies[heap.data[pi].name];
    heap.indicies[heap.data[pi].name] = tmp;
}

fn perc_up(heap: &mut Heap, vi: usize) {
    // Tree balancing from the bottom up. reverse of sift_down
    if vi == 0 { return; }
    let parent_i = (vi-1)/2;
    if heap.data[vi].label < heap.data[parent_i].label {
        heap_swap(heap, vi, parent_i);
        perc_up(heap, parent_i);
    }
}

