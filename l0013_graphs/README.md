# Graphs

## What is a node?

- A node is an abstract data type with two things. 
- First, a node stores data. This data can be whatever you want - an integer, a boolean, a hash map, your own custom objects, or all of the above. 
- Second, a node stores pointers to other nodes.

## What is a graph?

- A graph is any collection of nodes and their pointers to other nodes. 
- Linked lists and trees are both types of graphs. 
- As a topic, graphs are extremely broad. There is an entire field of study dedicated to graphs called [graph theory](https://en.wikipedia.org/wiki/Graph_theory).

    <img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/uploads/20200630111809/graph18.jpg"/>

## Graphs pratical applications

- Graphs are part of our everyday lives. Without even trying too hard, you can model literally anything as a graph. Some examples of practical applications would be: 
  - Social networks like Facebook, Twitter, Instagram, and TikTok;
  - The roads in cities;
  - Circuit boards;
  - Internet network traffic;
  - Crypto blockchains;
  - Models in biology, and so much more.

## Terminology

- **Vertices**: another term for nodes.
- **Edges**: connections between the nodes.
  - **Directed edges** mean that you can only traverse in one direction: `A -> B`.
  - **Undirected edges** mean that you can traverse in both directions: `A -> B` and `B -> A`.
- **Connected component**: group of nodes that are connected by edges.
- **Node's indegree**: the number of edges that can be used to reach the node.
- **Node's outdegree**: the number of edges that can be used to leave the node.
> In binary trees, all nodes except the root had an indegree of 1 (due to their parent). All nodes have an outdegree of 0, 1, or 2. An outdegree of 0 means that it is a leaf. Specific to trees, we used the parent/child terms instead of "neighbors".
- **Neighbors**: Nodes that are connected by an edge.
- **Cyclic** means that the graph has a cycle.
- **Acyclic** means that it doesn't


## Graphical representation

<img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/undirectedgraph.png"/>

- Nodes of a graph are also called vertices. 
- The pointers that connect them are called edges. 
- In graphical representations, nodes/vertices are usually represented with circles and the edges are lines/arrows that connect the circles (we saw this in the linked lists chapter).

### Directed and Undirected graphs
- Directed edges will be arrows between nodes.
- Undirected edges will just be straight lines between nodes.
    <img style="max-height: 300px" src="https://media.geeksforgeeks.org/wp-content/uploads/20200630114438/directed.jpg"/>

    > In binary trees, the edges were directed. Binary trees are directed graphs. You can't access a node's parent, only its children. Once you move to a child, you can't move back.

### Connected component in a graph

- There are 2 different connected components in the following graph: {0, 1, 2} and {3, 4}.
    <img style="max-height: 200px" src="https://media.geeksforgeeks.org/wp-content/uploads/20220905132251/graph.jpg"/>

    > In binary trees, there must only be one connected component (all nodes are reachable from the root).

### Cyclic and acyclic graphs

<img style="max-height: 300px" src="https://media.geeksforgeeks.org/wp-content/uploads/20200630122225/cyclic.jpg"/>

> Binary trees by definition cannot have a cycle.

### Node's indegree and outdegree

#### How to calculate Indegree of a node?
<img style="max-height: 200px" src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/digraph.png"/>

Calculate the number of arrows pointing towards the node. For e.g. for vertex `V4` there are two arrows pointing toward the node with edges as `e3` and `e4`, therefore Indegree `(V4) = 2`.

### How to calculate Outdegree of a Node?
<img style="max-height: 200px" src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/digraph.png"/>

To determine a vertex’s outdegree in a directed graph, one must count the number of directed edges that leave from that vertex.

- Outdegree `(V2) = 2` as there are two outgoing edges `e2` and `e4`.
- Outdegree `(V3) = 1` as there is only one outgoing edge `e3`.
- Outdegree `(V4) = 1` as there is only one outgoing edge `e5`.
- Outdegree `(V5) = 2` as there are two outgoing edges `e6` and `e7`.

## How are graphs given in algorithm problems?

- In linked list problems, the **head of the linked list** is given. 
- In binary tree problems, the **root of the tree** is given. 
- In graph problems, **only information about a graph** is given. There are multiple common formats that this information can come in. We will take a look at a few.
> An important thing to understand is that with linked lists and binary trees, you are literally given objects in memory that contain data and pointers. **With graphs, the graph doesn't literally exist in memory**.

> In fact, **only the "idea" of the graph exists**. The input will give you some information about it, and it's up to you to figure out how to represent and traverse the graph with code.

### First input format: array of edges

- In this input format, the input will be a 2D array. 
- Each element of the array will be in the form `[x, y]`, which indicates that there is an edge between `x` and `y`. 
- The problem may have a story for these edges - using the cities example, the story would be something like "`[x, y]` means there is a highway connecting city `x` and city `y`".
- Before starting the traversal, we can pre-process the input so that we can easily find all neighbors of any given node. Ideally, you want a data structure where you can give node as an argument and be returned a list of neighbors. **The easiest way to accomplish this is using a hash map.**

#### Rust
```Rust
fn build_graph(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in edges {
        let entry = graph.entry(edge[0]).or_insert(vec![]);
        entry.push(edge[1]);
    }
    graph
}
```

#### Typescript
```Typescript
function buildGraph(edges: number[][]): Map<number, number[]> {
    let graph: Map<number, number[]> = new Map()
    for (const [x, y] of edges) {
        const entry = graph.get(x) || []
        entry.push(y)
        graph.set(x, entry)
    }
    return graph
}
```

#### Python
```Python
from collections import defaultdict

def build_graph(edges):
    graph = defaultdict(list)
    for x, y in edges:
        graph[x].append(y)
    return graph
```

### Second input format: adjacency list






