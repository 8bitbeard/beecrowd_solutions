use std::io;

struct Graph {
    verticies: usize,
    graph: Vec<Vec<usize>>
}

impl Graph {
    fn min_distance(self, dist: &Vec<usize>, sptSet: Vec<bool>) -> usize {
        let mut min = usize::MAX;
        for u in 0..self.verticies {
            if dist[u] < min && sptSet[u] == false {
                min = dist[u];
                return u
            }
        }

    }
}


fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read n");
        let n: usize = match n.trim().parse() {
            Ok(x) => x,
            _ => break,
        };

        let v = (2 * n) + 2;
        let mut graph = Graph { verticies: 9, graph: vec![vec![0; v]; v] };

        let mut e = String::new();
        io::stdin().read_line(&mut e).expect("Failed to read e");
        let e: Vec<usize> = e
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        graph.graph[0][1] += e[0];
        graph.graph[0][v - 1] += e[1];

        let mut a1 = String::new();
        io::stdin().read_line(&mut a1).expect("Failed to read a1");
        let a1: Vec<usize> = a1
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for i in 0..n {
            graph.graph[i][i+1] += a1[i];
        }

        let mut a2 = String::new();
        io::stdin().read_line(&mut a2).expect("Failed to read a2");
        let a2: Vec<usize> = a2
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for i in 0..n {
            graph.graph[(v - i) % v][v-i-1] += a2[i];
        }

        let mut t1 = String::new();
        io::stdin().read_line(&mut t1).expect("Failed to read t1");
        let t1: Vec<usize> = t1
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for i in 1..=(n-1) {
            graph.graph[i][v-i-1] += t1[i - 1] + a2[i]
        }

        let mut t2 = String::new();
        io::stdin().read_line(&mut t2).expect("Failed to read t2");
        let t2: Vec<usize> = t2
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for i in 1..=(n-1) {
            graph.graph[v-i][i+1] += t2[i - 1] + a1[i]
        }

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read x");
        let x: Vec<usize> = x
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        graph.graph[n][n+1] += x[0];
        graph.graph[n+2][n+1] += x[1];

        for i in 0..v {
            println!("{:?}", graph.graph[i]);
        }
    }
}

fn djikstra(src) {
    let dist = usize::MAX * ;
}
