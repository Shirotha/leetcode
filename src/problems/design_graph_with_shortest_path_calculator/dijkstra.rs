/*
    There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1. The edges of the graph are initially represented by the given array edges where edges[i] = [from_i, to_i, edgeCost_i] meaning that there is an edge from from_i to to_i with the cost edgeCost_i.

    Implement the Graph class:

    Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
    addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
    int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.

    Constraints:

    1 <= n <= 100
    0 <= edges.length <= n * (n - 1)
    edges[i].length == edge.length == 3
    0 <= from_i, to_i, from, to, node1, node2 <= n - 1
    1 <= edgeCost_i, edgeCost <= 10^6
    There are no repeated edges and no self-loops in the graph at any point.
    At most 100 calls will be made for addEdge.
    At most 100 calls will be made for shortestPath.
 */

 use std::collections::BinaryHeap;

 struct Edge(usize, i32);
 struct Item(usize, i32);
 impl Ord for Item { fn cmp(&self, other: &Self) -> std::cmp::Ordering { other.1.cmp(&self.1) } }
 impl PartialOrd for Item { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
 impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.1 == other.1 } }
 impl Eq for Item {}
 
 struct Graph {
     edges: Vec<Vec<Edge>>,
     queue: BinaryHeap<Item>,
     lookup: Vec<i32>
 }
 impl Graph {
     fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
         let n = n as usize;
         let mut adj = Vec::new();
         adj.resize_with(n, Vec::new);
         let mut g = Graph { edges: adj, queue: BinaryHeap::new(), lookup: vec![i32::MAX; n] };
         for edge in edges { g.add_edge(edge); }
         g
     }
     #[inline] fn add_edge(&mut self, edge: Vec<i32>) {
         let (from, to, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
         self.edges[from].push(Edge(to, cost));
     }
     fn shortest_path(&mut self, node1: i32, node2: i32) -> i32 {
         let (node1, node2) = (node1 as usize, node2 as usize);
         self.queue.clear(); self.lookup.fill(i32::MAX);
         self.queue.push(Item(node1, 0)); self.lookup[node1] = 0;
         while let Some(Item(node, cost)) = self.queue.pop() {
             let prev = self.lookup[node];
             if prev != cost { continue; }
             if node == node2 { return prev; }
             for &Edge(next, cost) in &self.edges[node] {
                 let cost = prev + cost;
                 let prev = self.lookup[next];
                 if prev == i32::MAX || cost < prev {
                     self.queue.push(Item(next, cost));
                     self.lookup[next] = cost;
                 }
             }
         }
         -1
     }
 }
 
 #[cfg(test)]
 mod test {
     use super::*;
 
     fn run(n: i32, edges: &[[i32; 3]], cmds: &[&str], args: &[&[i32]], results: &[i32]) {
         let mut g = Graph::new(n, edges.iter().map( |e| e.to_vec() ).collect());
         for i in 0..cmds.len() {
             match cmds[i] {
                 "addEdge" => g.add_edge(args[i].to_vec()),
                 "shortestPath" => assert_eq!(g.shortest_path(args[i][0], args[i][1]), results[i]),
                 _ => (),
             }
         }
     }
 
     /*
         Input
         ["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
         [[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
         Output
         [null, 6, -1, null, 6]
 
         Explanation
         Graph g = new Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
         g.shortestPath(3, 2); // return 6. The shortest path from 3 to 2 in the first diagram above is 3 -> 0 -> 1 -> 2 with a total cost of 3 + 2 + 1 = 6.
         g.shortestPath(0, 3); // return -1. There is no path from 0 to 3.
         g.addEdge([1, 3, 4]); // We add an edge from node 1 to node 3, and we get the second diagram above.
         g.shortestPath(0, 3); // return 6. The shortest path from 0 to 3 now is 0 -> 1 -> 3 with a total cost of 2 + 4 = 6.     
      */
     #[test]
     fn example1() {
         run(4, &[[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]],
             &["shortestPath", "shortestPath", "addEdge", "shortestPath"],
             &[&[3, 2], &[0, 3], &[1, 3, 4], &[0, 3]],
             &[6, -1, -2, 6]);
     }
 
     /*
         Input
         ["Graph","addEdge","shortestPath","addEdge","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","shortestPath","addEdge","addEdge","shortestPath","addEdge","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","addEdge","addEdge","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","addEdge"]
         [[18,[[8,12,835881],[6,1,886222],[8,9,152139],[4,15,475979],[5,15,903985],[12,7,435256],[3,11,115877],[14,2,669007],[15,12,503987],[13,9,773256],[2,13,643974],[12,5,42565],[0,9,726934],[9,8,369110],[13,10,727485],[16,0,842868],[0,13,836101],[4,12,645669],[12,14,353649],[0,1,501402],[3,13,131383],[15,9,919433],[13,11,652190],[9,4,501551],[13,12,772479],[13,1,602418],[5,3,192091],[12,0,66326],[8,4,841136],[3,1,305879],[2,9,953806],[6,13,690575],[1,12,901363],[1,5,658225],[0,2,751532],[14,16,17590],[15,3,665278],[2,8,784019],[4,3,586413],[3,12,631462],[5,13,360949],[1,17,686861],[9,3,112100],[4,1,159862],[7,13,863940],[1,3,859524],[10,6,795021],[17,2,489450],[3,8,930965],[4,10,573998],[1,15,60334],[2,0,624060],[8,6,708518],[17,13,446713],[7,4,361258],[14,5,489098],[1,4,147944],[0,7,987717],[6,5,518191],[13,16,301057],[0,6,725177],[3,17,515457],[16,3,456018],[3,15,871393],[11,14,584427],[17,8,569473],[8,16,598786],[8,1,961881],[12,16,51206],[15,10,622641],[10,14,573146]]],[[9,2,775475]],[12,0],[[1,8,10688]],[16,16],[[7,10,194719]],[[6,15,10240]],[[4,8,7274]],[[5,1,5126]],[[14,12,230]],[3,11],[8,4],[[13,8,126]],[3,16],[13,3],[9,12],[[5,17,311469]],[7,0],[[11,2,659004]],[[12,3,470391]],[[11,5,112]],[[3,0,771142]],[[14,11,88]],[12,4],[17,17],[16,4],[15,8],[[9,6,176760]],[10,11],[[17,9,293661]],[10,13],[[9,12,54]],[[17,12,49]],[4,2],[[2,11,598588]],[5,17],[5,10],[[10,7,472952]],[0,15],[2,3],[[12,1,5]],[[9,16,147239]],[10,12],[[6,3,337163]],[10,7],[8,3],[17,10],[15,2],[[1,13,362779]],[3,14],[[8,17,112504]],[[16,2,2]],[13,17],[2,2],[15,12],[14,0],[[16,1,1]],[11,6],[0,9],[12,5],[0,15],[[3,5,1]],[[6,14,1]],[[13,15,1]],[[17,3,1]],[[8,2,1]],[5,1],[2,12],[7,8],[6,2],[12,5],[[7,0,962638]],[7,13],[13,12],[13,9],[6,13],[15,12],[0,16],[3,5],[[5,0,730643]],[9,16],[13,9],[[15,11,1]]]
         Output
         [null,null,66326,null,0,null,null,null,null,null,115877,653690,null,432440,264365,743562,null,834421,null,null,null,null,null,195635,0,725077,562366,null,573234,null,896820,null,null,934888,null,311469,688101,null,561736,790791,null,null,573376,null,472952,264239,630024,1321845,null,637351,null,null,112630,0,503987,66556,null,344825,664229,42565,561736,null,null,null,null,null,5126,727067,368532,10925,42565,null,612420,112679,152265,254812,503987,675849,1,null,51260,152265,null]
      */
         #[test]
         fn example2() {
             run(18, &[[8,12,835881],[6,1,886222],[8,9,152139],[4,15,475979],[5,15,903985],[12,7,435256],[3,11,115877],[14,2,669007],[15,12,503987],[13,9,773256],[2,13,643974],[12,5,42565],[0,9,726934],[9,8,369110],[13,10,727485],[16,0,842868],[0,13,836101],[4,12,645669],[12,14,353649],[0,1,501402],[3,13,131383],[15,9,919433],[13,11,652190],[9,4,501551],[13,12,772479],[13,1,602418],[5,3,192091],[12,0,66326],[8,4,841136],[3,1,305879],[2,9,953806],[6,13,690575],[1,12,901363],[1,5,658225],[0,2,751532],[14,16,17590],[15,3,665278],[2,8,784019],[4,3,586413],[3,12,631462],[5,13,360949],[1,17,686861],[9,3,112100],[4,1,159862],[7,13,863940],[1,3,859524],[10,6,795021],[17,2,489450],[3,8,930965],[4,10,573998],[1,15,60334],[2,0,624060],[8,6,708518],[17,13,446713],[7,4,361258],[14,5,489098],[1,4,147944],[0,7,987717],[6,5,518191],[13,16,301057],[0,6,725177],[3,17,515457],[16,3,456018],[3,15,871393],[11,14,584427],[17,8,569473],[8,16,598786],[8,1,961881],[12,16,51206],[15,10,622641],[10,14,573146]],
                 &["addEdge","shortestPath","addEdge","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","shortestPath","addEdge","addEdge","shortestPath","addEdge","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","addEdge","addEdge","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","addEdge","addEdge","addEdge","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","shortestPath","addEdge","shortestPath","shortestPath","addEdge"],
                 &[&[9,2,775475],&[12,0],&[1,8,10688],&[16,16],&[7,10,194719],&[6,15,10240],&[4,8,7274],&[5,1,5126],&[14,12,230],&[3,11],&[8,4],&[13,8,126],&[3,16],&[13,3],&[9,12],&[5,17,311469],&[7,0],&[11,2,659004],&[12,3,470391],&[11,5,112],&[3,0,771142],&[14,11,88],&[12,4],&[17,17],&[16,4],&[15,8],&[9,6,176760],&[10,11],&[17,9,293661],&[10,13],&[9,12,54],&[17,12,49],&[4,2],&[2,11,598588],&[5,17],&[5,10],&[10,7,472952],&[0,15],&[2,3],&[12,1,5],&[9,16,147239],&[10,12],&[6,3,337163],&[10,7],&[8,3],&[17,10],&[15,2],&[1,13,362779],&[3,14],&[8,17,112504],&[16,2,2],&[13,17],&[2,2],&[15,12],&[14,0],&[16,1,1],&[11,6],&[0,9],&[12,5],&[0,15],&[3,5,1],&[6,14,1],&[13,15,1],&[17,3,1],&[8,2,1],&[5,1],&[2,12],&[7,8],&[6,2],&[12,5],&[7,0,962638],&[7,13],&[13,12],&[13,9],&[6,13],&[15,12],&[0,16],&[3,5],&[5,0,730643],&[9,16],&[13,9],&[15,11,1]],
                 &[-2,66326,-2,0,-2,-2,-2,-2,-2,115877,653690,-2,432440,264365,743562,-2,834421,-2,-2,-2,-2,-2,195635,0,725077,562366,-2,573234,-2,896820,-2,-2,934888,-2,311469,688101,-2,561736,790791,-2,-2,573376,-2,472952,264239,630024,1321845,-2,637351,-2,-2,112630,0,503987,66556,-2,344825,664229,42565,561736,-2,-2,-2,-2,-2,5126,727067,368532,10925,42565,-2,612420,112679,152265,254812,503987,675849,1,-2,51260,152265,-2]);
         }
 }