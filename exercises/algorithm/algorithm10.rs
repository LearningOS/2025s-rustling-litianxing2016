/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (node1, node2, weight) = edge;
        
        // 确保两个节点都存在
        self.add_node(node1);
        self.add_node(node2);
        
        // 获取邻接表可变引用
        let table = self.adjacency_table_mutable();
        
        // 添加 node1 -> node2 的边
        table.entry(node1.to_string())
            .and_modify(|e| e.push((node2.to_string(), weight)))
            .or_insert(vec![(node2.to_string(), weight)]);
        
        // 添加 node2 -> node1 的边（无向图）
        table.entry(node2.to_string())
            .and_modify(|e| e.push((node1.to_string(), weight)))
            .or_insert(vec![(node1.to_string(), weight)]);
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        let node_str = node.to_string();
        if self.contains(node) {
            false  // 节点已存在
        } else {
            self.adjacency_table_mutable()
                .insert(node_str, Vec::new());  // 初始化空邻居列表
            true   // 节点新增成功
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (node1, node2, weight) = edge;
        self.add_node(node1);
        self.add_node(node2);
        
        let table = self.adjacency_table_mutable();
        // 默认实现仅添加单向边（需根据图类型覆盖）
        table.entry(node1.to_string())
            .and_modify(|neighbors| neighbors.push((node2.to_string(), weight)))
            .or_insert(vec![(node2.to_string(), weight)]);
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}