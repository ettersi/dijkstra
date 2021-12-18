// Binary heap with some extra bells and whistles specific to Dijkstra's algorithm
#[derive(Debug)]
pub struct DijkstraHeap<W> {
    h2v: Vec<usize>,
    v2h: Vec<usize>,
    v2d: Vec<W>,

    // Notation:
    //   v: Vertex index
    //   d: Distance (so `v2d[v]` is the distance from the start to vertex `v`)
    //   h: Heap index (so `v2h[v]` is the position of vertex `v` in the heap)
    //
    // Remarks:
    //  - `v2h` is the inverse permutation of `h2v`, i.e. `v2h[h2v] = 0..h2v.len()`.
    //  - `v2h[v]` is undefined if `v` is not currently in the heap.
}

use super::int_or_float::IntOrFloat;
impl<W: IntOrFloat> DijkstraHeap<W> {
    pub fn new(n: usize) -> DijkstraHeap<W> {
        let h2v = Vec::with_capacity(n);
        let v2h = vec![usize::MAX; n];
        let v2d = vec![W::max(); n];
        return DijkstraHeap{h2v,v2h,v2d};
    }

    fn bubble_up(&mut self, mut hc: usize, vc: usize, dc: W) {
        while hc > 0 {
            let hp = hc/2;
            let vp = self.h2v[hp];
            let dp = self.v2d[vp];
            if dc < dp {
                self.h2v[hc] = vp;
                self.v2h[vp] = hc;
                hc = hp;
            }
            else {
                break;
            }
        }
        self.h2v[hc] = vc;
        self.v2h[vc] = hc;
    }

    fn bubble_down(&mut self, mut hp: usize, vp: usize, dp: W) {
        let n = self.h2v.len();
        while 2*hp+2 < n {
            let hl = 2*hp+1;
            let vl = self.h2v[hl];
            let dl = self.v2d[vl];
            let hr = 2*hp+2;
            let vr = self.h2v[hr];
            let dr = self.v2d[vr];
            if dl < dr {
                if dl < dp {
                    self.h2v[hp] = vl;
                    self.v2h[vl] = hp;
                    hp = hl;
                }
                else {
                    break;
                }
            }
            else {
                if dr < dp {
                    self.h2v[hp] = vr;
                    self.v2h[vr] = hp;
                    hp = hr;
                }
                else {
                    break;
                }
            }
        }
        if 2*hp+1 < n {
            let hl = 2*hp+1;
            let vl = self.h2v[hl];
            let dl = self.v2d[vl];
            if dl < dp {
                self.h2v[hp] = vl;
                self.v2h[vl] = hp;
                hp = hl;
            }
        }
        self.h2v[hp] = vp;
        self.v2h[vp] = hp;
    }

    pub fn pop(&mut self) -> Option<usize> {
        let vr = *self.h2v.first()?;
        let h0 = 0;
        let v0 = self.h2v.pop().unwrap();
        if !self.h2v.is_empty() {
            let d0 = self.v2d[v0];
            self.bubble_down(h0,v0,d0);
        }
        return Some(vr);
    }

    pub fn update(&mut self, v: usize, d: W) {
        let mut h = self.v2h[v];
        if h == usize::MAX {
            h = self.h2v.len();
            self.h2v.push(v);
        }
        let old_d = self.v2d[v];
        self.v2d[v] = d;
        if d > old_d {
            self.bubble_down(h,v,d);
        }
        else {
            self.bubble_up(h,v,d);
        }
    }
}

impl<W> std::ops::Index<usize> for DijkstraHeap<W> {
    type Output = W;
    fn index(&self, v: usize) -> &W {
        return &self.v2d[v];
    }
}