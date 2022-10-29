use std::collections::VecDeque;

struct Graph{
    // Liczba wierzchołków
    V: usize,

    // Macierz sąsiedztwa
    adj: Vec<Vec<usize>>,
}

impl Graph{

    // Konstruktor
    fn new(V: usize) -> Graph{
        // Tworzymy macierz sąsiedztwa
        let mut adj: Vec<Vec<usize>> = Vec::new();
        // Wypełniamy ją zerami
        for _ in 0..V{
            adj.push(Vec::new());
        }
        // Zwracamy graf
        Graph{V, adj}
    }

    // Funkcja dodająca krawędź do grafu
    fn addEdge(&mut self, vertex: usize, edge: usize){
        self.adj[vertex].push(edge);
    }

    // Algorytm BFS
    fn BFS(&self, start: usize){

        // Zaznaczamy wszystkie wierzchołki jako nieodwiedzone
        let mut visited = vec![false; self.V];

        // Tworzymy kolejke do przechowywania kolejnych wierzchołków
        let mut queue = VecDeque::new();

        // Zaznaczamy wierzchołek startowy jako odwiedzony i dodajemy go do kolejki
        visited[start] = true;
        queue.push_back(start);

        // Dopóki kolejka nie jest pusta wykonujemy pętlę
        while !queue.is_empty(){

            // Pobieramy pierwszy wierzchołek z kolejki
            let vertex = queue.pop_front().unwrap();
            println!("{}", vertex);

            // Iterujemy po wszystkich sąsiadach wierzchołka
            for &edge in self.adj[vertex].iter(){
                if !visited[edge]{
                    visited[edge] = true;
                    queue.push_back(edge);
                }
            }
        }
    }
    fn DFS(&self, start: usize){
        // Zaznaczamy wszystkie wierzchołki jako nieodwiedzone
        let mut visited = vec![false; self.V];

        // Tworzymy stos do przechowywania kolejnych wierzchołków
        let mut stack = VecDeque::new();

        // Zaznaczamy wierzchołek startowy jako odwiedzony i dodajemy go do kolejki
        visited[start] = true;

        // Dodajemy wierzchołek do stosu
        stack.push_back(start);

        // Dopóki stos nie jest pusty wykonujemy pętlę
        while !stack.is_empty(){
            // Pobieramy pierwszy wierzchołek ze stosu
            let vertex = stack.pop_back().unwrap();
            println!("{}", vertex);

            // Iterujemy po wszystkich sąsiadach wierzchołka
            for &edge in self.adj[vertex].iter(){
                if !visited[edge]{
                    visited[edge] = true;
                    stack.push_back(edge);
                }
            }
        }
    }
}


fn main() {
    // Tworzymy graf
    let mut graph = Graph::new(4);

    // Dodajemy krawędzie
    graph.addEdge(0, 1);
    graph.addEdge(0, 2);
    graph.addEdge(1, 2);
    graph.addEdge(2, 0);
    graph.addEdge(2, 3);
    graph.addEdge(3, 3);

    // Wykonujemy algorytm BFS dla wierzchołka 2
    println!("BFS:");
    graph.BFS(2);


    // Wykonujemy algorytm DFS dla wierzchołka 2
    println!("DFS:");
    graph.DFS(2);
}