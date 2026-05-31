#[derive(Clone)]
pub struct NumaNode {
    pub node_id: u8,
    pub memory_start: u64,
    pub memory_end: u64,
    pub cpu_cores: Vec<usize>,
}

#[derive(Clone)]
pub struct NumaTopology {
    pub nodes: Vec<NumaNode>,
    pub enabled: bool,
}

impl NumaTopology {
    pub fn new() -> Self {
        Self {
            nodes: vec![
                NumaNode {
                    node_id: 0,
                    memory_start: 0,
                    memory_end: 0x7FFFFFFF, // 2GB
                    cpu_cores: vec![0, 1],
                },
                NumaNode {
                    node_id: 1,
                    memory_start: 0x80000000,
                    memory_end: 0xFFFFFFFF, // 2GB
                    cpu_cores: vec![2, 3],
                },
            ],
            enabled: false,
        }
    }
    
    pub fn get_node_for_address(&self, address: u64) -> Option<&NumaNode> {
        if !self.enabled {
            return None;
        }
        
        self.nodes.iter().find(|node| {
            address >= node.memory_start && address <= node.memory_end
        })
    }
    
    pub fn get_node_for_core(&self, core_id: usize) -> Option<&NumaNode> {
        if !self.enabled {
            return None;
        }
        
        self.nodes.iter().find(|node| node.cpu_cores.contains(&core_id))
    }
    
    pub fn latency_between(&self, from_core: usize, to_node: u8) -> u32 {
        if !self.enabled {
            return 1;
        }
        
        let from_node = self.get_node_for_core(from_core);
        match (from_node, to_node) {
            (Some(node), target) if node.node_id == target => 1,   // Same node
            (Some(_), _) => 3,                                      // Different node
            (None, _) => 2,                                         // Unknown
        }
    }
}