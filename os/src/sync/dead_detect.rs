use alloc::{vec, vec::Vec};

pub const DEAD: isize = -0xdead;

type Matrix = Vec<Vec<usize>>;
type Vector = Vec<usize>;

/// Resource Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// Mutex
    Mutex,
    /// Semaphore
    Semaphore(usize),
}

/// DeadDetector
#[derive(Debug)]
pub struct DeadDetector {
    /// is enabled?
    pub enabled: bool,
    inner: DeadDetectorImpl,
}

impl Default for DeadDetector {
    fn default() -> Self {
        DeadDetector::new()
    }
}

impl DeadDetector {
    /// new
    pub fn new() -> Self {
        DeadDetector {
            enabled: false,
            inner: DeadDetectorImpl::new(),
        }
    }

    /// add resource of mutex and semaphore
    pub fn add_resource(&mut self, id: usize, res: ResourceType) {
        // always record in case
        self.inner.add_resource(id, res);
    }

    /// alloc resource and check safety
    pub fn alloc_resource(&mut self, tid: usize, res_id: usize) -> Result<(), isize> {
        if self.enabled {
            return self.inner.alloc_resource(tid, res_id);
        } else {
            let _ = self.inner.alloc_resource(tid, res_id);
            Ok(())
        }
    }

    /// free resource
    pub fn free_resource(&mut self, tid: usize, res_id: usize) -> Result<(), isize> {
        if self.enabled {
            return self.inner.free_resource(tid, res_id);
        } else {
            let _ = self.inner.free_resource(tid, res_id);
            Ok(())
        }
    }

    /// add thread
    pub fn add_thread(&mut self, tid: usize) {
        self.inner.add_thread(tid);
    }

    /// remove thread
    pub fn remove_thread(&mut self, tid: usize) {
        self.inner.remove_thread(tid);
    }

