/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // 创建一个拥有 n 个顶点的图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 向图中添加一条边
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // 从指定的起始点开始进行广度优先搜索，返回访问节点的顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()];  // 记录每个节点是否访问过
        let mut queue = VecDeque::new();  // 队列用于广度优先搜索
        let mut visit_order = Vec::new();  // 记录访问顺序
        
        // 将起始节点加入队列并标记为已访问
        queue.push_back(start);
        visited[start] = true;
        
        while let Some(node) = queue.pop_front() {
            visit_order.push(node);  // 记录访问的节点
            
            // 访问当前节点的所有相邻节点
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor] = true;  // 标记为已访问
                }
            }
        }
        
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
