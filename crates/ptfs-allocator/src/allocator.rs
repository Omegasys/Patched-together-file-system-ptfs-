use crate::extents::Extent;
use crate::fragmentation::FragmentationStats;

/// Simple extent-based allocator (first-fit for now)
#[derive(Debug)]
pub struct Allocator {
    free_list: Vec<Extent>,
    used: Vec<Extent>,
}

impl Allocator {
    pub fn new(total_size: u128) -> Self {
        Self {
            free_list: vec![Extent::new(0, total_size)],
            used: Vec::new(),
        }
    }

    /// Allocate an extent using first-fit strategy
    pub fn allocate(&mut self, size: u128) -> Option<Extent> {
        for i in 0..self.free_list.len() {
            let extent = &mut self.free_list[i];

            if extent.length >= size {
                let allocated = Extent::new(extent.start, size);

                extent.start += size;
                extent.length -= size;

                if extent.length == 0 {
                    self.free_list.remove(i);
                }

                self.used.push(allocated.clone());
                return Some(allocated);
            }
        }

        None
    }

    /// Free an extent and merge where possible
    pub fn free(&mut self, extent: Extent) {
        self.used.retain(|e| e != &extent);
        self.free_list.push(extent);
        self.merge_free_list();
    }

    fn merge_free_list(&mut self) {
        self.free_list.sort_by_key(|e| e.start);

        let mut merged = Vec::new();

        for extent in self.free_list.drain(..) {
            if let Some(last) = merged.last_mut() {
                if let Some(m) = last.merge(&extent) {
                    *last = m;
                    continue;
                }
            }
            merged.push(extent);
        }

        self.free_list = merged;
    }

    pub fn fragmentation(&self) -> FragmentationStats {
        FragmentationStats::calculate(&self.free_list)
    }
}
