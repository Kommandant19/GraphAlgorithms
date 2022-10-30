
use std::collections::VecDeque;

struct Graph{
    // Liczba wierzchołków
    V: usize,

    // Macierz sąsiedztwa
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Konstruktor
    fn new(V: usize) -> Graph {
        // Tworzymy macierz sąsiedztwa
        let mut adj: Vec<Vec<usize>> = Vec::new();
        // Wypełniamy ją zerami
        for _ in 0..V {
            adj.push(Vec::new());
        }
        // Zwracamy graf
        Graph { V, adj }
    }

    fn printGraph(&self) {
        for i in 0..self.V {
            print!("{}: ", i);
            for j in 0..self.adj[i].len() {
                print!("{} ", self.adj[i][j]);
            }
            println!();
        }
    }

    // Funkcja dodająca krawędź do grafu
    fn addEdge(&mut self, vertex: usize, edge: usize) {
        self.adj[vertex].push(edge);
    }

    // Algorytm BFS
    fn BFS(&self, start: usize) {
        // Tworzymy kolejke
        let mut queue: VecDeque<usize> = VecDeque::new();
        // Tworzymy tablicę odwiedzonych wierzchołków
        let mut visited: Vec<bool> = vec![false; self.V];
        // Dodajemy wierzchołek startowy do kolejki
        queue.push_back(start);
        // Oznaczamy go jako odwiedzony
        visited[start] = true;
        // Dopóki kolejka nie jest pusta
        while !queue.is_empty() {
            // Pobieramy pierwszy element z kolejki
            let vertex = queue.pop_front().unwrap();
            // Wyświetlamy go
            print!("{} ", vertex);
            // Dla każdego sąsiada
            for i in 0..self.adj[vertex].len() {
                // Jeżeli nie odwiedziliśmy go wcześniej
                if !visited[self.adj[vertex][i]] {
                    // Oznaczamy go jako odwiedzonego
                    visited[self.adj[vertex][i]] = true;
                    // Dodajemy go do kolejki
                    queue.push_back(self.adj[vertex][i]);
                }
            }
        }


    }
    fn DFS(&self, start: usize) {
        // Zaznaczamy wszystkie wierzchołki jako nieodwiedzone
        let mut visited = vec![false; self.V];
        // Wywołujemy rekurencyjną funkcję DFS
        self.DFSUtil(start, &mut visited);
    }

    // Funkcja rekurencyjna DFS przyjmująca wierzchołek startowy i referencję do wektora odwiedzonych wierzchołków
    fn DFSUtil(&self, vertex: usize, visited: &mut Vec<bool>) {
        // Zaznaczamy wierzchołek jako odwiedzony
        visited[vertex] = true;
        println!("{}", vertex);
        // Iterujemy po wszystkich sąsiadach wierzchołka
        for &edge in self.adj[vertex].iter() {
            if !visited[edge] {
                self.DFSUtil(edge, visited);
            }
        }
    }
}

fn main() {
    /// PRZYKŁAD \\\

    println!("Graph1:");

    // Tworzymy graf
    let mut graph = Graph::new(4);

    // Dodajemy krawędzie
    graph.addEdge(0, 1);
    graph.addEdge(0, 2);
    graph.addEdge(1, 2);
    graph.addEdge(2, 0);
    graph.addEdge(2, 3);
    graph.addEdge(3, 3);

    // Wyświetlamy graf
    graph.printGraph();

    // Wykonujemy algorytm BFS dla wierzchołka 2
    println!("BFS:");
    graph.BFS(2);

    println!();


    println!("Graph2:");
    let mut graph2 = Graph::new(4);

    // Dodajemy krawędzie

    graph2.addEdge(0, 1);
    graph2.addEdge(0, 2);
    graph2.addEdge(1, 2);
    graph2.addEdge(2, 0);
    graph2.addEdge(2, 3);
    graph2.addEdge(3, 3);


    // Wyświetlamy graf
    graph2.printGraph();


    // Wykonujemy algorytm DFS dla wierzchołka 2
    println!("DFS:");
    graph.DFS(2);
}