    /// is safe
    pub fn is_safe(&self) -> Option<bool> {
        if self.enabled {
            Some(self.inner.is_safe())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DeadDetectorImpl {
    // Maps Resource ID (for mutex or semaphore) to internal resource index
    resource_map: Vec<(usize, ResourceType)>,
    avail: Vector, // m (available resources)
    alloc: Matrix, // n*m (allocated resources per thread)
    need: Matrix,  // n*m (needed resources per thread)
    max: Vector,   // m (max resources available)
}

impl DeadDetectorImpl {
    pub fn new() -> Self {
        DeadDetectorImpl {
            resource_map: Vec::new(),
            avail: Vec::new(),
            alloc: Vec::new(),
            need: Vec::new(),
            max: Vec::new(),
        }
    }

    // Find the internal index of a resource or add it if it doesn't exist
    fn get_resource_index(&mut self, res_id: usize, res_type: ResourceType) -> usize {
        for (idx, &(id, tp)) in self.resource_map.iter().enumerate() {
            if id == res_id {
                match (tp, res_type) {
                    (ResourceType::Mutex, ResourceType::Mutex) => return idx,
                    (ResourceType::Semaphore(_), ResourceType::Semaphore(_)) => return idx,
                    _ => continue, // Different types, keep looking
                }
            }
        }

        // Not found, add new resource
        let index = self.resource_map.len();
        self.resource_map.push((res_id, res_type));

        self.avail.push(0);
        self.max.push(0);

        for thread in self.alloc.iter_mut() {
            thread.push(0);
        }
        for thread in self.need.iter_mut() {
            thread.push(0);
        }

        index
    }

    pub fn add_resource(&mut self, res_id: usize, res: ResourceType) {
        let count = match res {
            ResourceType::Mutex => 1,
            ResourceType::Semaphore(count) => count,
        };

        let internal_idx = self.get_resource_index(res_id, res);
        self.avail[internal_idx] = count;
        self.max[internal_idx] = count;
    }

    /// alloc
    pub fn alloc_resource(&mut self, tid: usize, res_id: usize) -> Result<(), isize> {
        // Ensure thread exists
        self.ensure_thread(tid);

        // Find the resource type from existing resources
        let res_type = self.find_resource_type(res_id)?;

        let internal_idx = self.get_resource_index(res_id, res_type);
        log::debug!(
            "alloc_resource start: tid[{}] res_id[{}] res_type[{:?}] ",
            tid,
            res_id,
            res_type,
        );

        // Update need
        match res_type {
            ResourceType::Mutex => {
                self.need[tid][internal_idx] = 1;
            }
            ResourceType::Semaphore(max) => {
                if self.need [tid][internal_idx] + 1 >= max {
                    return Err(DEAD);
                }
                self.need[tid][internal_idx]  = self.need[tid][internal_idx] + 1;
            }
        }

        if self.avail[internal_idx] < 1 {
            return Err(DEAD);
        }

        log::debug!(
            "before alloc_resource: alloc[{}] avail[{}] need[{}]",
            self.alloc[tid][internal_idx],
            self.avail[internal_idx],
            self.need[tid][internal_idx]
        );
        // Simulate allocation
        self.avail[internal_idx] -= 1;
        self.alloc[tid][internal_idx] += 1;
        log::debug!(
            "after alloc_resource: alloc[{}] avail[{}] need[{}]",
            self.alloc[tid][internal_idx],
            self.avail[internal_idx],
            self.need[tid][internal_idx]
        );

        // Check if allocation is safe
        if self.is_safe() {
            Ok(())
        } else {
            // restore
            self.avail[internal_idx] += 1;
            self.alloc[tid][internal_idx] -= 1;
            Err(DEAD)
        }
    }

    /// free
    pub fn free_resource(&mut self, tid: usize, res_id: usize) -> Result<(), isize> {
        // Ensure thread exists
        if tid >= self.alloc.len() {
            return Err(-1);
        }

        let res_type = self.find_resource_type(res_id)?;
        let internal_idx = self.get_resource_index(res_id, res_type);

        if self.alloc[tid][internal_idx] < 1 {
            return Err(-1);
        }

        self.avail[internal_idx] += 1;
        self.alloc[tid][internal_idx] -= 1;

        Ok(())
    }

    // Find the type of an existing resource by ID
    fn find_resource_type(&self, res_id: usize) -> Result<ResourceType, isize> {
        for &(id, res_type) in &self.resource_map {
            if id == res_id {
                return Ok(res_type);
            }
        }
        Err(-1)
    }

    // Ensure a thread exists in our matrices
    fn ensure_thread(&mut self, tid: usize) {
        if tid >= self.alloc.len() {
            self.add_thread(tid);
        }
    }

    /// add thread
    pub fn add_thread(&mut self, tid: usize) {
        while self.alloc.len() <= tid {
            let resource_count = self.avail.len();
            self.alloc.push(vec![0; resource_count]);

            let need_vec = vec![0; resource_count];
            self.need.push(need_vec);
        }
    }

    pub fn remove_thread(&mut self, tid: usize) {
        if tid < self.alloc.len() {
            for internal_idx in 0..self.avail.len() {
                self.avail[internal_idx] += self.alloc[tid][internal_idx];

                // Clear
                self.alloc[tid][internal_idx] = 0;
                self.need[tid][internal_idx] = 0;
            }
        }
    }

    pub fn is_safe(&self) -> bool {
        // Empty system is safe
        if self.alloc.is_empty() || self.avail.is_empty() {
            return true;
        }

        // resource count
        let m = self.avail.len();
        // thread count
        let n = self.alloc.len();

        let mut work = self.avail.clone();
        let mut finish = vec![false; n];
        let remain_need: Matrix = (0..n)
            .map(|tid| {
                (0..m)
                    .map(|res_id| self.need[tid][res_id] - self.alloc[tid][res_id])
                    .collect()
            })
            .collect();

        while {
            let mut progress = false;

            finish
                .iter_mut()
                .filter(|done| !**done)
                .enumerate()
                .for_each(|(i, done)| {
                    let can_free = (0..m).all(|j| remain_need[i][j] <= work[j]);

                    if can_free {
                        work.iter_mut()
                            .zip(&self.alloc[i])
                            .for_each(|(w, &a)| *w += a);
                        *done = true;
                        progress = true;
                    }
                });

            progress
        } {}

        finish.iter().all(|&f| f)
    }
}
